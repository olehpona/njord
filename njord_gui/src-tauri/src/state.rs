use crate::utils::ping_and_reconnect;
use njord_backend::controller::{PlugConfig, PlugHandler, PlugState};
use njord_backend::device::{Device, DeviceConfig, DeviceState, SerialInfo};
use njord_backend::sensors::{Sensor, SensorFactory, SensorId, SensorType, SensorsProvidersStates};
use njord_backend::sensors_providers::lhm_sensor::LhmState;
use njord_backend::sensors_providers::nvml_sensor::NvmlState;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager, Wry};
use tauri::async_runtime::{JoinHandle, Mutex};
use tauri::menu::{Menu, MenuBuilder, MenuItem, Submenu, SubmenuBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconId};
use tokio::time::sleep;

#[derive(Serialize, Clone)]
pub enum CoreMessageKind {
    Info,
    Warning,
    Error,
}

#[derive(Serialize, Clone)]
pub struct CoreMessage {
    pub kind: CoreMessageKind,
    pub message: String,
}


struct HandlerWorker {
    pub worker_join: JoinHandle<()>,
    pub worker_stop_signal: Arc<Mutex<bool>>,
}

pub struct AppState {
    app: AppHandle,
    pub devices: HashMap<String, Arc<Mutex<Device>>>,
    pub plug_handlers: HashMap<String, Arc<Mutex<Vec<Option<PlugHandler>>>>>,
    pub handler_workers: HashMap<String, HandlerWorker>,
    pub sensors_providers_states: SensorsProvidersStates,
    pub sensors: HashMap<SensorType, HashMap<String, Arc<dyn Sensor>>>,
    pub core_messages: Vec<CoreMessage>,
}

impl AppState {
    pub fn new(app: AppHandle) {
        let mut core_messages: Vec<CoreMessage> = Vec::new();

        let sensors_providers_states = SensorsProvidersStates {
            lhm_state: if let Ok(lhm_state) = LhmState::new() {
                Some(lhm_state)
            } else {
                core_messages.push(CoreMessage {
                    kind: CoreMessageKind::Error,
                    message: "Failed to initialize Lhm".to_string(),
                });
                None
            },
            nvml_state: if let Ok(nvml_state) = NvmlState::new() {
                Some(Arc::new(nvml_state))
            } else {
                core_messages.push(CoreMessage {
                    kind: CoreMessageKind::Error,
                    message: "Failed to initialize NVML".to_string(),
                });
                None
            },
        };
        let sensors = SensorFactory::get_all_sensors(&sensors_providers_states);
        let state_self = Self {
            app: app.clone(),
            devices: HashMap::new(),
            plug_handlers: HashMap::new(),
            handler_workers: HashMap::new(),
            sensors_providers_states,
            sensors,
            core_messages,
        };
        app.manage(Mutex::new(state_self));
        let app_handle = app.clone();

        tauri::async_runtime::spawn(tray_menu_loop(app_handle));
    }

    pub async fn add_device(
        &mut self,
        serial_info: SerialInfo,
        device_config_option: Option<DeviceConfig>,
    ) -> Result<(), String> {
        let id = serial_info.com_port.clone();
        if self.devices.contains_key(&id) {
            self.remove_device(id.clone()).await?;
        }

        let mut device = Device::new(serial_info);

        if let Some(device_config) = device_config_option {
            device.set_device_config(&device_config).await?;
            ping_and_reconnect(&mut device).await?;
        }

        device.fetch_data().await?;

        let mut plug_handlers_vec = Vec::new();
        plug_handlers_vec.resize(device.device_config.ports.len(), None);

        self.devices
            .insert(id.to_string(), Arc::new(Mutex::new(device)));

        self.plug_handlers.insert(id.to_string(), Arc::new(Mutex::new(plug_handlers_vec)));
        self.create_handler_worker(&id.to_string())?;

        Ok(())
    }

    fn create_handler_worker(&mut self, id: &String) -> Result<(), String>{
        let stop_signal = Arc::new(Mutex::new(false));
        let plug_handler_vec = self.plug_handlers.get(id).ok_or("No such device".to_string())?;

        let handler_stop_signal = stop_signal.clone();
        let plug_handlers = plug_handler_vec.clone();

        let join_handler = tauri::async_runtime::spawn(async move{
            let mut sleep_time = 0;
            let mut last_update = Instant::now();

            'handler: loop {
                if *handler_stop_signal.lock().await {
                    println!("Good bye cap");
                    break 'handler;
                }

                if last_update.elapsed().as_millis() as u64 >= sleep_time {
                    last_update = Instant::now();
                    let mut plug_handler_lock = plug_handlers.lock().await;
                    if let Some(first_handler_option) = plug_handler_lock.first() {
                        if let Some(handler_option) = first_handler_option {
                            sleep_time = handler_option.plug_externals.update_time;
                        }
                    }
                    for handler_option in plug_handler_lock.iter_mut() {
                        if let Some(handler) = handler_option {
                            if let Err(data) = handler.calculate_speed().await {
                                eprintln!("{}", data);
                            }
                        }
                    }
                }
            }
        });

        self.handler_workers
            .insert(id.to_string(), HandlerWorker {
                worker_join: join_handler,
                worker_stop_signal: stop_signal
            });

        Ok(())
    }

    async fn stop_worker(&mut self, id: &String){
        let handle_workers_option = self.handler_workers.get_mut(id);

        if let Some(handle_workers) = handle_workers_option{
            let mut signal_lock = handle_workers.worker_stop_signal.lock().await;
            *signal_lock = true;
        }
    }

    async fn clean_and_resize_plug_handlers(&mut self, id: &String, new_size: usize) -> Result<(), String>{
        let plug_handlers_arc = self.plug_handlers.get(id).ok_or("No such device")?;

        let mut plug_handlers = plug_handlers_arc.lock().await;
        plug_handlers.clear();
        plug_handlers.resize(new_size, None);

        Ok(())
    }

    pub async fn remove_device(&mut self, id: String) -> Result<(), String>{

        self.clean_and_resize_plug_handlers(&id, 0).await?; // dropping all plug_handlers
        self.plug_handlers.remove(&id);
        self.devices.remove(&id);

        self.stop_worker(&id).await;
        self.handler_workers.remove(&id);

        Ok(())
    }

    pub async fn update_device_config(&mut self, id: String, device_config: DeviceConfig) -> Result<(), String>{
        self.clean_and_resize_plug_handlers(&id, device_config.ports.len()).await?;

        let device_option = self.devices.get(&id);

        if let Some(device_ark) = device_option{
            let mut device = device_ark.lock().await;
            device.set_device_config(&device_config).await?;

            ping_and_reconnect(&mut *device).await?;

            device.fetch_data().await?;
        }

        Ok(())
    }

    pub async fn set_plug_handler(&mut self, device_id: String, plug_index: u8, sensor_id: SensorId, plug_config: PlugConfig) -> Result<(), String>{
        let device = self.devices.get(&device_id).ok_or("No such device".to_string())?;
        let sensor = self.sensors
            .get(&sensor_id.sensor_type)
            .ok_or("No such sensor provider".to_string())?
            .get(&sensor_id.identifier)
            .ok_or("No such sensor".to_string())?;
        let mut plug_handlers = self.plug_handlers.get(&device_id).ok_or("No such device".to_string())?.lock().await;

        let plug_handler_option = plug_handlers.get_mut(plug_index as usize).ok_or("No such plug".to_string())?;

        if let Some(plug_handler) = plug_handler_option{
            plug_handler.set_config(plug_config);
            plug_handler.set_sensor(sensor.clone());
        } else {
            *plug_handler_option = Some(PlugHandler::new(plug_index, device.clone(), sensor.clone(), plug_config).await?)
        }

        Ok(())
    }
}

async fn tray_menu_loop(app_handle: AppHandle<Wry>) {
    let tray_menu = MenuBuilder::new(&app_handle)
        .build()
        .expect("Failed to build initial tray menu");

    let _tray = TrayIconBuilder::new()
        .menu(&tray_menu)
        .icon(app_handle.default_window_icon().unwrap().clone())
        .show_menu_on_left_click(true)
        .build(&app_handle)
        .expect("Failed to create tray icon");

    loop {
        let state: tauri::State<Mutex<AppState>> = app_handle.state();

        let state_guard = state.lock().await;

        let (device_states, plug_states) = gather_current_states(&state_guard).await;

        let _ = app_handle.emit("device_state_update", &device_states);
        let _ = app_handle.emit("plugs_states_update", &plug_states);

        sync_tray_menu(&app_handle, &tray_menu, &device_states, &plug_states);

        drop(state_guard);
        sleep(Duration::from_millis(500)).await;
    }
}

async fn gather_current_states(state: &AppState) -> (HashMap<String, DeviceState>, HashMap<String, Vec<Option<PlugState>>>) {
    let mut device_states = HashMap::new();
    for (id, device) in &state.devices {
        let device_lock = device.lock().await;
        device_states.insert(id.clone(), device_lock.device_state.clone());
    }

    let mut plug_states = HashMap::new();
    for (id, plug_handlers_arc) in &state.plug_handlers {
        let plug_handlers_lock = plug_handlers_arc.lock().await;
        let states = plug_handlers_lock
            .iter()
            .map(|handler| handler.as_ref().map(|h| h.get_state()))
            .collect();
        plug_states.insert(id.clone(), states);
    }

    (device_states, plug_states)
}

fn sync_tray_menu(
    app_handle: &AppHandle<Wry>,
    tray_menu: &Menu<Wry>,
    device_states: &HashMap<String, DeviceState>,
    plug_states: &HashMap<String, Vec<Option<PlugState>>>
) {
    let mut current_device_ids = HashSet::new();

    for (id, device_state) in device_states {
        current_device_ids.insert(id.clone());
        let submenu_id = format!("device-{}", id);

        if let Some(submenu) = tray_menu.get(&submenu_id).as_ref().and_then(|i| i.as_submenu()) {
            update_device_submenu(submenu, id, device_state, plug_states.get(id));
        } else {
            if let Ok(new_submenu) = build_device_submenu(app_handle, id, device_state, plug_states.get(id)) {
                let _ = tray_menu.append(&new_submenu);
            }
        }
    }

    let menu_items = tray_menu.items().unwrap_or_default();
    let submenus_to_remove: Vec<_> = menu_items.iter().filter_map(|item| {
        if let Some(submenu) = item.as_submenu() {
            let submenu_id = submenu.id().0.clone();
            if submenu_id.starts_with("device-") && !current_device_ids.contains(&submenu_id[7..]) {
                return Some(item.clone());
            }
        }
        None
    }).collect();

    for submenu in submenus_to_remove {
        let _ = tray_menu.remove(&submenu);
    }
}

fn update_device_submenu(
    submenu: &Submenu<Wry>,
    id: &str,
    device_state: &DeviceState,
    plugs: Option<&Vec<Option<PlugState>>>
) {
    let state_item_id = format!("device-state-{}", id);
    if let Some(item) = submenu.get(&state_item_id).as_ref().and_then(|i| i.as_menuitem()) {
        let text = format_device_state_text(device_state);
        let _ = item.set_text(text);
    }

    if let Some(plug_states) = plugs {
        for (i, plug_state) in plug_states.iter().enumerate() {
            let plug_item_id = format!("plug-{}-{}", id, i);
            if let Some(item) = submenu.get(&plug_item_id).as_ref().and_then(|i| i.as_menuitem()) {
                let text = format_plug_state_text(i, plug_state);
                let _ = item.set_text(text);
            }
        }
    }
}

fn build_device_submenu(
    app_handle: &AppHandle<Wry>,
    id: &str,
    device_state: &DeviceState,
    plugs: Option<&Vec<Option<PlugState>>>
) -> Result<Submenu<Wry>, tauri::Error> {
    let submenu_id = format!("device-{}", id);
    let state_item_id = format!("device-state-{}", id);
    let device_state_text = format_device_state_text(device_state);

    let mut builder = SubmenuBuilder::with_id(app_handle, submenu_id, id);

    // Додаємо елемент зі статусом пристрою та розділювач
    let state_item = MenuItem::with_id(app_handle, state_item_id, device_state_text, true, None::<&str>)?;
    builder = builder.item(&state_item).separator();

    if let Some(plug_states) = plugs {
        for (i, plug_state) in plug_states.iter().enumerate() {
            let plug_item_id = format!("plug-{}-{}", id, i);
            let plug_state_text = format_plug_state_text(i, plug_state);
            let plug_item = MenuItem::with_id(app_handle, plug_item_id, plug_state_text, true, None::<&str>)?;
            builder = builder.item(&plug_item);
        }
    }

    builder.build()
}

fn format_device_state_text(state: &DeviceState) -> String {
    match state {
        DeviceState::Ok => "Status: OK".to_string(),
        DeviceState::Error(e) => format!("Status: Error ({})", e),
    }
}

fn format_plug_state_text(index: usize, state: &Option<PlugState>) -> String {
    match state {
        Some(s) => format!("Plug {}: {}°C / {}%", index + 1, s.last_temp, s.plug_value),
        None => format!("Plug {}: Not configured", index + 1),
    }
}
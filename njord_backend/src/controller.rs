use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::device::{Device, PortValue};
use crate::sensors::Sensor;

#[derive(Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct CurvePoint {
    pub temp: f32,
    pub value: PortValue,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DeadAreaVariant {
    Min,
    Max,
    Center,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeadArea {
    pub min_value: PortValue,
    pub max_value: PortValue,
    pub variant: DeadAreaVariant,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct CoolHolderData {
    pub holding_time: Duration,
    pub on_delta: u8, // Δt for enabling cooling hold
    pub off_delta: u8, // Δt for disabling cooling hold
    #[serde(skip_serializing, skip_deserializing)]
    pub start_time: Option<Instant>,
    #[serde(skip_serializing)]
    pub is_holding: bool,
}

#[derive(Clone, Serialize)]
pub struct PlugState {
    pub plug_value: PortValue,
    pub last_temp: f32,
}

#[derive(Clone)]
pub struct PlugExternals {
    pub plug_index: u8,
    pub device: Arc<Mutex<Device>>,
    pub update_time: u64,
    pub sensor: Arc<dyn Sensor>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PlugConfig {
    pub curve: Vec<CurvePoint>,
    pub dead_areas: Vec<DeadArea>,
    pub cool_holder: Option<CoolHolderData>,
}

#[derive(Clone)]
pub struct PlugHandler {
    plug_state: PlugState,
    pub plug_externals: PlugExternals,
    pub plug_config: PlugConfig
}

impl PlugHandler {
    pub async fn new(
    plug_index: u8,
    device: Arc<Mutex<Device>>,
    sensor: Arc<dyn Sensor>,
    plug_config: PlugConfig
) -> Result<Self, String> {
    let plug_value;
    let update_time;
    {
        let mut device_lock = device.lock().await;
        plug_value = device_lock
            .get_plugs_values()
            .await?
            .get(plug_index as usize)
            .ok_or("Plug index out of range")?
            .clone();
        update_time = device_lock.device_config.update_time.clone();
    }

    let plug_state = PlugState {
        plug_value,
        last_temp: 0f32,
    };

    let plug_externals = PlugExternals {
        plug_index,
        device: device.clone(),
        update_time,
        sensor,
    };

    Ok(Self {
        plug_state,
        plug_externals,
        plug_config
    })
    
}
    pub fn get_state(&self) -> PlugState { self.plug_state.clone() }
    pub fn set_config(&mut self, plug_config: PlugConfig){
        self.plug_config = plug_config;
    }


    pub fn set_sensor(&mut self, sensor: Arc<dyn Sensor>){
        self.plug_externals.sensor = sensor;
    }

    fn calculate_curve(curve: &Vec<CurvePoint>, temp: f32) -> PortValue {
        if curve.len() == 0 {
            return 0;
        }
        
        if temp <= curve[0].temp {
            return curve[0].value;
        }
        if temp >= curve[curve.len() - 1].temp {
            return curve[curve.len() - 1].value;
        }

        for i in 0..curve.len() - 1 {
            if temp >= curve[i].temp && temp <= curve[i + 1].temp {
                let t = (temp - curve[i].temp) / (curve[i + 1].temp - curve[i].temp);
                return curve[i].value + t as PortValue * (curve[i + 1].value - curve[i].value);
            }
        }

        curve[curve.len() - 1].value
    }
    pub async fn calculate_speed(&mut self) -> Result<(), String> {
        let (plug_externals, plug_state) = {

            let current_temp = self.plug_externals.sensor.get_temperature()?;
            let mut calculated = 0;
            let last_temp = self.plug_state.last_temp;

            if let Some(ref mut cool_holder) = self.plug_config.cool_holder{

                if cool_holder.is_holding {
                    if let Some(start_time) = cool_holder.start_time {
                        if start_time.elapsed() > cool_holder.holding_time {
                            cool_holder.is_holding = false;
                            return Ok(());
                        }
                        if (current_temp - cool_holder.off_delta as f32) > last_temp {
                            cool_holder.is_holding = false;
                            return Ok(());
                        }
                    } else {
                        cool_holder.start_time = Some(Instant::now());
                    }
                    calculated = self.plug_state.plug_value;
                } else if (current_temp + cool_holder.on_delta as f32) < last_temp {
                    cool_holder.is_holding = true;
                    cool_holder.start_time = Some(Instant::now());
                    return Ok(());
                } else {
                    calculated = Self::calculate_curve(&self.plug_config.curve, current_temp);
                }
            } else {
                calculated = Self::calculate_curve(&self.plug_config.curve, current_temp);
            }


            for dead_area in &self.plug_config.dead_areas {
                if calculated > dead_area.min_value && calculated < dead_area.max_value {
                    calculated = match dead_area.variant {
                        DeadAreaVariant::Min => dead_area.min_value,
                        DeadAreaVariant::Max => dead_area.max_value,
                        DeadAreaVariant::Center => {
                            let delta = dead_area.max_value - dead_area.min_value;
                            if calculated - dead_area.min_value > delta / 2 {
                                dead_area.max_value
                            } else {
                                dead_area.min_value
                            }
                        }
                    };
                }
            }

            self.plug_state.plug_value = calculated;
            self.plug_state.last_temp = current_temp;

            (
                self.plug_externals.clone(),
                self.plug_state.clone(),
            )
        };

        {
            let mut device_lock = plug_externals.device.lock().await;
            device_lock.test_connection(Duration::from_millis(100), Duration::from_millis(10)).await;
            device_lock.set_plug_value(plug_externals.plug_index, plug_state.plug_value).await?;
        }


        Ok(())
    }

}

pub mod nvml_sensor;
pub mod sys_info_sensor;

#[cfg(target_os = "windows")]
pub mod lhm_sensor;
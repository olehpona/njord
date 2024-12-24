use crate::device::PortValue;

struct CurvePoint {
    pub temp: f32,
    pub value: PortValue,
}

enum DeadAreaVariant {
    Min,
    Max,
    Center
}

struct DeadArea {
    pub min_value: PortValue,
    pub max_value: PortValue,
    pub variant: DeadAreaVariant
}

struct CoolHandler {
    pub handle_time: u64,
    pub up_delta: u32, // −ΔT for enabling cooling hold
    pub off_delta: u32, // +ΔT for disabling cooling hold
    pub is_holding: bool
}

pub struct PlugConfig {
    plug_id: u8,
    curve: Vec<CurvePoint>,
    dead_areas: Vec<DeadArea>,
    cool_handler: CoolHandler
}
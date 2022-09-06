#[derive(Clone, Debug, PartialEq)]
pub enum ScopeMode {
    Stop,
    Run,
    Single,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TriggerMode {
    Auto,
    Rising,
    Falling,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AxisMode {
    Fit,
}

#[derive(Clone)]
pub struct TriggerSettings {
    pub mode: TriggerMode,
    pub level: f32,
}

impl Default for TriggerSettings {
    fn default() -> Self {
        Self {
            mode: TriggerMode::Auto,
            level: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct ScopeSettings {
    pub mode: ScopeMode,
    pub trigger: TriggerSettings,
    pub x_mode: AxisMode,
    pub y_mode: AxisMode,
}

impl Default for ScopeSettings {
    fn default() -> Self {
        Self {
            mode: ScopeMode::Run,
            trigger: TriggerSettings::default(),
            x_mode: AxisMode::Fit,
            y_mode: AxisMode::Fit,
        }
    }
}

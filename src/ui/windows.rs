pub struct Windows {
    pub settings: bool,
    pub scope: bool,
    pub messages: bool,
    pub about: bool,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            settings: false,
            scope: false,
            messages: false,
            about: false,
        }
    }
}

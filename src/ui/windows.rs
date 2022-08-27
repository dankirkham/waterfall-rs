pub struct Windows {
    pub settings: bool,
    pub scope: bool,
}

impl Default for Windows {
    fn default() -> Self {
        Self {
            settings: false,
            scope: false,
        }
    }
}


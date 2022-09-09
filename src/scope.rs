use tokio::sync::mpsc;

use crate::configuration::{Configuration, ScopeMode, TriggerMode, TriggerSettings};
use crate::types::SampleType;

fn trigger_position(data: &[SampleType], settings: &TriggerSettings) -> Option<usize> {
    let TriggerSettings { mode, level } = settings;
    let lower = |d| d < level;
    let higher = |d| d > level;
    match mode {
        TriggerMode::Rising => {
            let below = data.iter().position(lower)?;
            data[below..].iter().position(higher)
        }
        TriggerMode::Falling => {
            let above = data.iter().position(higher)?;
            data[above..].iter().position(lower)
        }
        TriggerMode::Auto => Some(0),
    }
}

pub struct Scope {
    plot_rx: mpsc::Receiver<Vec<SampleType>>,
    plot_data: Vec<SampleType>,
}

impl Scope {
    pub fn new(plot_rx: mpsc::Receiver<Vec<SampleType>>) -> Self {
        Self {
            plot_rx,
            plot_data: Vec::new(),
        }
    }

    pub fn run(&mut self, config: &mut Configuration) {
        let mut new_plot_data: Option<Vec<SampleType>> = None;
        while let Ok(plot_data) = self.plot_rx.try_recv() {
            new_plot_data = Some(plot_data);
        }
        if new_plot_data.is_none() {
            return;
        }

        if config.scope.mode == ScopeMode::Stop {
            return;
        }

        let new_plot_data = new_plot_data.unwrap();

        if let Some(start_pos) = trigger_position(&new_plot_data, &config.scope.trigger) {
            self.plot_data = Vec::from(&new_plot_data[start_pos..]);

            if config.scope.mode == ScopeMode::Single {
                config.scope.mode = ScopeMode::Stop;
            }
        };
    }

    pub fn get_plot_data(&mut self) -> &[SampleType] {
        &self.plot_data
    }
}

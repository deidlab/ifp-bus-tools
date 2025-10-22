use crate::serial::SerialTerminal;
use std::sync::Mutex;

pub struct AppState {
    pub serial: Mutex<SerialTerminal>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            serial: Mutex::new(SerialTerminal::new()),
        }
    }
}

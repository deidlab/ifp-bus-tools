use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tauri::{Emitter, Window};

use serialport::SerialPort;

#[derive(Default)]
pub struct SerialTerminal {
    port: Arc<Mutex<Option<Box<dyn SerialPort + Send>>>>,
}

impl SerialTerminal {
    pub fn new() -> Self {
        Self {
            port: Arc::new(Mutex::new(None)),
        }
    }

    pub fn open(&self, port_name: &str, baudrate: u32, window: Window) -> io::Result<()> {
        let serial = serialport::new(port_name, baudrate).open()?;
        *self.port.lock().unwrap() = Some(serial);
        Ok(())
    }

    pub fn write(&self, data: &str) -> io::Result<()> {
        let mut guard = self.port.lock().unwrap();
        if let Some(ref mut port) = *guard {
            port.write_all(data.as_bytes())?;
            port.flush()?;
        }
        Ok(())
    }

    fn start_read_loop(&self, window: Window) {
        let port_clone = Arc::clone(&self.port);

        thread::spawn(move || loop {
            let mut buf: [u8; 1024] = [0; 1024];
            let n = {
                let mut guard = port_clone.lock().unwrap();
                if let Some(ref mut port) = *guard {
                    match port.read(&mut buf) {
                        Ok(n) => n,
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => 0,
                        Err(_) => break, // errore grave, esci dal loop
                    }
                } else {
                    break; // porta chiusa
                }
            };

            if n > 0 {
                let data = String::from_utf8_lossy(&buf[..n]).to_string();
                let _ = window.emit("serial-data", data);
            }

            thread::sleep(Duration::from_millis(10));
        });
    }

    /// Chiude la porta
    pub fn close(&self) {
        *self.port.lock().unwrap() = None;
    }

    /// Cambia impostazioni (riapre la porta)
    pub fn change_settings(&self, port_name: &str, baud: u32, window: Window) -> io::Result<()> {
        self.close();
        self.open(port_name, baud, window)
    }
}

pub fn check_available_ports() -> Vec<String> {
    let ports_info = serialport::available_ports().unwrap();
    let mut ports = Vec::new();
    for p in ports_info {
        ports.push(p.port_name)
    }
    ports
}

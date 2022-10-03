#![warn(clippy::pedantic, clippy::nursery)] //,
											// clippy::cargo,
											// clippy::unwrap_used,
											// clippy::expect_used)]

use super::*;
use std::time::Duration;

// const NO_PORTS: &str = "There are no available ports!!!";
pub fn open_serial_port<'a,F>(port_chooser: F) -> Result<Box<dyn serialport::SerialPort>, String>
	where F: Fn(Vec<serialport::SerialPortInfo>) -> &'a serialport::SerialPortInfo
{
	let ports = serialport::available_ports().expect("No ports found!");
	if ports.is_empty()
	{
		return Err(String::from("There are no available ports!!!"));
	}

        let port = if ports.len() > 1
        {
                port_chooser(ports)
        }
        else
        {
                &ports[0]
        };
	let mut open_port = serialport::new(format! ("/{}",&(port.port_name)),9600)
			.timeout(Duration::from_millis(10))
			.open()
			.expect("Failed to open port");
	open_port.set_timeout(Duration::from_secs(2)).unwrap();
	Ok(open_port)
}

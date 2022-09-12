#![warn(clippy::pedantic,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]

use super::*;

// const NO_PORTS: &str = "There are no available ports!!!";
pub fn start_communication()
{
	let ports = serialport::available_ports().expect("No ports found!");
	if ports.is_empty()
	{
		println!("There are no available ports!!!");
	}
	else
	{
		for p in &ports
		{
			println!("{}", p.port_name);
		}

		let mut open_port = serialport::new(format! ("/{}",&(ports[0].port_name)), 9600)
			.timeout(Duration::from_millis(10))
			.open()
			.expect("Failed to open port");
		// add code to check if there is more than one
		// port and if there is, run a function that chooses in
		// between them
		let mut serial_buf: Vec<u8> = vec![0; 64];
		open_port.set_timeout(Duration::from_secs(2)).unwrap();
		loop
		{
			// This functio shouldn't have this loop, intead
			// it should return the com port that's connected to
			// and we could consider returning a String buffer too
			print!("{}",match std::str::from_utf8(serial_buf.as_mut_slice())
				   {
					   Ok(strin) => strin,
					   Err(error) =>"",
				   })
		};
	}
}



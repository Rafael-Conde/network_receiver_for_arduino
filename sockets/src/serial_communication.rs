// serialport = "4.2.0"
// finish the git flow in the uPnP-to-server branch in 
// order to start a new branch which will track this implementation
pub mod serial_communication;

use super::*;

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
			//now thew only change needed it's to implement writing to the 
			//port instead of reading from it
			if let Ok(_num_of_bytes_read) = open_port.read(serial_buf.as_mut_slice())
			{
				print!("{}",match std::str::from_utf8(serial_buf.as_mut_slice())
					   {
						   Ok(strin) => strin,
						   Err(error) =>"",
					   })
			};
		}
	}
}


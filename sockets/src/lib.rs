// #![warn(clippy::all,
#![warn(clippy::pedantic,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]

pub use igd::PortMappingProtocol;
use std::net::{SocketAddrV4, TcpListener};
use std::time::Duration;

pub mod networking
{
	use std::net::IpAddr;

	use super::*;

	pub fn open_server(protocol: igd::PortMappingProtocol, port: u16)
	                   -> Result<TcpListener, String>
	{
		let local_addr = match local_ip_address::local_ip()
		{
			Err(e) => return Err(e.to_string()),
			Ok(local_addr) =>
			{
				if let IpAddr::V4(ipv4) = local_addr
				{
					ipv4
				}
				else
				{
					return Err(String::from("The local IP address isn't the IPv4."));
				}
			}
		};
		println!("The local ip is: {}", local_addr);

		let gateway = match igd::search_gateway(Default::default())
		{
			Err(e) => return Err(e.to_string()),
			Ok(gateway) => gateway,
		};

		//add logging for which port is
		//being used
		let local_addr = SocketAddrV4::new(local_addr, port); //add a way to get the
													  //local ip address

		if let Err(e) =
			gateway.add_port(
			                 protocol,
			                 port,
			                 local_addr,
			                 20,
			                 "Server to transmite to arduino the incoming data from google colab",
			)
		{
			return Err(e.to_string());
		};
		//include logging for successfull addewd port on the router
		match TcpListener::bind(local_addr)
		{
			Err(e) => Err(e.to_string()),
			Ok(TcpListener) => Ok(TcpListener), //log that the server started0
		}
	}
	//add a logging that the server started
}

pub mod serial_communication
{
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
}

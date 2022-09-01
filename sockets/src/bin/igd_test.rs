use std::{net::{Ipv4Addr, SocketAddrV4, TcpListener, SocketAddr, IpAddr}, io::Read};

extern crate igd;

fn main()
{
	match igd::search_gateway(Default::default())
	{
		Err(ref err) => println!("Error while Searching gateway: {}", err),
		Ok(gateway) =>
		{
			let local_addr = match std::env::args().nth(1)
			{
				Some(local_addr) => local_addr,
				None => panic!("Expected IP address (cargo run --example add_port <your IP here>)"),
			};
			let local_addr = local_addr.parse::<Ipv4Addr>().unwrap();
			let local_addr = SocketAddrV4::new(local_addr, 50000u16);

			match gateway.add_port(
			                       igd::PortMappingProtocol::TCP,
			                       50000,
			                       local_addr,
			                       20,
			                       "add_port example",
			)
			{
				Err(ref err) =>
				{
					println!("There was an error! {}", err);
				}
				Ok(()) =>
				{
					println!("It worked");
					// let loopback = "192.168.1.102".trim() //"::".trim()
					//                               .parse::<IpAddr>()
					//                               .expect("Error while parsing the address");
					// let port: u16 = 50000;
					// let socket = SocketAddr::new(local_addr, port);
                    // let socket = SocketAddr::new("192.168.15.10".parse::<IpAddr>().unwrap(), port);
					let listener = TcpListener::bind(local_addr).unwrap();
					println!(
					         "Listening on {}, access this port to end the program",
					         local_addr
					);

					let (mut tcp_stream, addr) = listener.accept().unwrap(); //block  until requested
					println!("Connection received! {:?} is sending data.", addr);
					let mut input = String::new();
					let _ = tcp_stream.read_to_string(&mut input).unwrap();
					println!("{:?} says {}", addr, input);
				}
			}
		}
	}
}

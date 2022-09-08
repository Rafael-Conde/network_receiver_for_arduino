// #![warn(clippy::all,
//         clippy::pedantic,
//         clippy::nursery,
//         clippy::cargo,
//         clippy::unwrap_used)]
//
use std::{
	io::Read,
	net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

extern crate igd;

fn main()
{
	// testando a edição de texto com Astro NeoVim
	// This is incredible!!!
	match igd::search_gateway(igd::SearchOptions::default())
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

			if let Err(ref err) = gateway.add_port(
			                                       igd::PortMappingProtocol::TCP,
			                                       50000,
			                                       local_addr,
			                                       20,
			                                       "add_port example",
			)
			{
				println!("There was an error! {}", err);
			}
			else
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

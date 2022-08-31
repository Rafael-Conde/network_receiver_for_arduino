use std::{net::{TcpListener, SocketAddr, IpAddr}, io::{Read,Error}};

fn main()
{
	let loopback = "192.168.1.102".trim()//"::".trim()
	                   .parse::<IpAddr>()
	                   .expect("Error while parsing the address");
	let port: u16 = 50000;
	let socket = SocketAddr::new(loopback, port);
	let listener = TcpListener::bind(socket).unwrap();
	println!(
	         "Listening on {}, access this port to end the program",
	         socket
	);

	let (mut tcp_stream, addr) = listener.accept().unwrap(); //block  until requested
	println!("Connection received! {:?} is sending data.", addr);
	let mut input = String::new();
	let _ = tcp_stream.read_to_string(&mut input).unwrap();
	println!("{:?} says {}", addr, input);
}

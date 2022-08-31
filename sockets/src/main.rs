use std::io::{stdin, Error, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

fn main() -> Result<(), Error>
{
	println!("Please insert your IP address:");
	let mut input_string = String::new();
	stdin().read_line(&mut input_string)
	       .expect("Failed to read line");

	let loopback = input_string.trim()
	                           .parse::<IpAddr>()
	                           .expect("Error while parsing the address");
	let port: u16 = 50000;
	let socket = SocketAddr::new(loopback, port);

	let listener = TcpListener::bind(socket)?;
	println!(
	         "Listening on {}, access this port to end the program",
	         socket
	);

	let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
	println!("Connection received! {:?} is sending data.", addr);
	let mut input = String::new();
	let _ = tcp_stream.read_to_string(&mut input)?;
	println!("{:?} says {}", addr, input);

	Ok(())
}

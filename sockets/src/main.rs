use pcp::{self, Alert, InboundMap, ProtocolNumber, Request, RequestType};
use std::io::{stdin, Error, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};

fn main() -> Result<(), Error>
{
	// let loopback = IpAddr::V4(Ipv4Addr::new(192,168,15,10));

	println!("Please insert your IPv4 address:");
	let mut input_string = String::new();
	stdin().read_line(&mut input_string)
	       .expect("Failed to read line");

	println!("IpAddr: {}", input_string);
	let server = input_string.trim()
	                         .parse::<Ipv4Addr>()
	                         .expect("Error while parsing the address");
	let default_gateway = Ipv4Addr::new(192, 168, 1, 1);

	let pcp = pcp::Client::<Ipv4Addr>::start(
	                                         server,          // My address
	                                         default_gateway, // PCP server address
	).unwrap_or_else(|err| {
		          println!("Error starting pcp - {}", err);
		          panic!();
	          });

	// Define a mapping that maps any incoming request on TCP port 50000 to my address
	let map = InboundMap::new(50000, 20).protocol(ProtocolNumber::Tcp);

	let handle = pcp.request(map, RequestType::KeepAlive)
	                .unwrap_or_else(|err| {
		                println!("Error on request - {}", err);
		                panic!();
	                });

	let loopback = "192.168.1.102".trim()
	                   .parse::<IpAddr>()
	                   .expect("Error while parsing the address");
	let port: u16 = 50000;
	let socket = SocketAddr::new(loopback, port);

    let mut res_alert = handle.wait_alert();

	while let Ok(alert) = res_alert
	{
		match alert
		{
			Alert::StateChange => println!("State: {:?}", handle.state()),
			Alert::Assigned(ip, port, lifetime) =>
			{
				println!(
				         "Assigned ip: {:?}\nAssigned port: {}\nAssigned lifetime: {}",
				         ip, port, lifetime
				);

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
                handle.revoke();
			}
		}
        res_alert = handle.wait_alert();
	}
    if let Err(e) = res_alert
    {
        println!("Error on alert - {}",e);
    }

	Ok(())
}

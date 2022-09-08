// #![warn(clippy::all,
//         clippy::pedantic,
//         clippy::nursery,
//         clippy::cargo,
//         clippy::unwrap_used,
//         clippy::expect_used)]
//

use std::io::Read;
use std::process;

use sockets::{networking, PortMappingProtocol};

fn main()
{
	let listener = match networking::open_server(PortMappingProtocol::TCP, 50000u16)
	{
		Err(t) =>
		{
			println!("{}", t);
			panic!();
		}
		Ok(tcp) => tcp,

	};
	let (mut tcp_stream, _) = match listener.accept()
	{
		Err(e) => panic!("{e}"),
		Ok((tcp_stream, addr)) => (tcp_stream, addr),
	};
	let mut input_buff = String::new();
	if let Err(e) = tcp_stream.read_to_string(&mut input_buff)
	{
		println!("{e}");
		process::exit(1);
	};
	println!("{}", input_buff);
}

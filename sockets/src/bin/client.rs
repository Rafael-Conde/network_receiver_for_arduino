use std::io::{Error, stdin, Write};
use std::net::{IpAddr, SocketAddr,TcpStream};


fn main() -> Result<(), Error>
{
    println!("Please insert the IP address:");
	let mut input_string = String::new();
	stdin().read_line(&mut input_string)
	       .expect("Failed to read line");

    println!("IpAddr: {}",input_string);
    let server = input_string.trim().parse::<IpAddr>().expect("Error while parsing the address");

    println!("Please insert the port number:");
	let mut input_port = String::new();
	stdin().read_line(&mut input_port)
	       .expect("Failed to read line");

    let socket = SocketAddr::new(server,input_port.trim().parse::<u16>().unwrap());
    let mut sender = TcpStream::connect(socket).unwrap();

    println!("Please write your messge:");
	let mut message = String::new();
    stdin().read_line(&mut message)
	       .expect("Failed to read line");
    match sender.write(message.as_bytes()) {
        Ok(_) => println!("Message sent successfully"),
        Err(e) => println!("Message wasn't send successfully due to the following error - {}",e)
    }
	Ok(())
}

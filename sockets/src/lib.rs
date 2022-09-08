// #![warn(clippy::all,
#![warn(clippy::pedantic,
        clippy::nursery,
        // clippy::cargo,
        clippy::unwrap_used,
        clippy::expect_used)]

use std::net::{SocketAddrV4, TcpListener};
pub use igd::PortMappingProtocol;

/// todo docs for the network_protocol enum
///

pub mod networking
{
	use std::net::IpAddr;

        use super::*;


	pub fn open_server(protocol: igd::PortMappingProtocol,
	                   port: u16)
	        -> Result<TcpListener, String>
	        {
		        let gateway = match igd::search_gateway(igd::SearchOptions::default())
		        {
		                Err(e) => return Err(e.to_string()),
		                Ok(gateway) => gateway

		        };
		        let local_addr = match local_ip_address::local_ip()
		        {
		                Err(e) => return Err(e.to_string()),
		                Ok(local_addr) => 
		                        if let IpAddr::V4(ipv4) = local_addr
		                        {
                                                ipv4
		                        }
		                        else
		                        {
                                                return Err(String::from("The local IP address isn't the IPv4."));
		                        }
		        };

		        //add logging for which port is
		        //being used
	                let local_addr = SocketAddrV4::new(local_addr, port); //add a way to get the
							                      //local ip address

	                if let Err(e) = gateway.add_port(
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
		                Ok(TcpListener) => //log that the server started
		                        Ok(TcpListener)

	                }
	        }
	//add a logging that the server started
}


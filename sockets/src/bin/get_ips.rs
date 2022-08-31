extern crate pnet_datalink;

//use pnet_datalink;

fn main() {
    for interface in pnet_datalink::interfaces() {
        println!("{}", interface);
    }
}
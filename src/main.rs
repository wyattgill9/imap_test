use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::Packet;
use std::net::IpAddr;

fn get_interface(name: &str) -> Option<NetworkInterface> {
    
    datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == name)
}

fn create_ethernet_frame(source_mac: [u8; 6], dest_mac: [u8; 6], payload: &[u8]) -> Vec<u8> {
    
    let mut buffer = vec![0u8; 14 + payload.len()];
    
    let mut ethernet_packet = MutableEthernetPacket::new(&mut buffer)
        .expect("Error creating Ethernet packet");
    
    ethernet_packet.set_source(source_mac);
    ethernet_packet.set_destination(dest_mac);
    
    ethernet_packet.set_ethertype(EtherTypes::Ipv4);
    
    ethernet_packet.set_payload(payload);
    
    buffer
}

fn main() {
    let interface_name = "eth0";
    
    if let Some(interface) = get_interface(interface_name) {
        println!("Found interface: {}", interface.name);
        

        let source_mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let dest_mac = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        
        let payload = b"Hello, Network!";
        
        let frame = create_ethernet_frame(source_mac, dest_mac, payload);
        
        // Create a channel to send packets
        let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
            Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unhandled channel type"),
            Err(e) => panic!("Error creating channel: {}", e),
        };
        
        match tx.send(&frame) {
            Some(_) => println!("Packet sent successfully"),
            None => println!("Error sending packet"),
        }
        
        if let Some(packet) = EthernetPacket::new(&frame) {
            println!("Source MAC: {:?}", packet.get_source());
            println!("Destination MAC: {:?}", packet.get_destination());
            println!("EtherType: {:?}", packet.get_ethertype());
            println!("Payload: {:?}", String::from_utf8_lossy(packet.payload()));
        }
    } else {
        println!("Interface {} not found", interface_name);
    }
}
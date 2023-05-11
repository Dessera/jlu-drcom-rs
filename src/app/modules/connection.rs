use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet::packet::udp::{MutableUdpPacket, UdpPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;

/**
 * DrcomConnection， which only satisfies the D version.
 */
pub struct DrcomConnection {
  pub username: String,
  pub password: String,
  pub mac_addr: MacAddr,
  pub serv_addr: String,
  pub serv_port: u16,

  tx: Box<dyn datalink::DataLinkSender>,
  rx: Box<dyn datalink::DataLinkReceiver>,
}

impl DrcomConnection {
  pub fn new(username: &str, password: &str) -> Self {
    //  look for available network interfaces
    // TODO: All Error handling
    // TODO: Enable user to choose interface
    let interfaces = datalink::interfaces();
    let interface = interfaces
      .iter()
      .find(|iface: &&NetworkInterface| {
        iface.is_up() && !iface.is_loopback() && !iface.ips.is_empty()
      })
      .expect("Failed to get interface");

    // get the MAC address of the interface
    let mac_addr = interface.mac.unwrap();

    // create UDP socket
    let (tx, rx) = match datalink::channel(&interface, Default::default()) {
      Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
      Ok(_) => panic!("Unhandled channel type"),
      Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    Self {
      username: String::from(username),
      password: String::from(password),
      mac_addr,
      serv_addr: String::from("10.100.61.3"),
      serv_port: 61440,
      tx,
      rx,
    }
  }
}

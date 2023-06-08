use crate::app::utils::{error::DrResult, interface::Ichallenge};

pub struct DrcomConnection<Challenger>
where
  Challenger: Ichallenge,
{
  pub socket: std::net::UdpSocket,
  pub challenger: Challenger,
}

impl<Challenger> DrcomConnection<Challenger>
where
  Challenger: Ichallenge + Default,
{
  pub fn new() -> DrResult<Self> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("10.100.61.3:61440")?;
    Ok(Self {
      socket,
      challenger: Challenger::default(),
    })
  }
}

use std::net::UdpSocket;

pub fn send_magic_packet(mac: &str, ip: &str) -> std::io::Result<()> {
    let mac_bytes: Vec<u8> = mac
        .split(|c| c == ':' || c == '-')
        .filter_map(|x| u8::from_str_radix(x, 16).ok())
        .collect();
    if mac_bytes.len() != 6 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid MAC address"));
    }

    let mut packet = vec![0xFF; 6];
    for _ in 0..16 {
        packet.extend_from_slice(&mac_bytes);
    }

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.send_to(&packet, format!("{}:9", ip))?;
    Ok(())
}

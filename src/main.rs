use std::sync::mpsc;
use tray_item::{IconSource, TrayItem};

enum Message {
    Quit,
    SendPacket,
}

fn main() {
    let mut tray = TrayItem::new(
        "Magic Packet sender",
        IconSource::Resource("exe-icon"),
    )
    .unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let send_packet_tx = tx.clone();
    tray.add_menu_item("Send Packet", move || {
        send_packet_tx.send(Message::SendPacket).unwrap();
    })
    .unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Ok(Message::SendPacket) => {
                println!("Send Packet");
            }
            _ => {}
        }
    }
}

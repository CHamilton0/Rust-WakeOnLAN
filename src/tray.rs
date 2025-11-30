use crate::Message;
use std::sync::mpsc::SyncSender;
use tray_item::{IconSource, TrayItem};

pub fn init_tray(tx: &SyncSender<Message>) -> TrayItem {
    let mut tray = TrayItem::new("Magic Packet sender", IconSource::Resource("exe-icon")).unwrap();

    let send_packet_tx = tx.clone();
    tray.add_menu_item("Send Packet", move || {
        send_packet_tx.send(Message::SendPacket).unwrap();
    }).unwrap();

    let config_tx = tx.clone();
    tray.add_menu_item("Config", move || {
        config_tx.send(Message::ShowConfig).unwrap();
    }).unwrap();

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    }).unwrap();

    tray
}

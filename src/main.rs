
#![cfg_attr(windows, windows_subsystem = "windows")]

mod config;
mod tray;
#[cfg(windows)]
mod win32_dialog;

use std::sync::mpsc;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Quit,
    SendPacket,
    ShowConfig,
}

fn main() {
    // Ensure only one instance runs at a time
    #[cfg(windows)]
    {
        use windows::Win32::Foundation::ERROR_ALREADY_EXISTS;
        use windows::Win32::System::Threading::CreateMutexW;
        use windows::core::PCWSTR;

        let mutex_name: Vec<u16> = "MagicPacketSingleton"
            .encode_utf16()
            .chain(Some(0))
            .collect();
        let mutex = unsafe { CreateMutexW(None, false, PCWSTR(mutex_name.as_ptr())) };
        let mutex = match mutex {
            Ok(h) => h,
            Err(_) => {
                // Failed to create mutex, assume already running or error
                return;
            }
        };
        if mutex.is_invalid()
            || unsafe { windows::Win32::Foundation::GetLastError() } == ERROR_ALREADY_EXISTS
        {
            // Already running
            return;
        }
    }

    let (tx, rx) = mpsc::sync_channel(1);
    let _tray = tray::init_tray(&tx);

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                break;
            }
            Ok(Message::SendPacket) => {
                println!("Send Packet");
            }
            Ok(Message::ShowConfig) => {
                #[cfg(windows)]
                {
                    win32_dialog::show_config_dialog();
                }
                #[cfg(not(windows))]
                {
                    println!("Config dialog is only available on Windows.");
                }
            }
            _ => {}
        }
    }
}

#![cfg_attr(windows, windows_subsystem = "windows")]

use std::sync::mpsc;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    mac: String,
    ip: String,
}

fn config_path() -> PathBuf {
    // Save config in the user's home directory as ".magic_packet_config.toml"
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".magic_packet_config.toml");
    path
}

fn load_config() -> AppConfig {
    let path = config_path();
    if let Ok(data) = fs::read_to_string(&path) {
        toml::from_str(&data).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

fn save_config(cfg: &AppConfig) {
    let path = config_path();
    if let Ok(toml) = toml::to_string(cfg) {
        let _ = fs::write(path, toml);
    }
}
use tray_item::{IconSource, TrayItem};
#[cfg(windows)]
mod win32_dialog {
    use std::ffi::OsStr;
    use std::mem::MaybeUninit;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Foundation::*;
    use windows::Win32::Graphics::Gdi::{DEFAULT_GUI_FONT, GetStockObject};
    use windows::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows::Win32::UI::WindowsAndMessaging::ES_AUTOHSCROLL;
    use windows::Win32::UI::WindowsAndMessaging::{
        BS_DEFPUSHBUTTON, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow,
        DispatchMessageW, GetDlgItemTextW, GetMessageW, HMENU, MB_OK, MSG, MessageBoxW,
        PostQuitMessage, RegisterClassW, SendMessageW, TranslateMessage, WINDOW_STYLE, WM_COMMAND,
        WM_CREATE, WM_DESTROY, WM_SETFONT, WNDCLASSW, WS_BORDER, WS_CHILD, WS_OVERLAPPEDWINDOW,
        WS_VISIBLE, GetDlgItem, SetWindowTextW
    };
    use windows::core::PCWSTR;

    const ID_MAC: i32 = 101;
    const ID_IP: i32 = 102;

    use super::{AppConfig, load_config, save_config};
    use std::cell::RefCell;
    thread_local! {
        static CONFIG: RefCell<AppConfig> = RefCell::new(load_config());
    }

    pub fn show_config_dialog() {
        unsafe {
            let h_instance = GetModuleHandleW(None).unwrap();
            let class_name = to_wide("ConfigDialog");
            let window_name = to_wide("Config");

            let wc = WNDCLASSW {
                lpfnWndProc: Some(wnd_proc),
                hInstance: h_instance.into(),
                lpszClassName: PCWSTR(class_name.as_ptr()),
                ..Default::default()
            };
            RegisterClassW(&wc);

            let hwnd = CreateWindowExW(
                Default::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR(window_name.as_ptr()),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                300,
                180,
                None,
                None,
                Some(h_instance.into()),
                None,
            );
            let hwnd = hwnd.unwrap();
            if hwnd.0.is_null() {
                return;
            }

            // Prefill fields with config values
            CONFIG.with(|cfg| {
                let cfg = cfg.borrow();
                let mac_wide = to_wide(&cfg.mac);
                let ip_wide = to_wide(&cfg.ip);
                if let Ok(mac_edit) = GetDlgItem(Some(hwnd), ID_MAC) {
                    let _ = SetWindowTextW(mac_edit, PCWSTR(mac_wide.as_ptr()));
                }
                if let Ok(ip_edit) = GetDlgItem(Some(hwnd), ID_IP) {
                    let _ = SetWindowTextW(ip_edit, PCWSTR(ip_wide.as_ptr()));
                }
            });

            let mut msg = MaybeUninit::<MSG>::uninit();
            while GetMessageW(msg.as_mut_ptr(), None, 0, 0).into() {
                TranslateMessage(msg.as_ptr());
                DispatchMessageW(msg.as_ptr());
            }
        }
    }

    extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match msg {
                WM_CREATE => {
                    let hfont = GetStockObject(DEFAULT_GUI_FONT);
                    // MAC label
                    let mac_label = CreateWindowExW(
                        Default::default(),
                        PCWSTR(to_wide("STATIC").as_ptr()),
                        PCWSTR(to_wide("MAC Address:").as_ptr()),
                        WS_CHILD | WS_VISIBLE,
                        10,
                        10,
                        90,
                        20,
                        Some(hwnd),
                        None,
                        None,
                        None,
                    );
                    SendMessageW(
                        mac_label.unwrap(),
                        WM_SETFONT,
                        Some(WPARAM(hfont.0 as usize)),
                        Some(LPARAM(1)),
                    );
                    // MAC input
                    let mac_edit = CreateWindowExW(
                        Default::default(),
                        PCWSTR(to_wide("EDIT").as_ptr()),
                        PCWSTR(to_wide("").as_ptr()),
                        WS_CHILD | WS_VISIBLE | WS_BORDER | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
                        110,
                        10,
                        160,
                        20,
                        Some(hwnd),
                        Some(HMENU(ID_MAC as usize as *mut _)),
                        None,
                        None,
                    );
                    SendMessageW(
                        mac_edit.unwrap(),
                        WM_SETFONT,
                        Some(WPARAM(hfont.0 as usize)),
                        Some(LPARAM(1)),
                    );
                    // IP label
                    let ip_label = CreateWindowExW(
                        Default::default(),
                        PCWSTR(to_wide("STATIC").as_ptr()),
                        PCWSTR(to_wide("IP Address:").as_ptr()),
                        WS_CHILD | WS_VISIBLE,
                        10,
                        40,
                        90,
                        20,
                        Some(hwnd),
                        None,
                        None,
                        None,
                    );
                    SendMessageW(
                        ip_label.unwrap(),
                        WM_SETFONT,
                        Some(WPARAM(hfont.0 as usize)),
                        Some(LPARAM(1)),
                    );
                    // IP input
                    let ip_edit = CreateWindowExW(
                        Default::default(),
                        PCWSTR(to_wide("EDIT").as_ptr()),
                        PCWSTR(to_wide("").as_ptr()),
                        WS_CHILD | WS_VISIBLE | WS_BORDER | WINDOW_STYLE(ES_AUTOHSCROLL as u32),
                        110,
                        40,
                        160,
                        20,
                        Some(hwnd),
                        Some(HMENU(ID_IP as usize as *mut _)),
                        None,
                        None,
                    );
                    SendMessageW(
                        ip_edit.unwrap(),
                        WM_SETFONT,
                        Some(WPARAM(hfont.0 as usize)),
                        Some(LPARAM(1)),
                    );
                    // OK button
                    let ok_btn = CreateWindowExW(
                        Default::default(),
                        PCWSTR(to_wide("BUTTON").as_ptr()),
                        PCWSTR(to_wide("OK").as_ptr()),
                        WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_DEFPUSHBUTTON as u32),
                        110,
                        80,
                        60,
                        25,
                        Some(hwnd),
                        Some(HMENU(1 as usize as *mut _)),
                        None,
                        None,
                    );
                    SendMessageW(
                        ok_btn.unwrap(),
                        WM_SETFONT,
                        Some(WPARAM(hfont.0 as usize)),
                        Some(LPARAM(1)),
                    );
                    // Cancel button
                    let cancel_btn = CreateWindowExW(
                        Default::default(),
                        PCWSTR(to_wide("BUTTON").as_ptr()),
                        PCWSTR(to_wide("Cancel").as_ptr()),
                        WS_CHILD | WS_VISIBLE,
                        210,
                        80,
                        60,
                        25,
                        Some(hwnd),
                        Some(HMENU(2 as usize as *mut _)),
                        None,
                        None,
                    );
                    SendMessageW(
                        cancel_btn.unwrap(),
                        WM_SETFONT,
                        Some(WPARAM(hfont.0 as usize)),
                        Some(LPARAM(1)),
                    );
                }
                WM_COMMAND => {
                    let id = wparam.0 as u16;
                    if id == 1 {
                        // OK
                        let mut mac_buf = [0u16; 32];
                        let mut ip_buf = [0u16; 32];
                        let mac_len = GetDlgItemTextW(hwnd, ID_MAC, &mut mac_buf);
                        let ip_len = GetDlgItemTextW(hwnd, ID_IP, &mut ip_buf);
                        let mac = String::from_utf16_lossy(&mac_buf[..mac_len as usize]);
                        let ip = String::from_utf16_lossy(&ip_buf[..ip_len as usize]);
                        // Save config
                        super::win32_dialog::CONFIG.with(|cfg| {
                            *cfg.borrow_mut() = super::AppConfig { mac: mac.clone(), ip: ip.clone() };
                            super::save_config(&cfg.borrow());
                        });
                        MessageBoxW(
                            Some(hwnd),
                            PCWSTR(to_wide(&format!("Saved!\nMAC: {}\nIP: {}", mac, ip)).as_ptr()),
                            PCWSTR(to_wide("Config Saved").as_ptr()),
                            MB_OK,
                        );
                    } else if id == 2 {
                        // Cancel
                        DestroyWindow(hwnd);
                    }
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                }
                _ => {}
            }
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }

    fn to_wide(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }
}

enum Message {
    Quit,
    SendPacket,
    ShowConfig,
}

fn main() {
    let mut tray = TrayItem::new("Magic Packet sender", IconSource::Resource("exe-icon")).unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let send_packet_tx = tx.clone();
    tray.add_menu_item("Send Packet", move || {
        send_packet_tx.send(Message::SendPacket).unwrap();
    })
    .unwrap();

    let config_tx = tx.clone();
    tray.add_menu_item("Config", move || {
        config_tx.send(Message::ShowConfig).unwrap();
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

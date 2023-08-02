#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use core::{sync::atomic::{AtomicBool, Ordering}, time};
use hookmap_core::{
    button::{Button, ButtonAction},
    event::Event,
};
use lazy_static::{__Deref, lazy_static};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::{sync::{Mutex}, thread};
use tauri::{AppHandle, Manager};

#[derive(Debug, Deserialize, Serialize)]
struct CODES {
    code: String
}

static AUTO_PRESS_KEY: AtomicBool = AtomicBool::new(true);
static RANDOM_KEY_BIND: AtomicBool = AtomicBool::new(false);
static CHANGE_KEYBIND: AtomicBool = AtomicBool::new(false);
static IS_TYPING: AtomicBool = AtomicBool::new(false);
lazy_static! {
    static ref CUR_NEXT_POS: Mutex<usize> = Mutex::new(1);
    static ref CODES_STRING: String = include_str!("./codes.json").to_string();
    static ref CODES_PARSED: Vec<CODES> = serde_json::from_str(&CODES_STRING.to_string()).unwrap();
    static ref PREV_CODE: Mutex<String> = Mutex::new(String::from("NONE"));
    static ref CUR_CODE: Mutex<String> = Mutex::new(String::from(CODES_PARSED[0].code.to_string()));
    static ref NEXT_CODE: Mutex<String> = Mutex::new(String::from(CODES_PARSED[1].code.to_string()));
    static ref CUR_KEYBIND: Mutex<u8> = Mutex::new(0);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.app_handle();
            std::thread::spawn(move || {
                let rx = hookmap_core::install_hook();

                while let Ok((event, native_handler)) = rx.recv() {
                    match event {
                        Event::Button(event) => {
                            native_handler.dispatch();
                            let cur_keybind = u8_to_button(*CUR_KEYBIND.lock().unwrap().deref());
                            let is_typing = IS_TYPING.load(Ordering::Relaxed);
                            if cur_keybind == event.target && event.action == ButtonAction::Release && !is_typing
                            {
                                let _ = &handle.emit_all("keybind_pressed", "".to_string());
                                type_code(&handle);
                            }
                        }
                        _ => {}
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_auto_press_key,
            set_auto_press_key,
            get_random_key_bind,
            set_random_key_bind,
            get_codes,
            get_keybind,
            set_keybind,
            get_cur_code,
            get_next_code,
            get_prev_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_auto_press_key() -> bool {
    AUTO_PRESS_KEY.load(Ordering::Relaxed)
}

#[tauri::command]
fn set_auto_press_key(val: bool) {
    AUTO_PRESS_KEY.store(val, Ordering::Relaxed)
}

#[tauri::command]
fn get_random_key_bind() -> bool {
    RANDOM_KEY_BIND.load(Ordering::Relaxed)
}

#[tauri::command]
fn set_random_key_bind(val: bool) {
    RANDOM_KEY_BIND.store(val, Ordering::Relaxed)
}

#[tauri::command]
fn get_keybind() -> u8 {
    *CUR_KEYBIND.lock().unwrap().deref()
}

#[tauri::command]
fn set_keybind() {
    CHANGE_KEYBIND.store(true, Ordering::Relaxed)
}

#[tauri::command]
fn get_cur_code() -> String {
    CUR_CODE.lock().unwrap().deref().to_string()
}

#[tauri::command]
fn get_next_code() -> String {
    NEXT_CODE.lock().unwrap().deref().to_string()
}

#[tauri::command]
fn get_prev_code() -> String {
    PREV_CODE.lock().unwrap().deref().to_string()
}

#[tauri::command]
fn get_codes() -> String {
    CODES_STRING.to_string()
}

fn type_code(handle: &AppHandle) {
    let mut cur_code = CUR_CODE.lock().unwrap();
    let mut prev_code = PREV_CODE.lock().unwrap();
    let mut next_code = NEXT_CODE.lock().unwrap();
    let mut cur_keybind = CUR_KEYBIND.lock().unwrap();
    let mut cur_next_pos = CUR_NEXT_POS.lock().unwrap();
    let auto_press_key = AUTO_PRESS_KEY.load(Ordering::Relaxed);
    let random_key_bind = RANDOM_KEY_BIND.load(Ordering::Relaxed);
    IS_TYPING.store(true, Ordering::Relaxed);
    Button::Backspace.press();
    Button::Backspace.release();
    thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(100..500)));
    for char in (*cur_code).chars() {
        u8_to_button(char.to_string().parse::<u8>().unwrap()).press();
        u8_to_button(char.to_string().parse::<u8>().unwrap()).release();
        thread::sleep(time::Duration::from_millis(rand::thread_rng().gen_range(100..500)));
    }
    *prev_code = cur_code.deref().to_string();
    *cur_code = next_code.deref().to_string();
    *next_code = CODES_PARSED[*cur_next_pos].code.to_string();
    *cur_next_pos = *cur_next_pos + 1;
    if auto_press_key && random_key_bind {
        *cur_keybind = rand::thread_rng().gen_range(0..10);
        let _ = handle.emit_all("keybind_changed", *cur_keybind);
    }
    let _ = handle.emit_all("update_cur_code", &*cur_code);
    let _ = handle.emit_all("update_prev_code", &*prev_code);
    let _ = handle.emit_all("update_next_code", &*next_code);
    IS_TYPING.store(false, Ordering::Relaxed);
}

fn u8_to_button(val: u8) -> Button {
    match val {
        0 => Button::Numpad0,
        1 => Button::Numpad1,
        2 => Button::Numpad2,
        3 => Button::Numpad3,
        4 => Button::Numpad4,
        5 => Button::Numpad5,
        6 => Button::Numpad6,
        7 => Button::Numpad7,
        8 => Button::Numpad8,
        9 => Button::Numpad9,
        _ => Button::Esc,
    }
}
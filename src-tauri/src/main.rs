// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod commands;
mod controllers;

use std::sync::Mutex;

use controllers::contract::Contract;
use commands::contract;

fn main() {
  let contract = Mutex::new(Contract::new());

  tauri::Builder::default()
    .manage(contract)
    .invoke_handler(tauri::generate_handler![
      contract::read_buyer,
      contract::read_vendor,
      contract::write_buyer,
      contract::write_vendor,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

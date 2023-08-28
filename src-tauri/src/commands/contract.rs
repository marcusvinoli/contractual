use std::sync::Mutex;

use tauri::State;

use crate::controllers::contract::Contract;

#[tauri::command] 
pub fn read_buyer(contract: State<'_, Mutex<Contract>>) -> String {
  contract
    .lock()
    .unwrap()
    .get_buyer_as_json()
} 

#[tauri::command] 
pub fn read_vendor(contract: State<'_, Mutex<Contract>>) -> String {
  contract
    .lock()
    .unwrap()
    .get_vendor_as_json()
} 

#[tauri::command] 
pub fn write_buyer(contract: State<'_, Mutex<Contract>>, data: String) {
  contract
    .lock()
    .unwrap()
    .set_buyer_from_json(&data)
    .expect("Error writing Buyer data"); 
  //.set_buyer_from_json(&data).expect("Error writing Buyer")
} 

#[tauri::command] 
pub fn write_vendor(contract: State<'_, Mutex<Contract>>, data: String) {
  contract
    .lock()
    .unwrap()
    .set_vendor_from_json(&data)
    .expect("Error writing Buyer data");
} 

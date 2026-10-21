use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MACRO_REGISTERS: Mutex<HashMap<char, String>> = Mutex::new(HashMap::new());
}

pub fn save_macro(register: char, element_path: String) {
    let mut regs = MACRO_REGISTERS.lock().unwrap();
    regs.insert(register, element_path);
}

pub fn get_macro(register: char) -> Option<String> {
    let regs = MACRO_REGISTERS.lock().unwrap();
    regs.get(&register).cloned()
}

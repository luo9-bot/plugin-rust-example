pub mod core;
use luo9_sdk::Bot;
use luo9_sdk::command::Command;
use luo9_sdk::command::PrefixMode;

use std::ffi::CString;

pub fn handle_private_msg(user_id: u64, msg: &str) {
    match Command::parse(msg, "echo", PrefixMode::Required('/')) {
        Some(cmd) => {
            let c_msg = CString::new(cmd.args_raw()).unwrap();
            Bot::send_private_msg(user_id, c_msg);
        }
        None => return,
    };

}

pub fn handle_group_msg(group_id: u64, user_id: u64, msg: &str) {
    match Command::parse(msg, "echo", PrefixMode::Required('/')) {
        Some(cmd) => {
            let c_msg = CString::new(cmd.args_raw()).unwrap();
            Bot::send_group_msg(group_id, c_msg);
        }
        None => return,
    };

}

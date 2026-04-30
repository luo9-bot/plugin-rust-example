pub mod core;

use luo9_sdk::Bot;
use luo9_sdk::command::{Command, PrefixMode};
use luo9_sdk::payload::*;
use std::ffi::CString;

// ── 消息处理 ────────────────────────────────────────────────────

pub fn handle_private_msg(user_id: u64, msg: &str) {
    match Command::parse(msg, "echo", PrefixMode::Required('/')) {
        Some(cmd) => {
            let c_msg = CString::new(cmd.args_raw()).unwrap();
            Bot::send_private_msg(user_id, c_msg);
        }
        None => return,
    }
}

pub fn handle_group_msg(group_id: u64, user_id: u64, msg: &str) {
    match Command::parse(msg, "echo", PrefixMode::Required('/')) {
        Some(cmd) => {
            let c_msg = CString::new(cmd.args_raw()).unwrap();
            Bot::send_group_msg(group_id, c_msg);
        }
        None => return,
    }
}

// ── 元事件处理 ──────────────────────────────────────────────────

pub fn handle_meta_event(ev: MetaEventPayload) {
    match ev.meta_event_type {
        MetaEventType::Heartbeat => { /* 心跳处理 */ }
        MetaEventType::Lifecycle => { /* 生命周期事件 */ }
        _ => {}
    }
}

// ── 通知处理 ────────────────────────────────────────────────────

pub fn handle_notice(notice: NoticePayload) {
    match notice.notice_type {
        NoticeType::GroupIncrease => { /* 新成员入群 */ }
        NoticeType::FriendAdd => { /* 新好友 */ }
        _ => {}
    }
}

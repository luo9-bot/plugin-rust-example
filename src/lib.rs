pub mod core;

use luo9_sdk::Bot;
use luo9_sdk::bus::Bus;
use luo9_sdk::command::{Command, PrefixMode};
use luo9_sdk::payload::*;
use std::ffi::CString;
use serde_json::json;


// ── 消息处理 ────────────────────────────────────────────────────

pub fn handle_private_msg(user_id: u64, msg: &str) {
    // /echo <text>
    if let Some(cmd) = Command::parse(msg, "echo", PrefixMode::Required('/')) {
        Bot::send_private_msg(user_id, CString::new(cmd.args().join(" ")).unwrap());
    }
    // /task start <name> <cron> [payload] | /task end <name>
    if let Some(cmd) = Command::parse(msg, "task", PrefixMode::Required('/')) {
        let reply = |text: String| { let _ = Bot::send_private_msg(user_id, CString::new(text).unwrap()); };
        cmd.on("start", |args| handle_task_start(&reply, args))
            .on("end", |args| handle_task_end(&reply, args));
    }
}

pub fn handle_group_msg(group_id: u64, _user_id: u64, msg: &str) {
    // /echo <text>
    if let Some(cmd) = Command::parse(msg, "echo", PrefixMode::Required('/')) {
        Bot::send_group_msg(group_id, CString::new(cmd.args().join(" ")).unwrap());
    }
    // /task start <name> <cron> [payload] | /task end <name>
    if let Some(cmd) = Command::parse(msg, "task", PrefixMode::Required('/')) {
        let reply = |text: String| { let _ = Bot::send_group_msg(group_id, CString::new(text).unwrap()); };
        cmd.on("start", |args| handle_task_start(&reply, args))
            .on("end", |args| handle_task_end(&reply, args));
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

fn handle_task_event(json: &str) {
    let Ok(event) = serde_json::from_str::<serde_json::Value>(json) else {
        return;
    };
    let task_name = event["task_name"].as_str().unwrap_or("unknown");
    let payload = event["payload"].as_str().unwrap_or("");
    println!("[task] 定时任务触发: name={}, payload={}", task_name, payload);
}

fn handle_task_start(reply: &dyn Fn(String), args: &[String]) {
    // /task start <name> <秒 分 时 日 月 周> [payload]
    if args.len() < 7 {
        reply("用法: /task start <名称> <秒 分 时 日 月 周> [payload]".to_string());
        return;
    }
    let name = &args[0];
    let cron = args[1..7].join(" ");
    let payload = if args.len() > 7 { args[7..].join(" ") } else { String::new() };
    let req = json!({
        "action": "schedule",
        "task_name": name,
        "cron": cron,
        "payload": payload
    });
    let _ = Bus::topic("luo9_task_miso").publish(&req.to_string());
    reply(format!("已创建任务: {} [{}]", name, cron));
}

fn handle_task_end(reply: &dyn Fn(String), args: &[String]) {
    // /task end <name>
    let Some(name) = args.first() else {
        reply("用法: /task end <名称>".to_string());
        return;
    };
    let req = json!({
        "action": "cancel",
        "task_name": name
    });
    let _ = Bus::topic("luo9_task_miso").publish(&req.to_string());
    reply(format!("已取消任务: {}", name));
}

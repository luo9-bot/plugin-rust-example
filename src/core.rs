use luo9_sdk::bus::Bus;
use luo9_sdk::payload::*;

use crate::{handle_private_msg, handle_group_msg, handle_meta_event, handle_notice, handle_task_event};

#[unsafe(no_mangle)]
pub extern "C" fn plugin_main() {
    let msg_sub = Bus::topic("luo9_message").subscribe().unwrap();
    let event_sub = Bus::topic("luo9_meta_event").subscribe().unwrap();
    let notice_sub = Bus::topic("luo9_notice").subscribe().unwrap();
    let task_sub = Bus::topic("luo9_task").subscribe().unwrap();
    let ver_sub = Bus::topic("luo9_version").subscribe().unwrap();

    let msg_topic = Bus::topic("luo9_message");
    let event_topic = Bus::topic("luo9_meta_event");
    let notice_topic = Bus::topic("luo9_notice");
    let task_topic = Bus::topic("luo9_task");
    let ver_topic = Bus::topic("luo9_version");

    loop {
        // ── 消息 ──
        if let Some(json) = msg_topic.pop(msg_sub) {
            if let Some(BusPayload::Message(msg)) = BusPayload::parse(&json) {
                match msg.message_type {
                    MsgType::Group => {
                        handle_group_msg(msg.group_id.unwrap_or(0), msg.user_id, &msg.message);
                    }
                    MsgType::Private => {
                        handle_private_msg(msg.user_id, &msg.message);
                    }
                    _ => {}
                }
            }
        }

        // ── 元事件 ──
        if let Some(json) = event_topic.pop(event_sub) {
            if let Some(BusPayload::MetaEvent(ev)) = BusPayload::parse(&json) {
                handle_meta_event(ev);
            }
        }

        // ── 通知 ──
        if let Some(json) = notice_topic.pop(notice_sub) {
            if let Some(BusPayload::Notice(notice)) = BusPayload::parse(&json) {
                handle_notice(notice);
            }
        }

        // ── 定时任务事件 ──
        if let Some(json) = task_topic.pop(task_sub) {
            handle_task_event(&json);
        }

        // ── 版本查询 ──
        if let Some(json) = ver_topic.pop(ver_sub) {
            if luo9_sdk::version::is_version_query(&json) {
                luo9_sdk::version::reply_version(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            }
        }

        // 短暂让出 CPU，避免空转
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

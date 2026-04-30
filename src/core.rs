use luo9_sdk::bus::Bus;
use luo9_sdk::payload::*;
use std::time::{Duration, Instant};

use crate::{handle_private_msg, handle_group_msg, handle_meta_event, handle_notice};

const POLL_RATE: Duration = Duration::from_millis(10); // 100Hz

#[unsafe(no_mangle)]
pub extern "C" fn plugin_main() {
    let msg_topic = Bus::topic("luo9_message");
    let event_topic = Bus::topic("luo9_meta_event");
    let notice_topic = Bus::topic("luo9_notice");

    loop {
        let tick = Instant::now();

        // ── 消息 ──
        if let Some(json) = msg_topic.pop() {
            if let Some(BusPayload::Message(msg)) = BusPayload::parse(&json) {
                match msg.message_type {
                    MsgType::Group => {
                        handle_group_msg(
                            msg.group_id.unwrap_or(0),
                            msg.user_id,
                            &msg.message,
                        );
                    }
                    MsgType::Private => {
                        handle_private_msg(msg.user_id, &msg.message);
                    }
                    _ => {}
                }
            }
        }

        // ── 元事件 ──
        if let Some(json) = event_topic.pop() {
            if let Some(BusPayload::MetaEvent(ev)) = BusPayload::parse(&json) {
                handle_meta_event(ev);
            }
        }

        // ── 通知 ──
        if let Some(json) = notice_topic.pop() {
            if let Some(BusPayload::Notice(notice)) = BusPayload::parse(&json) {
                handle_notice(notice);
            }
        }

        // rate 控制
        let elapsed = tick.elapsed();
        if elapsed < POLL_RATE {
            std::thread::sleep(POLL_RATE - elapsed);
        }
    }
}

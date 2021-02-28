use macroquad::prelude::warn;
use sapp_jsutils::JsObject;
use super::Event;
use super::MatchData;

extern "C" {
    fn nakama_connect(key: JsObject, server: JsObject, port: u32, protocol: JsObject);
    fn nakama_is_connected() -> bool;
    fn nakama_self_id() -> JsObject;
    fn nakama_send(opcode: i32, data: JsObject);
    fn nakama_try_recv() -> JsObject;
    fn nakama_events() -> JsObject;
}

#[no_mangle]
pub extern "C" fn quad_nakama_crate_version() -> u32 {
    (0 << 24) + (1 << 16) + 1
}

pub fn connect(key: &str, server: &str, port: u32, protocol: &str) {
    unsafe {
        nakama_connect(
            JsObject::string(key),
            JsObject::string(server),
            port,
            JsObject::string(protocol),
        );
    }
}

pub fn connected() -> bool {
    unsafe { nakama_is_connected() }
}

pub fn self_id() -> String {
    let mut id = String::new();
    let js_obj = unsafe { nakama_self_id() };
    js_obj.to_string(&mut id);

    id
}

pub fn send(opcode: i32, data: &[u8]) {
    unsafe { nakama_send(opcode, JsObject::buffer(data)) }
}

pub fn send_bin<T: serde::Serialize>(opcode: i32, data: &T) {
    match bincode::serialize(data) {
        Ok(ref data) => {
            send(opcode, data);
        }
        Err(err) => {
            warn!("Problem serializing data with error {}", err);
        }
    }
    ;
}

pub fn try_recv() -> Option<MatchData> {
    let js_obj = unsafe { nakama_try_recv() };
    if js_obj.is_nil() == false {
        let mut buf = vec![];
        let mut user_id = String::new();

        let opcode = js_obj.field_u32("opcode") as i32;
        js_obj.field("data").to_byte_buffer(&mut buf);
        js_obj.field("user_id").to_string(&mut user_id);

        return Some(MatchData {
            opcode,
            user_id,
            data: buf,
        });
    }
    None
}

pub fn events() -> Option<Event> {
    let js_obj = unsafe { nakama_events() };
    if js_obj.is_nil() == false {
        let mut user_id = String::new();

        js_obj.field("user_id").to_string(&mut user_id);
        let event_type = js_obj.field_u32("event");

        match event_type {
            1 => return Some(Event::Join(user_id)),
            2 => return Some(Event::Leave(user_id)),
            _ => panic!("Unknown nakama event type"),
        }
    }
    None
}
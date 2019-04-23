use std::io::stdin;
use std::collections::HashMap;
extern crate tungstenite;
extern crate url;
use url::Url;
use tungstenite::{Message, connect};
use serde_json::{Result, Value};
use super::rest;
use std::thread::{spawn, sleep};
use std::time;
use std::borrow::BorrowMut;
use std::thread;
use crossbeam::crossbeam_channel::unbounded;
use std::rc::Rc;
use std::sync::RwLock;
use std::cell::RefCell;
// Defining Opcodes here as per https://discordapp.com/developers/docs/topics/opcodes-and-status-codes#gateway
#[allow(dead_code)]
const OPCODE_DISPATCH: i32 = 0;
#[allow(dead_code)]
const OPCODE_HEARTBEAT: i32 = 1;
#[allow(dead_code)]
const OPCODE_IDENTIFY: i32 = 2;
#[allow(dead_code)]
const OPCODE_STATUS_UPDATE: i32 = 3;
#[allow(dead_code)]
const OPCODE_VOICE_STATE_UPDATE: i32 = 4;
#[allow(dead_code)]
const OPCODE_RESUME: i32 = 6;
#[allow(dead_code)]
const OPCODE_RECONNECT: i32 = 7;
#[allow(dead_code)]
const OPCODE_REQUEST_GUILD_MEMBERS: i32 = 8;
#[allow(dead_code)]
const OPCODE_INVAILD_SESSION: i32 = 9;
const OPCODE_HELLO: i32 = 10;
const OPCODE_HEARTBEAT_ACK: i32 = 11;

lazy_static! {
    static ref GLOBAL_SEQ_NUM: RwLock<String> = RwLock::new("null".to_string());
    static ref GLOBAL_HEARTBEAT_I: RwLock<Option<u64>> = RwLock::new(Some(0));
}

pub fn login(auth_token: String) {

    let (mut socket, response) = connect(Url::parse(&rest::findgateway().unwrap()).unwrap())
        .expect("Connection failed.");


    println!("Gateway connection established.");
    let handler = thread::spawn( move || {
        loop {
            println!("event loop");

            let msg = socket.read_message().expect("Error reading message");
            if let Message::Text(text) = msg {
                println!("Received: {}", text);
                let v: Value = serde_json::from_str(&text).unwrap();
                if v["op"] == OPCODE_HELLO {
                    let mut seq_num_mod = GLOBAL_SEQ_NUM.write().unwrap();
                    *seq_num_mod = v["s"].to_string();
                    let mut set_heartbeat_interval = GLOBAL_HEARTBEAT_I.write().unwrap();
                    *set_heartbeat_interval = v["d"]["heartbeat_interval"].as_u64();
                    socket.write_message(Message::Text(format!(r#"{{"op": 2,"d": {{"token": "{}","properties":{{"$os":"windows","$browser":"oxidecord","$device":"oxidecord"}},"compress":false,"large_threshold":250}}}} "#, auth_token).into())).unwrap();
                    socket.write_message(Message::Text(format!(r#"{{"op": 1,"d": {} }}"#, seq_num_mod)).into()).unwrap();
                }
                if v["op"] == OPCODE_HEARTBEAT_ACK {
                    let mut seq_num_mod = GLOBAL_SEQ_NUM.write().unwrap();
                    *seq_num_mod = v["s"].to_string();
                    println!("[OXIDECORD] Heartbeat ACK recieved.")
                }
                if v["op"] == OPCODE_DISPATCH {
                    let mut seq_num_mod = GLOBAL_SEQ_NUM.write().unwrap();
                    *seq_num_mod = v["s"].to_string();
                    println!("[OXIDECORD] Payload recieved.")
                }
            }
        }
    });
    thread::spawn(move|| {
        loop {
            let seq_num = GLOBAL_HEARTBEAT_I.read().unwrap();
            let heartbeat_sleep_millis = time::Duration::from_millis(seq_num.unwrap() - 4000);
            let now = time::Instant::now();
            thread::sleep(heartbeat_sleep_millis);
            let seq_num = GLOBAL_SEQ_NUM.read().unwrap();
            // socket.write_message(Message::Text(format!(r#"{{"op": 1,"d": {} }}"#, *seq_num)).into()).unwrap();
        }
    });
    handler.join();

}

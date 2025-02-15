// Copyright (c) 2025 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2025 Feb 15.

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Deserialize)]
pub struct Alarm {
    pub id: String,
    pub alert_time: i64,
    pub created_at: i64,
}

impl Alarm {
    pub fn to_js(&self) -> JsValue {
        let json_str = serde_json::to_string(self).unwrap();
        JsValue::from_str(&json_str)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DurablerResponse {
    pub status: u16,
    pub message: String,
}

impl DurablerResponse {
    pub fn new(status: u16, msg: &str) -> Self {
        Self {
            status,
            message: msg.to_string(),
        }
    }
}

// Copyright (c) 2025 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2025 Feb 15.

use std::borrow::Borrow;

use chrono::prelude::*;
use worker::*;

use super::model;

#[durable_object]
pub struct AlarmWKRS {
    state: State,
    env: Env,
}

#[durable_object]
impl DurableObject for AlarmWKRS {
    fn new(state: State, env: Env) -> Self {
        Self { state, env }
    }

    async fn fetch(&mut self, req: Request) -> Result<Response> {
        let env = self.env.clone();
        let router = Router::with_data(self);

        router
            .post_async("/alarm/:obj_id", |mut req, ctx| async move {
                let state = ctx.data.state.borrow();
                let mut storage = state.storage();
                let data = req.json::<model::Alarm>().await?;

                let mut resp_msg = "Alarm existed!".to_string();
                let prev_alarm = storage.get_alarm().await.unwrap();

                if prev_alarm.is_none() {
                    let dt = DateTime::from_timestamp(data.alert_time, 0).unwrap();
                    let scheduled_time = ScheduledTime::from(dt);

                    let mut opts = SetAlarmOptions::default();
                    opts.allow_unconfirmed = Some(true);

                    storage.put("alarm", data.id).await.unwrap();

                    state.wait_until(async move {
                        match storage.set_alarm(scheduled_time).await {
                            Ok(_) => console_log!("set_alarm: OK"),
                            Err(e) => console_log!("set_alarm: {:?}", e),
                        };
                    });

                    let scheduled_str = dt.format("%d/%m/%Y %H:%M:%S%.3f");
                    resp_msg = format!("Alarm set to {}", scheduled_str);
                } else {
                    // delete alarm if existed use for testing
                    storage.delete_alarm().await.unwrap();
                    storage.delete_all().await.unwrap();
                }

                console_debug!("[DEBUG] {}", resp_msg);

                Response::from_json(&model::DurablerResponse::new(200, resp_msg.as_str()))
            })
            .run(req, env)
            .await
    }

    async fn alarm(&mut self) -> Result<Response> {
        console_log!(">>>>>>>>>>>>>>>>>>>> Alarm triggered");
        let storage = self.state.storage();

        let vals = storage.list().await.unwrap();

        console_debug!("Vals: {:?}", vals);

        Response::from_json(&model::DurablerResponse::new(200, "Alarm triggered"))
    }
}

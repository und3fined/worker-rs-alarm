// Copyright (c) 2025 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2025 Feb 15.

use chrono::prelude::*;
use worker::*;

pub async fn register(evt: &ScheduledEvent, env: &Env) -> anyhow::Result<()> {
    let internal_host = env.var("INTERNAL_HOST").unwrap();
    match evt.cron().as_str() {
        "* * * * *" => {
            let scheduled_time = DateTime::from_timestamp_millis(evt.schedule() as i64).unwrap();
            console_log!(
                "Scheduled time {}",
                scheduled_time.format("%d/%m/%Y %H:%M:%S%.3f")
            );

            let alarm_do = env.durable_object("Duo").unwrap();
            let alarm_id = alarm_do.id_from_name("SCHEDULED_ALARM").unwrap();
            let alarm_stub = alarm_id.get_stub().unwrap();

            let current_time = Utc::now().timestamp();

            let alarm = super::model::Alarm {
                id: alarm_id.to_string(),
                alert_time: current_time + 5,
                created_at: current_time,
            };

            let mut req_init = RequestInit::new();
            req_init.with_method(Method::Post);
            req_init.with_body(Some(alarm.to_js()));

            let endpoint = format!(
                "{}/alarm/{}",
                internal_host.to_string(),
                alarm_id.to_string()
            );
            let req = Request::new_with_init(endpoint.as_str(), &req_init).unwrap();

            let mut resp = alarm_stub.fetch_with_request(req).await?;

            console_debug!("scheduler::register::alarm {}", resp.text().await?);
        }
        _ => console_log!("No handler for this scheduler"),
    }

    Ok(())
}

// Copyright (c) 2025 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2025 Feb 15.

mod durabler;
mod model;
mod scheduler;

use worker::*;

#[event(start)]
fn start() {
    console_error_panic_hook::set_once();
    console_log!("Worker RS Alarm is start")
}

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    Response::ok("Hello World!")
}

#[event(scheduled)]
async fn scheduled(event: ScheduledEvent, env: Env, ctx: ScheduleContext) {
    ctx.wait_until(async move {
        match scheduler::register(&event, &env).await {
            Ok(_) => console_debug!("Successfully processed scheduled event"),
            Err(e) => console_error!("Scheduler {}", e),
        }
    });
}

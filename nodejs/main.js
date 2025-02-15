// Copyright (c) 2025 und3fy.dev. All rights reserved.
// Created by und3fined <me@und3fy.dev> on 2025 Feb 16.

import { DurableObject } from 'cloudflare:workers';

export default {
  async fetch(_req, _env) {
    return new Response(JSON.stringify({ hello: "world" }), {
      headers: {
        "content-type": "application/json;charset=UTF-8",
      },
    });
  }
}


export class AlarmWK_JS extends DurableObject {
  constructor(state, env) {
    this.state = state;
    this.env = env;
  }

  async alarm() {
    console.log(">>>>> has alarrmmm");
  }
}

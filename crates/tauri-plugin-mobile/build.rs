// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

const COMMANDS: &[&str] = &["share_text"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .build();
}

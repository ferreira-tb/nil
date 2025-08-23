// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

package com.plugin.mobile.model

import app.tauri.annotation.InvokeArg

@InvokeArg
class ShareTextRequest {
  lateinit var text: String
  lateinit var mimeType: String
  lateinit var title: String
}
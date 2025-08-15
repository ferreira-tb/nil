// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

package com.plugin.mobile

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@TauriPlugin
class MobilePlugin(private val activity: Activity) : Plugin(activity) {
  @Command
  fun getAndroidVersion(invoke: Invoke) {
    val ret = JSObject()
    ret.put("sdkInt", android.os.Build.VERSION.SDK_INT)
    invoke.resolve(ret)
  }
}

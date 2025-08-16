// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

package com.plugin.mobile

import android.app.Activity
import android.content.Intent
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Plugin
import com.plugin.mobile.model.ShareTextRequest

@TauriPlugin
class MobilePlugin(private val activity: Activity) : Plugin(activity) {
  @Command
  fun shareText(invoke: Invoke) {
    val args = invoke.parseArgs(ShareTextRequest::class.java)
    val sendIntent = Intent().apply {
      this.action = Intent.ACTION_SEND
      this.type = args.mimeType
      this.putExtra(Intent.EXTRA_TEXT, args.text)
      this.putExtra(Intent.EXTRA_TITLE, args.title)
    }

    val shareIntent = Intent.createChooser(sendIntent, null)
    shareIntent.flags = Intent.FLAG_ACTIVITY_NEW_TASK
    activity.applicationContext?.startActivity(shareIntent)
  }
}

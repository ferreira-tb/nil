// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

package tsukilabs.nil

import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
  }
}

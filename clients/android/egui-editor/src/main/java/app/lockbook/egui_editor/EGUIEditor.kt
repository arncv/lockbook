package app.lockbook.egui_editor

import android.view.Surface

external fun createWgpuCanvas(surface: Surface, idx: Int): Long
external fun enterFrame(rustObj: Long)
external fun dropWgpuCanvas(rustObj: Long)

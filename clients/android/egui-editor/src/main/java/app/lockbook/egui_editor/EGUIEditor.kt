package app.lockbook.egui_editor

import android.view.Surface

class EGUIEditor {
    init {
        System.loadLibrary("egui_editor")
    }

    external fun createWgpuCanvas(surface: Surface, content: String, scale_factor: Float, dark_mode: Boolean): Long
    external fun enterFrame(rustObj: Long)
    external fun setText(rustObj: Long, content: String)
    external fun dropWgpuCanvas(rustObj: Long)
}
package app.lockbook.egui_editor

import android.text.Editable
import android.view.Surface

public class EGUIEditor {
    init {
        System.loadLibrary("egui_editor")
    }

    external fun createWgpuCanvas(surface: Surface, content: String, scaleFactor: Float, darkMode: Boolean): Long
    external fun enterFrame(rustObj: Long)
    external fun resizeEditor(rustObj: Long, scaleFactor: Float)

    external fun touchesBegin(rustObj: Long, id: Int, x: Float, y: Float, pressure: Float)
    external fun touchesMoved(rustObj: Long, id: Int, x: Float, y: Float, pressure: Float)
    external fun touchesEnded(rustObj: Long, id: Int, x: Float, y: Float, pressure: Float)


    external fun setText(rustObj: Long, content: String)
    external fun dropWgpuCanvas(rustObj: Long)
    external fun getTextBeforeCursor(rustObj: Long, n: Int): String
    external fun getTextAfterCursor(rustObj: Long, n: Int): String
    external fun getAllText(rustObj: Long): String
    external fun getSelectedText(rustObj: Long): String
    external fun deleteSurroundingText(rustObj: Long, beforeLength: Int, afterlength: Int)
    external fun setSelection(rustObj: Long, start: Int, end: Int)
    external fun sendKeyEvent(rustObj: Long, keyCode: Int, pressed: Boolean, alt: Boolean, ctrl: Boolean, shift: Boolean)
}
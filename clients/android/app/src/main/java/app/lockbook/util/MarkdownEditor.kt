package app.lockbook.util

import android.annotation.SuppressLint
import android.content.Context
import android.content.res.Configuration
import android.graphics.Canvas
import android.graphics.PixelFormat
import android.os.Bundle
import android.os.Handler
import android.text.InputType
import android.util.AttributeSet
import android.util.Log
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.SurfaceHolder
import android.view.SurfaceView
import android.view.View
import android.view.inputmethod.BaseInputConnection
import android.view.inputmethod.CompletionInfo
import android.view.inputmethod.CorrectionInfo
import android.view.inputmethod.EditorInfo
import android.view.inputmethod.ExtractedText
import android.view.inputmethod.ExtractedTextRequest
import android.view.inputmethod.InputConnection
import android.view.inputmethod.InputContentInfo
import android.view.inputmethod.InputMethodManager
import app.lockbook.egui_editor.EGUIEditor
import timber.log.Timber

class MarkdownEditor : SurfaceView, SurfaceHolder.Callback2 {
    private var wgpuObj: Long = Long.MAX_VALUE
    var content: String = "what is wrong with this string?"

    private var eguiEditor = EGUIEditor()
    private var inputManager = EGUIInputManager(eguiEditor, wgpuObj)

    constructor(context: Context) : super(context) {
    }
    constructor(context: Context, attrs: AttributeSet) : super(context, attrs) {
    }
    constructor(context: Context, attrs: AttributeSet, defStyle: Int) : super(
        context,
        attrs,
        defStyle
    ) {
    }

    constructor(context: Context, startingContent: String) : super(
        context,
    ) {
        content = startingContent
    }

    init {
        holder.addCallback(this)
        this.setZOrderOnTop(true)
        holder.setFormat(PixelFormat.TRANSPARENT)
    }

    fun adjustTouchPoint(axis: Float): Float {
        return axis / context.resources.displayMetrics.scaledDensity
    }

    @SuppressLint("ClickableViewAccessibility")
    override fun onTouchEvent(event: MotionEvent?): Boolean {
        println("touching...")
        if(event != null) {
            Timber.e("touch event registered: (${event.rawX}, ${event.rawY}) (${event.x}, ${event.y}) (${adjustTouchPoint(event.x)}, ${adjustTouchPoint(event.y)} (scale: ${context.resources.displayMetrics.scaledDensity})")

            when(event.action) {
                MotionEvent.ACTION_DOWN -> {
                    eguiEditor.touchesBegin(wgpuObj, event.getPointerId(0), adjustTouchPoint(event.x), adjustTouchPoint(event.y), event.pressure)
                }
                MotionEvent.ACTION_MOVE -> {
                    eguiEditor.touchesMoved(wgpuObj, event.getPointerId(0), adjustTouchPoint(event.x), adjustTouchPoint(event.y), event.pressure)
                }
                MotionEvent.ACTION_UP -> {
                    eguiEditor.touchesEnded(wgpuObj, event.getPointerId(0), adjustTouchPoint(event.x), adjustTouchPoint(event.y), event.pressure)
                }
            }
        }

        return false
    }


    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
    }

    override fun surfaceCreated(holder: SurfaceHolder) {
        holder.let { h ->
            wgpuObj = eguiEditor.createWgpuCanvas(h.surface, content, context.resources.displayMetrics.scaledDensity, (context.resources.configuration.uiMode and Configuration.UI_MODE_NIGHT_MASK) == Configuration.UI_MODE_NIGHT_YES)
            setWillNotDraw(false)
        }
    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {
        if (wgpuObj != Long.MAX_VALUE) {
            eguiEditor.dropWgpuCanvas(wgpuObj)
            wgpuObj = Long.MAX_VALUE
        }
    }

    override fun surfaceRedrawNeeded(holder: SurfaceHolder) {
    }

    override fun draw(canvas: Canvas?) {
        super.draw(canvas)
        if (wgpuObj == Long.MAX_VALUE) {
            return
        }

        if(!inputManager.stopEditsAndDisplay) {
            eguiEditor
            eguiEditor.enterFrame(wgpuObj)
        }

        invalidate()
    }

    override fun onCreateInputConnection(outAttrs: EditorInfo?): InputConnection {
        return inputManager
    }
}

data class ComposingRange(
    var start: Int = -1,
    var end: Int = -1
)

//class EGUIBaseInputManager(editorView: View, val eguiEditor: EGUIEditor, val wgpuObj: Long): BaseInputConnection(editorView, true) {
//
//
//
//}


class EGUIInputManager(val eguiEditor: EGUIEditor, val wgpuObj: Long): InputConnection {
    private val composingRange = ComposingRange()

    var stopEditsAndDisplay = false

    override fun getTextBeforeCursor(n: Int, flags: Int): CharSequence? {
        return eguiEditor.getTextBeforeCursor(wgpuObj, n)
    }

    override fun getTextAfterCursor(n: Int, flags: Int): CharSequence? {
        return eguiEditor.getTextAfterCursor(wgpuObj, n)
    }

    // not necessarily required
    override fun getSelectedText(flags: Int): CharSequence {
        return eguiEditor.getSelectedText(wgpuObj)
    }

    override fun getCursorCapsMode(reqModes: Int): Int {
        return InputType.TYPE_TEXT_FLAG_CAP_WORDS
    }

    override fun getExtractedText(request: ExtractedTextRequest?, flags: Int): ExtractedText {
        val extractedText = ExtractedText()
        extractedText.text = eguiEditor.getAllText(wgpuObj)

        return extractedText
    }

    override fun deleteSurroundingText(beforeLength: Int, afterLength: Int): Boolean {
        eguiEditor.deleteSurroundingText(wgpuObj, beforeLength, afterLength)

        return true
    }

    // not necessarily required
    override fun deleteSurroundingTextInCodePoints(beforeLength: Int, afterLength: Int): Boolean {
        return false
    }

    override fun setComposingText(text: CharSequence?, newCursorPosition: Int): Boolean {
        // todo

        return true
    }

    // not necessarily required
    override fun setComposingRegion(start: Int, end: Int): Boolean {
        return false
    }

    override fun finishComposingText(): Boolean {
        // todo

        return true
    }

    override fun commitText(text: CharSequence?, newCursorPosition: Int): Boolean {
        // todo

        return true
    }

    override fun commitCompletion(text: CompletionInfo?): Boolean {
        // todo

        return true
    }

    // not necessarily required
    override fun commitCorrection(correctionInfo: CorrectionInfo?): Boolean {
        return false
    }

    override fun setSelection(start: Int, end: Int): Boolean {
        eguiEditor.setSelection(wgpuObj, start, end)

        return true
    }

    // not important for our use case
    override fun performEditorAction(editorAction: Int): Boolean {
        return true
    }

    override fun performContextMenuAction(id: Int): Boolean {
        // todo
        return true
    }

    override fun beginBatchEdit(): Boolean {
        stopEditsAndDisplay = true
        return true
    }

    override fun endBatchEdit(): Boolean {
        stopEditsAndDisplay = false
        return false
    }

    override fun sendKeyEvent(event: KeyEvent?): Boolean {
        event?.let { realEvent ->
            eguiEditor.sendKeyEvent(wgpuObj, realEvent.keyCode, realEvent.action == KeyEvent.ACTION_DOWN, realEvent.isAltPressed, realEvent.isCtrlPressed, realEvent.isShiftPressed)
        }

        return true
    }

    override fun clearMetaKeyStates(states: Int): Boolean {
        return true
    }

    // not important for our use case
    override fun reportFullscreenMode(enabled: Boolean): Boolean {
        return true
    }

    // not important for our use case
    override fun performPrivateCommand(action: String?, data: Bundle?): Boolean {
        return true
    }

    // not necessarily required
    override fun requestCursorUpdates(cursorUpdateMode: Int): Boolean {
        TODO("Not yet implemented")
    }

    // not necessarily required
    override fun getHandler(): Handler? {
        return null
    }

    // not necessarily required
    override fun closeConnection() {}

    // not necessarily required for initial run
    override fun commitContent(
        inputContentInfo: InputContentInfo,
        flags: Int,
        opts: Bundle?
    ): Boolean {
        return true
    }

}
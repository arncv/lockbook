package app.lockbook.util

import android.content.Context
import android.content.res.Configuration
import android.graphics.Canvas
import android.graphics.PixelFormat
import android.os.Bundle
import android.os.Handler
import android.util.AttributeSet
import android.util.Log
import android.view.KeyEvent
import android.view.SurfaceHolder
import android.view.SurfaceView
import android.view.inputmethod.CompletionInfo
import android.view.inputmethod.CorrectionInfo
import android.view.inputmethod.EditorInfo
import android.view.inputmethod.ExtractedText
import android.view.inputmethod.ExtractedTextRequest
import android.view.inputmethod.InputConnection
import android.view.inputmethod.InputContentInfo
import app.lockbook.egui_editor.EGUIEditor

class MarkdownEditor : SurfaceView, SurfaceHolder.Callback2 {
    private var wgpuObj: Long = Long.MAX_VALUE
    var content: String = "what is wrong with this string?"

    private var eguiEditor = EGUIEditor()
    private var inputManager = EGUIInputManager(eguiEditor)

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
        println("SETTING")
        if (wgpuObj == Long.MAX_VALUE) {
            return
        }
        eguiEditor.enterFrame(wgpuObj)
        invalidate()
    }

    override fun onCreateInputConnection(outAttrs: EditorInfo?): InputConnection {
        return inputManager
    }
}

class EGUIInputManager(var eguiEditor: EGUIEditor): InputConnection {
    override fun getTextBeforeCursor(n: Int, flags: Int): CharSequence? {
        TODO("Not yet implemented")
    }

    override fun getTextAfterCursor(n: Int, flags: Int): CharSequence? {
        TODO("Not yet implemented")
    }

    override fun getSelectedText(flags: Int): CharSequence {
        TODO("Not yet implemented")
    }

    override fun getCursorCapsMode(reqModes: Int): Int {
        TODO("Not yet implemented")
    }

    override fun getExtractedText(request: ExtractedTextRequest?, flags: Int): ExtractedText {
        TODO("Not yet implemented")
    }

    override fun deleteSurroundingText(beforeLength: Int, afterLength: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun deleteSurroundingTextInCodePoints(beforeLength: Int, afterLength: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun setComposingText(text: CharSequence?, newCursorPosition: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun setComposingRegion(start: Int, end: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun finishComposingText(): Boolean {
        TODO("Not yet implemented")
    }

    override fun commitText(text: CharSequence?, newCursorPosition: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun commitCompletion(text: CompletionInfo?): Boolean {
        TODO("Not yet implemented")
    }

    override fun commitCorrection(correctionInfo: CorrectionInfo?): Boolean {
        TODO("Not yet implemented")
    }

    override fun setSelection(start: Int, end: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun performEditorAction(editorAction: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun performContextMenuAction(id: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun beginBatchEdit(): Boolean {
        TODO("Not yet implemented")
    }

    override fun endBatchEdit(): Boolean {
        TODO("Not yet implemented")
    }

    override fun sendKeyEvent(event: KeyEvent?): Boolean {
        TODO("Not yet implemented")
    }

    override fun clearMetaKeyStates(states: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun reportFullscreenMode(enabled: Boolean): Boolean {
        TODO("Not yet implemented")
    }

    override fun performPrivateCommand(action: String?, data: Bundle?): Boolean {
        TODO("Not yet implemented")
    }

    override fun requestCursorUpdates(cursorUpdateMode: Int): Boolean {
        TODO("Not yet implemented")
    }

    override fun getHandler(): Handler {
        TODO("Not yet implemented")
    }

    override fun closeConnection() {
        TODO("Not yet implemented")
    }

    override fun commitContent(
        inputContentInfo: InputContentInfo,
        flags: Int,
        opts: Bundle?
    ): Boolean {
        TODO("Not yet implemented")
    }

}
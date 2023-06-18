package app.lockbook.util

import android.content.Context
import android.content.res.Configuration
import android.graphics.Canvas
import android.graphics.PixelFormat
import android.util.AttributeSet
import android.util.Log
import android.view.SurfaceHolder
import android.view.SurfaceView
import app.lockbook.egui_editor.EGUIEditor
import timber.log.Timber

class MarkdownEditor : SurfaceView, SurfaceHolder.Callback2 {
    private var wgpuObj: Long = Long.MAX_VALUE
    var content: String = "what is wrong with this string?"

    private var eguiEditor = EGUIEditor()

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

    fun setText(content: String) {
        println("TEXT")
        eguiEditor.setText(wgpuObj, content)
    }

}
package app.lockbook.util

import app.lockbook.egui_editor.createWgpuCanvas
import app.lockbook.egui_editor.dropWgpuCanvas
import app.lockbook.egui_editor.enterFrame
import android.content.Context
import android.graphics.Canvas
import android.graphics.PixelFormat
import android.util.AttributeSet
import android.util.Log
import android.view.SurfaceHolder
import android.view.SurfaceView

class MarkdownEditor : SurfaceView, SurfaceHolder.Callback2 {
    private var wgpuObj: Long = Long.MAX_VALUE
    private var idx: Int = 0

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
            wgpuObj = createWgpuCanvas(h.surface, this.idx)
            setWillNotDraw(false)
        }
    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {
        if (wgpuObj != Long.MAX_VALUE) {
            dropWgpuCanvas(wgpuObj)
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
        enterFrame(wgpuObj)
        invalidate()
    }

}
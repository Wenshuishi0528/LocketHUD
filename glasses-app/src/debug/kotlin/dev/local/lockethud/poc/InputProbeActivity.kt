package dev.local.lockethud.poc

import android.app.Activity
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.os.Bundle
import android.util.Log
import android.view.GestureDetector
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.View
import android.view.WindowInsets

class InputProbeActivity : Activity() {
    private lateinit var probeView: ProbeView
    private lateinit var gestureDetector: GestureDetector

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        window.insetsController?.hide(WindowInsets.Type.statusBars() or WindowInsets.Type.navigationBars())
        probeView = ProbeView()
        gestureDetector = GestureDetector(this, GestureListener())
        setContentView(probeView)
        record("Input Probe ready")
    }

    override fun dispatchKeyEvent(event: KeyEvent): Boolean {
        record(
            "KEY action=${keyActionName(event.action)} " +
                "code=${KeyEvent.keyCodeToString(event.keyCode)}(${event.keyCode}) " +
                "scan=${event.scanCode} repeat=${event.repeatCount} source=0x${event.source.toString(16)}",
        )
        return super.dispatchKeyEvent(event)
    }

    override fun dispatchTouchEvent(event: MotionEvent): Boolean {
        record(
            "MOTION action=${MotionEvent.actionToString(event.actionMasked)} " +
                "x=${event.x.toInt()} y=${event.y.toInt()} source=0x${event.source.toString(16)} " +
                "buttons=0x${event.buttonState.toString(16)}",
        )
        gestureDetector.onTouchEvent(event)
        return super.dispatchTouchEvent(event)
    }

    override fun onGenericMotionEvent(event: MotionEvent): Boolean {
        record(
            "GENERIC action=${MotionEvent.actionToString(event.actionMasked)} " +
                "source=0x${event.source.toString(16)} buttons=0x${event.buttonState.toString(16)}",
        )
        return super.onGenericMotionEvent(event)
    }

    private fun record(message: String) {
        Log.i(TAG, message)
        probeView.add(message)
    }

    private fun keyActionName(action: Int): String = when (action) {
        KeyEvent.ACTION_DOWN -> "DOWN"
        KeyEvent.ACTION_UP -> "UP"
        else -> action.toString()
    }

    private inner class GestureListener : GestureDetector.SimpleOnGestureListener() {
        override fun onDown(event: MotionEvent): Boolean = true

        override fun onSingleTapConfirmed(event: MotionEvent): Boolean {
            record("GESTURE single_tap")
            return false
        }

        override fun onDoubleTap(event: MotionEvent): Boolean {
            record("GESTURE double_tap")
            return false
        }

        override fun onLongPress(event: MotionEvent) {
            record("GESTURE long_press")
        }

        override fun onScroll(
            first: MotionEvent?,
            second: MotionEvent,
            distanceX: Float,
            distanceY: Float,
        ): Boolean {
            record("GESTURE scroll dx=${distanceX.toInt()} dy=${distanceY.toInt()}")
            return false
        }
    }

    private inner class ProbeView : View(this@InputProbeActivity) {
        private val lines = ArrayDeque<String>()
        private val paint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
            color = Color.rgb(0, 255, 90)
            textSize = 14f
        }

        init {
            setBackgroundColor(Color.BLACK)
        }

        fun add(line: String) {
            lines.addLast(line)
            while (lines.size > MAX_LINES) lines.removeFirst()
            invalidate()
        }

        override fun onDraw(canvas: Canvas) {
            super.onDraw(canvas)
            val lineHeight = paint.textSize * 1.35f
            lines.forEachIndexed { index, line ->
                canvas.drawText(line.take(MAX_CHARS), 8f, 22f + index * lineHeight, paint)
            }
        }
    }

    companion object {
        private const val TAG = "LocketHUD.Input"
        private const val MAX_LINES = 28
        private const val MAX_CHARS = 72
    }
}

package dev.local.lockethud.poc

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.RectF
import android.view.View
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

class PortraitHudView(context: Context) : View(context) {
    private val loader = PortraitAssetLoader(context)
    private val portraitPaint = Paint(Paint.ANTI_ALIAS_FLAG or Paint.FILTER_BITMAP_FLAG)
    private val clockPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        color = Color.rgb(0, 210, 80)
        textAlign = Paint.Align.CENTER
    }
    private val minuteFormatter = SimpleDateFormat("HH:mm", Locale.getDefault())
    private val portraitRect = RectF()
    private var config = LocketConfig()
    private var bitmap: Bitmap? = null
    private val minuteTick = object : Runnable {
        override fun run() {
            if (config.clockEnabled) {
                invalidate()
                postDelayed(this, millisUntilNextMinute())
            }
        }
    }

    init {
        setBackgroundColor(Color.BLACK)
    }

    fun updateConfig(next: LocketConfig) {
        val safe = next.sanitized()
        val reload = safe.assetId != config.assetId || safe.size != config.size
        val clockChanged = safe.clockEnabled != config.clockEnabled
        config = safe
        if (reload && width > 0 && height > 0) loadBitmap()
        if (clockChanged) scheduleClockIfNeeded()
        invalidate()
    }

    override fun onSizeChanged(width: Int, height: Int, oldWidth: Int, oldHeight: Int) {
        super.onSizeChanged(width, height, oldWidth, oldHeight)
        loadBitmap()
    }

    override fun onAttachedToWindow() {
        super.onAttachedToWindow()
        scheduleClockIfNeeded()
    }

    override fun onDetachedFromWindow() {
        removeCallbacks(minuteTick)
        super.onDetachedFromWindow()
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)
        val portrait = bitmap
        if (config.visible && portrait != null) {
            val layout = PortraitLayoutCalculator.calculate(
                viewWidth = width,
                viewHeight = height,
                insetLeft = paddingLeft,
                insetTop = paddingTop,
                insetRight = paddingRight,
                insetBottom = paddingBottom,
                bitmapAspectRatio = portrait.width.toFloat() / portrait.height.coerceAtLeast(1),
                anchor = config.anchor,
                size = config.size,
            )
            portraitPaint.alpha = (config.opacity * 255f).toInt().coerceIn(0, 255)
            portraitRect.set(layout.left, layout.top, layout.right, layout.bottom)
            canvas.drawBitmap(portrait, null, portraitRect, portraitPaint)
        }
        if (config.clockEnabled) drawClock(canvas)
    }

    private fun loadBitmap() {
        bitmap = loader.load(
            config.assetId,
            (width * config.size.widthRatio).toInt().coerceAtLeast(1),
            height.coerceAtLeast(1),
        )
        invalidate()
    }

    private fun drawClock(canvas: Canvas) {
        val contentWidth = (width - paddingLeft - paddingRight).coerceAtLeast(1)
        val contentHeight = (height - paddingTop - paddingBottom).coerceAtLeast(1)
        clockPaint.textSize = contentWidth * 0.055f
        val x = if (config.anchor.horizontal == AnchorPreset.Horizontal.RIGHT) {
            paddingLeft + contentWidth * 0.13f
        } else {
            paddingLeft + contentWidth * 0.87f
        }
        val y = paddingTop + contentHeight * 0.09f - (clockPaint.ascent() + clockPaint.descent()) / 2f
        canvas.drawText(minuteFormatter.format(Date()), x, y, clockPaint)
    }

    private fun scheduleClockIfNeeded() {
        removeCallbacks(minuteTick)
        if (config.clockEnabled && isAttachedToWindow) postDelayed(minuteTick, millisUntilNextMinute())
    }

    private fun millisUntilNextMinute(): Long {
        val remainder = System.currentTimeMillis() % 60_000L
        return (60_000L - remainder).coerceAtLeast(1_000L)
    }
}

package dev.local.lockethud.poc

import android.content.Context
import android.graphics.BitmapFactory
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.RectF
import android.view.View

class CalibrationView(context: Context) : View(context) {
    private val linePaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        style = Paint.Style.STROKE
        color = Color.rgb(0, 255, 90)
    }
    private val fillPaint = Paint().apply { style = Paint.Style.FILL }
    private val textPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
        color = Color.rgb(0, 220, 80)
        textAlign = Paint.Align.LEFT
    }
    private val alphaRect = RectF()
    private val alphaBitmap by lazy { BitmapFactory.decodeResource(resources, R.drawable.alpha_test) }

    init {
        setBackgroundColor(Color.BLACK)
    }

    override fun onDraw(canvas: Canvas) {
        super.onDraw(canvas)
        val left = paddingLeft.toFloat()
        val top = paddingTop.toFloat()
        val right = (width - paddingRight).toFloat()
        val bottom = (height - paddingBottom).toFloat()
        val contentWidth = (right - left).coerceAtLeast(1f)
        val contentHeight = (bottom - top).coerceAtLeast(1f)
        val unit = contentWidth / 100f
        textPaint.textSize = unit * 3.2f

        linePaint.strokeWidth = 1f
        canvas.drawRect(left + 0.5f, top + 0.5f, right - 0.5f, bottom - 0.5f, linePaint)
        linePaint.color = Color.rgb(0, 100, 35)
        canvas.drawRect(left + contentWidth * 0.25f, top, left + contentWidth * 0.75f, bottom, linePaint)
        linePaint.color = Color.rgb(0, 255, 90)

        AnchorPreset.entries.forEach { anchor ->
            val rect = PortraitLayoutCalculator.calculate(
                width,
                height,
                paddingLeft,
                paddingTop,
                paddingRight,
                paddingBottom,
                0.68f,
                anchor,
                SizePreset.MEDIUM,
            )
            linePaint.strokeWidth = 1f
            canvas.drawRect(rect.left, rect.top, rect.right, rect.bottom, linePaint)
        }

        val stripTop = top + contentHeight * 0.34f
        for (lineWidth in 1..4) {
            linePaint.strokeWidth = lineWidth.toFloat()
            val y = stripTop + unit * lineWidth * 2.2f
            canvas.drawLine(left + unit * 28f, y, left + unit * 47f, y, linePaint)
            canvas.drawText("${lineWidth}px", left + unit * 49f, y + textPaint.textSize * 0.3f, textPaint)
        }

        drawLevels(canvas, left + unit * 28f, top + contentHeight * 0.52f, unit * 44f, unit * 5f, 8)
        drawLevels(canvas, left + unit * 28f, top + contentHeight * 0.60f, unit * 44f, unit * 5f, 16)

        val alphaTop = top + contentHeight * 0.69f
        alphaBitmap?.let { bitmap ->
            alphaRect.set(left + unit * 28f, alphaTop, left + unit * 48f, alphaTop + unit * 14f)
            canvas.drawBitmap(bitmap, null, alphaRect, fillPaint)
        }
        intArrayOf(64, 128, 192, 255).forEachIndexed { index, alpha ->
            fillPaint.color = Color.argb(alpha, 0, 255, 90)
            val blockLeft = left + unit * (50f + index * 5.5f)
            canvas.drawRect(blockLeft, alphaTop, blockLeft + unit * 4.5f, alphaTop + unit * 14f, fillPaint)
        }

        val sizeTop = top + contentHeight * 0.88f
        var cursor = left + unit * 28f
        SizePreset.entries.forEach { size ->
            val frameWidth = contentWidth * size.widthRatio
            val frameHeight = frameWidth * 0.75f
            linePaint.strokeWidth = 1f
            canvas.drawRect(cursor, sizeTop - frameHeight, cursor + frameWidth, sizeTop, linePaint)
            cursor += frameWidth + unit * 2f
        }

        canvas.drawText("SAFE 50%", left + contentWidth * 0.41f, top + textPaint.textSize * 1.2f, textPaint)
        canvas.drawText("8", left + unit * 25f, top + contentHeight * 0.56f, textPaint)
        canvas.drawText("16", left + unit * 24f, top + contentHeight * 0.64f, textPaint)
        canvas.drawText("ALPHA", left + unit * 28f, alphaTop - unit, textPaint)
        canvas.drawText("S / M / L", left + unit * 28f, sizeTop + textPaint.textSize * 1.2f, textPaint)
    }

    private fun drawLevels(
        canvas: Canvas,
        left: Float,
        top: Float,
        width: Float,
        height: Float,
        count: Int,
    ) {
        val blockWidth = width / count
        repeat(count) { index ->
            val value = (255f * index / (count - 1)).toInt()
            fillPaint.color = Color.rgb(0, value, (value * 0.35f).toInt())
            canvas.drawRect(left + index * blockWidth, top, left + (index + 1) * blockWidth, top + height, fillPaint)
        }
    }
}

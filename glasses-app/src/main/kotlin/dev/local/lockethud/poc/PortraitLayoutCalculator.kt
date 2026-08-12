package dev.local.lockethud.poc

import kotlin.math.max
import kotlin.math.min

data class FloatRect(val left: Float, val top: Float, val right: Float, val bottom: Float) {
    val width: Float get() = right - left
    val height: Float get() = bottom - top
}

object PortraitLayoutCalculator {
    fun calculate(
        viewWidth: Int,
        viewHeight: Int,
        insetLeft: Int,
        insetTop: Int,
        insetRight: Int,
        insetBottom: Int,
        bitmapAspectRatio: Float,
        anchor: AnchorPreset,
        size: SizePreset,
    ): FloatRect {
        val contentLeft = insetLeft.toFloat().coerceIn(0f, viewWidth.toFloat())
        val contentTop = insetTop.toFloat().coerceIn(0f, viewHeight.toFloat())
        val contentRight = (viewWidth - insetRight).toFloat().coerceAtLeast(contentLeft)
        val contentBottom = (viewHeight - insetBottom).toFloat().coerceAtLeast(contentTop)
        val contentWidth = max(1f, contentRight - contentLeft)
        val contentHeight = max(1f, contentBottom - contentTop)
        val margin = contentWidth * 0.04f
        val protectedLeft = contentLeft + contentWidth * 0.25f
        val protectedRight = contentLeft + contentWidth * 0.75f
        val sideBandWidth = when (anchor.horizontal) {
            AnchorPreset.Horizontal.LEFT -> protectedLeft - contentLeft
            AnchorPreset.Horizontal.RIGHT -> contentRight - protectedRight
        }
        val maxWidth = max(1f, sideBandWidth - margin)
        var portraitWidth = min(contentWidth * size.widthRatio, maxWidth)
        var portraitHeight = portraitWidth / bitmapAspectRatio.coerceAtLeast(0.05f)
        val maxHeight = max(1f, contentHeight - margin * 2f)
        if (portraitHeight > maxHeight) {
            portraitHeight = maxHeight
            portraitWidth = portraitHeight * bitmapAspectRatio.coerceAtLeast(0.05f)
        }

        val left = when (anchor.horizontal) {
            AnchorPreset.Horizontal.LEFT -> contentLeft + margin
            AnchorPreset.Horizontal.RIGHT -> contentRight - margin - portraitWidth
        }.coerceIn(contentLeft, max(contentLeft, contentRight - portraitWidth))
        val top = when (anchor.vertical) {
            AnchorPreset.Vertical.TOP -> contentTop + margin
            AnchorPreset.Vertical.MIDDLE -> contentTop + (contentHeight - portraitHeight) / 2f
            AnchorPreset.Vertical.BOTTOM -> contentBottom - margin - portraitHeight
        }.coerceIn(contentTop, max(contentTop, contentBottom - portraitHeight))

        return FloatRect(left, top, left + portraitWidth, top + portraitHeight)
    }
}

package dev.local.lockethud.poc

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class PortraitLayoutCalculatorTest {
    @Test
    fun sixAnchorsStayOutsideCentralProtectionZone() {
        AnchorPreset.entries.forEach { anchor ->
            val rect = PortraitLayoutCalculator.calculate(
                viewWidth = 480,
                viewHeight = 640,
                insetLeft = 0,
                insetTop = 0,
                insetRight = 0,
                insetBottom = 0,
                bitmapAspectRatio = 0.67f,
                anchor = anchor,
                size = SizePreset.LARGE,
            )

            assertTrue(rect.left >= 0f)
            assertTrue(rect.top >= 0f)
            assertTrue(rect.right <= 480f)
            assertTrue(rect.bottom <= 640f)
            if (anchor.horizontal == AnchorPreset.Horizontal.LEFT) {
                assertTrue(rect.right <= 120f)
            } else {
                assertTrue(rect.left >= 360f)
            }
        }
    }

    @Test
    fun aspectRatioIsPreservedWhenHeightIsClamped() {
        val rect = PortraitLayoutCalculator.calculate(
            viewWidth = 200,
            viewHeight = 80,
            insetLeft = 4,
            insetTop = 5,
            insetRight = 4,
            insetBottom = 5,
            bitmapAspectRatio = 0.5f,
            anchor = AnchorPreset.RIGHT_MIDDLE,
            size = SizePreset.LARGE,
        )

        assertEquals(0.5f, rect.width / rect.height, 0.001f)
        assertTrue(rect.top >= 5f)
        assertTrue(rect.bottom <= 75f)
    }

    @Test
    fun runtimeInsetsAreRespected() {
        val rect = PortraitLayoutCalculator.calculate(
            viewWidth = 480,
            viewHeight = 640,
            insetLeft = 10,
            insetTop = 20,
            insetRight = 30,
            insetBottom = 40,
            bitmapAspectRatio = 1f,
            anchor = AnchorPreset.RIGHT_BOTTOM,
            size = SizePreset.SMALL,
        )

        assertTrue(rect.left >= 10f)
        assertTrue(rect.top >= 20f)
        assertTrue(rect.right <= 450f)
        assertTrue(rect.bottom <= 600f)
    }
}

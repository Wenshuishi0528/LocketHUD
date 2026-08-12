package dev.local.lockethud.poc

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class LocketConfigTest {
    @Test
    fun codecRoundTripPreservesSupportedValues() {
        val original = LocketConfig(
            assetId = LocketConfig.ASSET_PRIVATE_GIF,
            anchor = AnchorPreset.LEFT_BOTTOM,
            size = SizePreset.LARGE,
            opacity = 0.6f,
            visible = false,
            keepScreenOn = false,
            clockEnabled = true,
            renderProfile = RenderProfile.DITHERED,
        )

        val decoded = LocketConfigCodec.decode(LocketConfigCodec.encode(original))

        assertEquals(original, decoded)
    }

    @Test
    fun invalidValuesFallBackToSafeDefaults() {
        val decoded = LocketConfigCodec.decode(
            mapOf(
                "schema_version" to "999",
                "asset_id" to "../../private",
                "anchor" to "center",
                "size" to "huge",
                "opacity" to "42",
                "visible" to "yes",
                "keep_screen_on" to "1",
                "clock_enabled" to "no",
                "render_profile" to "unknown",
            ),
        )

        assertEquals(LocketConfig.CURRENT_SCHEMA_VERSION, decoded.schemaVersion)
        assertEquals(LocketConfig.ASSET_DEFAULT, decoded.assetId)
        assertEquals(AnchorPreset.RIGHT_MIDDLE, decoded.anchor)
        assertEquals(SizePreset.MEDIUM, decoded.size)
        assertEquals(1.0f, decoded.opacity)
        assertTrue(decoded.visible)
        assertTrue(decoded.keepScreenOn)
        assertFalse(decoded.clockEnabled)
        assertEquals(RenderProfile.QUANTIZED_16, decoded.renderProfile)
    }
}

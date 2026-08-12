package dev.local.lockethud.poc

import kotlin.math.abs

enum class AnchorPreset(val wireName: String, val horizontal: Horizontal, val vertical: Vertical) {
    LEFT_TOP("left_top", Horizontal.LEFT, Vertical.TOP),
    LEFT_MIDDLE("left_middle", Horizontal.LEFT, Vertical.MIDDLE),
    LEFT_BOTTOM("left_bottom", Horizontal.LEFT, Vertical.BOTTOM),
    RIGHT_TOP("right_top", Horizontal.RIGHT, Vertical.TOP),
    RIGHT_MIDDLE("right_middle", Horizontal.RIGHT, Vertical.MIDDLE),
    RIGHT_BOTTOM("right_bottom", Horizontal.RIGHT, Vertical.BOTTOM);

    enum class Horizontal { LEFT, RIGHT }
    enum class Vertical { TOP, MIDDLE, BOTTOM }

    companion object {
        fun fromWireName(value: String?): AnchorPreset? = entries.firstOrNull { it.wireName == value }
    }
}

enum class SizePreset(val wireName: String, val widthRatio: Float) {
    SMALL("small", 0.14f),
    MEDIUM("medium", 0.18f),
    LARGE("large", 0.22f);

    companion object {
        fun fromWireName(value: String?): SizePreset? = entries.firstOrNull { it.wireName == value }
    }
}

object PortraitOpacity {
    val allowed = floatArrayOf(0.4f, 0.6f, 0.8f, 1.0f)

    fun nearest(value: Float): Float = allowed.minBy { abs(it - value) }

    fun parse(value: String?): Float? {
        val parsed = value?.toFloatOrNull() ?: return null
        return allowed.firstOrNull { abs(it - parsed) < 0.001f }
    }
}

enum class RenderProfile(val wireName: String) {
    NATURAL_GREEN("natural_green"),
    QUANTIZED_8("quantized_8"),
    QUANTIZED_16("quantized_16"),
    DITHERED("dithered");

    companion object {
        fun fromWireName(value: String?): RenderProfile? = entries.firstOrNull { it.wireName == value }
    }
}

data class LocketConfig(
    val schemaVersion: Int = CURRENT_SCHEMA_VERSION,
    val assetId: String = ASSET_DEFAULT,
    val anchor: AnchorPreset = AnchorPreset.RIGHT_MIDDLE,
    val size: SizePreset = SizePreset.MEDIUM,
    val opacity: Float = 0.8f,
    val visible: Boolean = true,
    val keepScreenOn: Boolean = true,
    val clockEnabled: Boolean = false,
    val renderProfile: RenderProfile = RenderProfile.QUANTIZED_16,
) {
    fun sanitized(): LocketConfig = copy(
        schemaVersion = CURRENT_SCHEMA_VERSION,
        assetId = if (assetId in ALLOWED_ASSET_IDS) assetId else ASSET_DEFAULT,
        opacity = PortraitOpacity.nearest(opacity.coerceIn(0.4f, 1.0f)),
    )

    companion object {
        const val CURRENT_SCHEMA_VERSION = 1
        const val ASSET_DEFAULT = "portrait_default"
        const val ASSET_PRIVATE = "portrait_private"
        const val ASSET_PRIVATE_GIF = "portrait_private_gif"
        val ALLOWED_ASSET_IDS = setOf(ASSET_DEFAULT, ASSET_PRIVATE, ASSET_PRIVATE_GIF)
    }
}

object LocketConfigCodec {
    fun encode(config: LocketConfig): Map<String, String> {
        val safe = config.sanitized()
        return mapOf(
            "schema_version" to safe.schemaVersion.toString(),
            "asset_id" to safe.assetId,
            "anchor" to safe.anchor.wireName,
            "size" to safe.size.wireName,
            "opacity" to safe.opacity.toString(),
            "visible" to safe.visible.toString(),
            "keep_screen_on" to safe.keepScreenOn.toString(),
            "clock_enabled" to safe.clockEnabled.toString(),
            "render_profile" to safe.renderProfile.wireName,
        )
    }

    fun decode(values: Map<String, String?>): LocketConfig {
        return LocketConfig(
            schemaVersion = values["schema_version"]?.toIntOrNull() ?: LocketConfig.CURRENT_SCHEMA_VERSION,
            assetId = values["asset_id"] ?: LocketConfig.ASSET_DEFAULT,
            anchor = AnchorPreset.fromWireName(values["anchor"]) ?: AnchorPreset.RIGHT_MIDDLE,
            size = SizePreset.fromWireName(values["size"]) ?: SizePreset.MEDIUM,
            opacity = values["opacity"]?.toFloatOrNull() ?: 0.8f,
            visible = values["visible"].strictBooleanOrNull() ?: true,
            keepScreenOn = values["keep_screen_on"].strictBooleanOrNull() ?: true,
            clockEnabled = values["clock_enabled"].strictBooleanOrNull() ?: false,
            renderProfile = RenderProfile.fromWireName(values["render_profile"])
                ?: RenderProfile.QUANTIZED_16,
        ).sanitized()
    }

    private fun String?.strictBooleanOrNull(): Boolean? = when (this) {
        "true" -> true
        "false" -> false
        else -> null
    }
}

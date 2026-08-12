package dev.local.lockethud.poc

import android.content.Context

class ConfigRepository(context: Context) {
    private val preferences = context.getSharedPreferences(PREFS_NAME, Context.MODE_PRIVATE)

    fun load(): LocketConfig {
        val keys = listOf(
            "schema_version",
            "asset_id",
            "anchor",
            "size",
            "opacity",
            "visible",
            "keep_screen_on",
            "clock_enabled",
            "render_profile",
        )
        return LocketConfigCodec.decode(keys.associateWith { preferences.getString(it, null) })
    }

    fun save(config: LocketConfig) {
        val editor = preferences.edit()
        LocketConfigCodec.encode(config).forEach { (key, value) -> editor.putString(key, value) }
        editor.apply()
    }

    companion object {
        private const val PREFS_NAME = "locket_config_v1"
    }
}

package dev.local.lockethud.poc

import android.app.Activity
import android.annotation.SuppressLint
import android.content.Intent
import android.graphics.Color
import android.os.Build
import android.os.Bundle
import android.util.Log
import android.view.Gravity
import android.view.View
import android.view.WindowInsets
import android.view.WindowManager
import android.window.OnBackInvokedDispatcher
import android.widget.TextView

class MainActivity : Activity() {
    private lateinit var repository: ConfigRepository
    private var config = LocketConfig()
    private var mode = DisplayMode.PORTRAIT

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        repository = ConfigRepository(this)
        config = repository.load()
        restoreVisibilityForLaunch(intent)
        applyDebugIntent(intent)
        configureWindow()
        if (Build.VERSION.SDK_INT >= 33) {
            onBackInvokedDispatcher.registerOnBackInvokedCallback(
                OnBackInvokedDispatcher.PRIORITY_DEFAULT,
            ) { handleBack() }
        }
        showCurrentMode()
    }

    override fun onNewIntent(intent: Intent) {
        super.onNewIntent(intent)
        setIntent(intent)
        restoreVisibilityForLaunch(intent)
        applyDebugIntent(intent)
        showCurrentMode()
        applyKeepScreenOn()
    }

    override fun onResume() {
        super.onResume()
        applyKeepScreenOn()
    }

    override fun onPause() {
        window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        super.onPause()
    }

    @SuppressLint("GestureBackNavigation")
    @Deprecated("Classic back behavior is retained for the API 32 target device")
    override fun onBackPressed() {
        handleBack()
    }

    private fun handleBack() {
        if (mode != DisplayMode.PORTRAIT) {
            mode = DisplayMode.PORTRAIT
            showCurrentMode()
            return
        }
        if (config.visible) {
            config = config.copy(visible = false)
            (window.decorView.findViewWithTag<View>(PORTRAIT_VIEW_TAG) as? PortraitHudView)
                ?.updateConfig(config)
            return
        }
        finish()
    }

    private fun configureWindow() {
        window.statusBarColor = Color.BLACK
        window.navigationBarColor = Color.BLACK
    }

    private fun showCurrentMode() {
        val view = when (mode) {
            DisplayMode.PORTRAIT -> PortraitHudView(this).apply {
                tag = PORTRAIT_VIEW_TAG
                updateConfig(config)
            }
            DisplayMode.CALIBRATION -> CalibrationView(this)
            DisplayMode.MINIMAL -> TextView(this).apply {
                setBackgroundColor(Color.BLACK)
                setTextColor(Color.rgb(0, 255, 90))
                setText(R.string.minimal_message)
                textSize = 20f
                gravity = Gravity.CENTER
            }
        }
        view.setOnApplyWindowInsetsListener { target, insets ->
            val bars = insets.getInsets(WindowInsets.Type.systemBars() or WindowInsets.Type.displayCutout())
            target.setPadding(bars.left, bars.top, bars.right, bars.bottom)
            insets
        }
        view.addOnLayoutChangeListener { _, left, top, right, bottom, _, _, _, _ ->
            if (right > left && bottom > top) {
                Log.i(TAG, "Runtime view=${right - left}x${bottom - top}; mode=${mode.wireName}")
            }
        }
        setContentView(view)
        hideSystemBars()
    }

    @Suppress("DEPRECATION")
    private fun hideSystemBars() {
        window.decorView.systemUiVisibility =
            View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY or
                View.SYSTEM_UI_FLAG_FULLSCREEN or
                View.SYSTEM_UI_FLAG_HIDE_NAVIGATION or
                View.SYSTEM_UI_FLAG_LAYOUT_STABLE or
                View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN or
                View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
    }

    private fun applyKeepScreenOn() {
        if (config.keepScreenOn && mode == DisplayMode.PORTRAIT) {
            window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        } else {
            window.clearFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        }
    }

    private fun restoreVisibilityForLaunch(intent: Intent) {
        if (!intent.hasExtra("visible")) {
            config = config.copy(visible = true)
        }
    }

    private fun applyDebugIntent(intent: Intent) {
        if (!BuildConfig.DEBUG) return
        val modeValue = intent.getStringExtra("mode")
        DisplayMode.fromWireName(modeValue)?.let { mode = it }

        var updated = config
        val anchor = AnchorPreset.fromWireName(intent.getStringExtra("anchor"))
        val size = SizePreset.fromWireName(intent.getStringExtra("size"))
        val opacity = PortraitOpacity.parse(intent.getStringExtra("opacity"))
        val asset = when (intent.getStringExtra("asset")) {
            "default" -> LocketConfig.ASSET_DEFAULT
            "private" -> LocketConfig.ASSET_PRIVATE
            "private_gif" -> LocketConfig.ASSET_PRIVATE_GIF
            else -> null
        }
        val keepScreenOn = strictBoolean(intent.getStringExtra("keep_screen_on"))
        val visible = strictBoolean(intent.getStringExtra("visible"))
        val clockEnabled = strictBoolean(intent.getStringExtra("clock_enabled"))
        val profile = RenderProfile.fromWireName(intent.getStringExtra("render_profile"))

        anchor?.let { updated = updated.copy(anchor = it) }
        size?.let { updated = updated.copy(size = it) }
        opacity?.let { updated = updated.copy(opacity = it) }
        asset?.let { updated = updated.copy(assetId = it) }
        keepScreenOn?.let { updated = updated.copy(keepScreenOn = it) }
        visible?.let { updated = updated.copy(visible = it) }
        clockEnabled?.let { updated = updated.copy(clockEnabled = it) }
        profile?.let { updated = updated.copy(renderProfile = it) }

        config = updated.sanitized()
        repository.save(config)
    }

    private fun strictBoolean(value: String?): Boolean? = when (value) {
        "true" -> true
        "false" -> false
        else -> null
    }

    private enum class DisplayMode(val wireName: String) {
        PORTRAIT("portrait"),
        CALIBRATION("calibration"),
        MINIMAL("minimal");

        companion object {
            fun fromWireName(value: String?): DisplayMode? = entries.firstOrNull { it.wireName == value }
        }
    }

    companion object {
        private const val TAG = "LocketHUD.Main"
        private const val PORTRAIT_VIEW_TAG = "portrait_view"
    }
}

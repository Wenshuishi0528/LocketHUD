package dev.local.lockethud.poc

import android.content.Context
import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.util.Log
import java.io.File
import kotlin.math.max

class PortraitAssetLoader(private val context: Context) {
    init {
        internalPortraitDirectory().mkdirs()
        externalPortraitDirectory()?.mkdirs()
    }

    fun load(assetId: String, targetWidth: Int, targetHeight: Int): Bitmap {
        if (assetId == LocketConfig.ASSET_PRIVATE) {
            privateCandidates().firstNotNullOfOrNull { decodeValidatedFile(it, targetWidth, targetHeight) }
                ?.let {
                    Log.i(TAG, "Loaded validated private portrait")
                    return it
                }
            Log.w(TAG, "Private portrait unavailable or invalid; using generated default")
        }
        return BitmapFactory.decodeResource(context.resources, R.drawable.portrait_default)
            ?: error("Generated default portrait is missing")
    }

    private fun decodeValidatedFile(file: File, targetWidth: Int, targetHeight: Int): Bitmap? {
        if (!file.isFile || file.length() !in 1..MAX_FILE_BYTES) return null
        val bounds = BitmapFactory.Options().apply { inJustDecodeBounds = true }
        BitmapFactory.decodeFile(file.absolutePath, bounds)
        if (bounds.outMimeType != "image/png") return null
        if (bounds.outWidth !in 1..MAX_DIMENSION || bounds.outHeight !in 1..MAX_DIMENSION) return null

        val options = BitmapFactory.Options().apply {
            inSampleSize = sampleSize(
                bounds.outWidth,
                bounds.outHeight,
                max(1, targetWidth),
                max(1, targetHeight),
            )
        }
        return BitmapFactory.decodeFile(file.absolutePath, options)
    }

    private fun privateCandidates(): List<File> = listOfNotNull(
        externalPortraitDirectory()?.resolve(PRIVATE_FILE_NAME),
        internalPortraitDirectory().resolve(PRIVATE_FILE_NAME),
    )

    private fun internalPortraitDirectory(): File = File(context.filesDir, "portraits")

    private fun externalPortraitDirectory(): File? = context.getExternalFilesDir("portraits")

    companion object {
        private const val TAG = "LocketHUD.Asset"
        private const val PRIVATE_FILE_NAME = "current.png"
        private const val MAX_FILE_BYTES = 8L * 1024L * 1024L
        private const val MAX_DIMENSION = 4096

        internal fun sampleSize(width: Int, height: Int, targetWidth: Int, targetHeight: Int): Int {
            var sample = 1
            while (width / (sample * 2) >= targetWidth && height / (sample * 2) >= targetHeight) {
                sample *= 2
            }
            return sample
        }
    }
}

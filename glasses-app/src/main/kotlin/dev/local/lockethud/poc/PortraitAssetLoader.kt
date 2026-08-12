package dev.local.lockethud.poc

import android.content.Context
import android.graphics.ImageDecoder
import android.graphics.drawable.Drawable
import android.util.Log
import java.io.File

class PortraitAssetLoader(private val context: Context) {
    init {
        internalPortraitDirectory().mkdirs()
        externalPortraitDirectory()?.mkdirs()
    }

    fun load(assetId: String, targetWidth: Int, targetHeight: Int): Drawable {
        if (assetId == LocketConfig.ASSET_PRIVATE || assetId == LocketConfig.ASSET_PRIVATE_GIF) {
            privateCandidates(assetId).firstNotNullOfOrNull {
                decodeValidatedFile(it, targetWidth, targetHeight, assetId)
            }?.let {
                Log.i(TAG, "Loaded validated private portrait; animated=${assetId == LocketConfig.ASSET_PRIVATE_GIF}")
                return it
            }
            Log.w(TAG, "Private portrait unavailable or invalid; using generated default")
        }
        return context.getDrawable(R.drawable.portrait_default)
            ?: error("Generated default portrait is missing")
    }

    private fun decodeValidatedFile(
        file: File,
        targetWidth: Int,
        targetHeight: Int,
        assetId: String,
    ): Drawable? {
        if (!file.isFile || file.length() !in 1..MAX_FILE_BYTES) return null
        return try {
            ImageDecoder.decodeDrawable(ImageDecoder.createSource(file)) { decoder, info, _ ->
                val expectedMime = if (assetId == LocketConfig.ASSET_PRIVATE_GIF) "image/gif" else "image/png"
                require(info.mimeType == expectedMime)
                require(info.size.width in 1..MAX_DIMENSION && info.size.height in 1..MAX_DIMENSION)
                decoder.setTargetSampleSize(
                    sampleSize(
                        info.size.width,
                        info.size.height,
                        targetWidth.coerceAtLeast(1),
                        targetHeight.coerceAtLeast(1),
                    ),
                )
            }
        } catch (error: Exception) {
            Log.w(TAG, "Rejected private portrait ${file.name}: ${error.message}")
            null
        }
    }

    private fun privateCandidates(assetId: String): List<File> {
        val fileName = if (assetId == LocketConfig.ASSET_PRIVATE_GIF) GIF_FILE_NAME else PNG_FILE_NAME
        return listOfNotNull(
            externalPortraitDirectory()?.resolve(fileName),
            internalPortraitDirectory().resolve(fileName),
        )
    }

    private fun internalPortraitDirectory(): File = File(context.filesDir, "portraits")

    private fun externalPortraitDirectory(): File? = context.getExternalFilesDir("portraits")

    companion object {
        private const val TAG = "LocketHUD.Asset"
        private const val PNG_FILE_NAME = "current.png"
        private const val GIF_FILE_NAME = "current.gif"
        private const val MAX_FILE_BYTES = 32L * 1024L * 1024L
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

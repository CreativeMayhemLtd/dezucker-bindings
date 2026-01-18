package coffee.outof.dezucker.plugins

import coffee.outof.dezucker.storage.InternalStorage
import coffee.outof.dezucker.types.PostFragment

data class ExportContext(
    val postId: Any? = null, // String | Long | null
    val timestamp: Long,
    val index: Int,
    val total: Int,
    val directory: String? = null,
    val media: List<PostFragment>? = null,
    val storage: InternalStorage,
    val extra: Map<String, Any?> = emptyMap()
)

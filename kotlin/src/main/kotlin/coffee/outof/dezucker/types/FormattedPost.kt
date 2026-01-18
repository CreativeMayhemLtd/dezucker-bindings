package coffee.outof.dezucker.types

data class FormattedPost(
    val tid: String? = null,
    val id: Any? = null, // string | number | null
    val text: String,
    val timestamp: Any? = null, // string | number | null
    val attachmentsCount: Int? = null,
    val meaningfulEntriesCount: Int,
    val tags: List<Map<String, String?>>? = null,
    val fragments: List<PostFragment>? = null,
    val media: List<PostFragment>? = null,
    val _raw: Any? = null
)

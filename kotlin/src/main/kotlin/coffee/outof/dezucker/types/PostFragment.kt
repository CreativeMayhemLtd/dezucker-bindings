package coffee.outof.dezucker.types

/**
 * Represents a structured fragment of a post for rendering.
 */
data class PostFragment(
    val text: String,
    val timestamp: Long,
    val mediaUri: String? = null,
    val webUri: String? = null,
    val isPhoto: Boolean = false,
    val isUnknown: Boolean = false,
    val isMeaningful: Boolean = false,
    val _raw: Any? = null
)

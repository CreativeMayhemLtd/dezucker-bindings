package coffee.outof.dezucker.plugins

data class PluginMetadata(
    val name: String,
    val slug: String,
    val description: String? = null,
    val version: String? = null
)

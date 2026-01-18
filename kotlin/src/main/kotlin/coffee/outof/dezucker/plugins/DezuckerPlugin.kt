package coffee.outof.dezucker.plugins

interface DezuckerPlugin<T, C> {
    val metadata: PluginMetadata
    val transformer: DataTransformer<T, C>
    val sink: OutputSink<T, C>
    val defaultConfig: C? get() = null
}

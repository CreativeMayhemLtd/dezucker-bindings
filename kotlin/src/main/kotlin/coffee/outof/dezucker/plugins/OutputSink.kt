package coffee.outof.dezucker.plugins

interface OutputSink<T, C> {
    suspend fun persist(data: T, context: ExportContext, config: C? = null)
    val databaseCollectionKey: String? get() = null
}

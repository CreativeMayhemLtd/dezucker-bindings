package coffee.outof.dezucker.storage

interface InternalStorage {
    suspend fun init()
    suspend fun push(key: String, data: Any)
    suspend fun update(key: String, updateFn: (collection: MutableList<Any>) -> Unit)
    suspend fun dataFor(key: String): List<Any>
    val collectionKeys: Map<String, String>
}

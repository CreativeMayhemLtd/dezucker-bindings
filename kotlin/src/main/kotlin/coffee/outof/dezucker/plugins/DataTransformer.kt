package coffee.outof.dezucker.plugins

import coffee.outof.dezucker.types.FormattedPost

interface DataTransformer<T, C> {
    suspend fun transform(post: FormattedPost, context: ExportContext, config: C? = null): T
}

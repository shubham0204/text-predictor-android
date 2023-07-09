package com.shubham0204.text_predictor

import android.content.Context
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import java.io.File
import java.io.FileOutputStream

/**
Next-word prediction system which internally uses a prebuilt Markov-model
to predict the next-word given the current word.
Markov models could modelled as directed-graphs where nodes are the words from the training
corpus (huge text dataset) and edges represent dependencies. If a directed-edge goes from
word A to word B, then its weight is the frequency of word B, given that word A precedes it in the corpus.
Once the graph is built, the top-3 words that precede any word 'w' are stored in a hashmap, where the
key is the word 'w' and the value is `List<String>` that contains the top-3 words succeeding 'w'. These words
are displayed to the user as suggestions.
For more implementation details, see the `rust` branch.

The implementation of the graph is built in Rust, and is interfaced with this class through JNI.
The .so libraries for various targets could be found in `text-predictor/src/main/jniLibs`. The JNI
methods would be found on the `rust` branch in `src/lib.rs` script.

Example:

```
val nextWordPredictor = NextWordPredictor( context )
nextWordPredictor.load()

val input = "why"
nextWordPredictor.predict(
    input ,
    onResult = { results ->

    } ,
    onError = { error ->
        println( error.message
    }
)
```

 **/
class NextWordPredictor( private val context : Context ) {

    // A pointer to an instance of `predictor` which is used in the Rust
    // code to call methods and allocate objects at run-time
    // It is set by the `createNativeInstance` which, in Rust, creates a new object
    // and returns a pointer to it.
    private var instancePtr = 0L

    // The asset required by the native code, which is copied from the assets directory of the
    // library to the app's internal (private) storage.
    private val assetName = "corpus.txt"

    companion object {
        init {
            // Load the .so libraries from JNI-libs
            System.loadLibrary( "predictor" )
        }
    }

    init {
        load()
    }

    /**
     * Predicts the next words, given the current word. If `word="how"`, then this method will return
     * `[ are , you , should ]` as a `List<String>` in the `onResult` callback.
     * @param word The current word entered by the user, using which following words will be suggested.
     * @param onResult The callback which delivers a `List<String>` containing the predicted words.
     * @param onError Provides a [TextPredictorError] when the operation fails
     */
    fun predict( word: String ,
                 onResult: ((List<String>) -> Unit) ,
                 onError: ((TextPredictorError) -> Unit)
    ) = runBlocking( Dispatchers.Default ) {
        val input = word.lowercase().trim()
        if( !InputValidators.checkIfWord( input ) ) {
            if( !InputValidators.checkIfContainsNumber( input ) ) {
                val output = predictToken(instancePtr, word.lowercase() )
                onResult( output.split("\n")
                    .map { it.trim() }
                    .filter { it.isNotEmpty() }
                    .toList() )
            }
            else {
                onError( TextPredictorError.ERROR_CONTAINS_NUMBER )
            }
        }
        else {
            onError( TextPredictorError.ERROR_NO_WORD )
        }
    }

    /**
     * Release the native resources acquired by the object.
     * Internally, it deallocates the memory which was acquired with the [createNativeInstance] method.
     */
    fun close() {
        deleteNativeInstance( instancePtr )
    }

    private fun load() = runBlocking( Dispatchers.IO ) {
        val appDirCorpusPath = copyFromAssetsToAppDir( assetName , assetName )
        instancePtr = createNativeInstance( appDirCorpusPath )
    }

    private fun copyFromAssetsToAppDir( assetsFilename: String , appDirFilename: String ) : String {
        val inputStream = context.assets.open( assetsFilename )
        val bufferSize = inputStream.available()
        val buffer = ByteArray( bufferSize )
        inputStream.read( buffer , 0 , bufferSize )
        inputStream.close()

        val file = File( context.filesDir , appDirFilename )
        val outputStream = FileOutputStream( file )
        outputStream.write( buffer )
        outputStream.close()

        return file.absolutePath
    }

     private external fun createNativeInstance(filepath : String ): Long
     private external fun deleteNativeInstance(instancePtr : Long )
     private external fun predictToken(instancePtr: Long, token : String ) : String

}
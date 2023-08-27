package com.shubham0204.text_predictor

import android.content.Context
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import java.io.File
import java.io.FileOutputStream

/**
 * Auto-completes a given word, for instance, `hel` could be completed as `[ hell , hello ]`.
 * It contains JNI methods that interface with .so libraries contained in `src/main/jniLibs`. Internally,
 * a [trie / prefix tree](https://en.wikipedia.org/wiki/Trie) is used to provide suggestions for words.
 */
internal class WordAutoCompletion( private val context: Context ){

    // A pointer to an instance of `predictor` which is used in the Rust
    // code to call methods and allocate objects at run-time
    // It is set by the `createNativeInstance` which, in Rust, creates a new object
    // and returns a pointer to it.
    private var instancePtr = 0L

    // The asset required by the native code, which is copied from the assets directory of the
    // library to the app's internal (private) storage.
    private val assetName = "vocab.txt"

    // Load .so libraries
    companion object {
        init {
            System.loadLibrary( "predictor" )
        }
    }

    init {
        load()
    }

    /**
     * Complete the given `phrase`, where `phrase` is the incomplete word. For instance, if the given input
     * is `hel`, the predictions would be [ hell , hello , help , helped , helps ].
     * @param phrase The input word, which has to be completed
     * @param onResult The callback which delivers the possible completions for the given `phrase` as a `List<String>`
     */
    fun predict( phrase: String ,
                 onResult: ((List<String>) -> Unit) )
    = runBlocking( Dispatchers.Default ) {
        var input = phrase.lowercase().trim()
        if( InputValidators.checkIfWord( input ) ) {
            input = InputValidators.stripNonAlphabet( input )
            if( input.isNotEmpty() ) {
                val output = predictWord( instancePtr , input )
                onResult( output.split(" ")
                    .map { it.trim() }
                    .filter { it.isNotEmpty() }
                    .toList() )
            }
            else {
                onResult( listOf() )
            }
        }
        else {
            onResult( listOf() )
        }
    }

    /**
     * Release the native resources acquired by the object.
     * Internally, it deallocates the memory which was acquired with the [createNativeInstance] method.
     */
    fun close() {
        deleteNativeInstance( instancePtr )
    }

    // Copies assets to app's storage, and creates a native object,
    // assigning its address to `instancePtr`
    private fun load() = runBlocking( Dispatchers.IO ) {
        val vocabFilePath = copyFromAssetsToAppDir( assetName , assetName )
        instancePtr = createNativeInstance( vocabFilePath )
    }

    // Copies a file from the library's assetFolder to the app's internal storage (private storage)
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

    // JNI methods whose implementation is stored in the .so files
    // Note: These methods should not be modified, nor the package name of this
    //       class should change. JVM would not be able to find the implementation
    //       for these methods, if their signature is changed.
    private external fun createNativeInstance(filepath : String ): Long
    private external fun deleteNativeInstance(instancePtr : Long )
    private external fun predictWord(instancePtr: Long, phrase : String ) : String

}
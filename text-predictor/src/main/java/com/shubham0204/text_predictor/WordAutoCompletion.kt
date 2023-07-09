package com.shubham0204.text_predictor

import android.content.Context
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import java.io.File
import java.io.FileOutputStream

class WordAutoCompletion( private val context: Context ){

    // A pointer to an instance of `predictor` which is used in the Rust
    // code to call methods and allocate objects at run-time
    // It is set by the `createNativeInstance` which, in Rust, creates a new object
    // and returns a pointer to it.
    private var instancePtr = 0L

    // The asset required by the native code, which is copied from the assets directory of the
    // library to the app's internal (private) storage.
    private val assetName = "vocab.txt"

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
     * @param onResult The callback which delivers the possible completions for the given `phrase` as a `List<String>`.
     * @param onError The callback which delivers a `TextPredictorError` if the operation fails
     */
    fun predict( phrase: String ,
                 onResult: ((List<String>) -> Unit) ,
                 onError: ((TextPredictorError) -> Unit) )
    = runBlocking( Dispatchers.Default ) {
        val input = phrase.lowercase().trim()
        if( !InputValidators.checkIfWord( input ) ) {
            if( !InputValidators.checkIfContainsNumber( input ) ) {
                val output = predictWord( instancePtr , input )
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
        val vocabFilePath = copyFromAssetsToAppDir( assetName , assetName )
        instancePtr = createNativeInstance( vocabFilePath )
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
    private external fun predictWord(instancePtr: Long, phrase : String ) : String

}
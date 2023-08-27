package com.shubham0204.text_predictor

import android.content.Context
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking

/**
* Provides text predictions when used along a text-based input widget. It combines next-word prediction with
* word autocompletion and returns results through [resultCallback].
* This has to used with a `onTextChange` callback of a text input widget.
*
* ### Example
*
* ```
* val textPredictor = TextPredictor( this , results )
*
* var inputString by remember{ mutableStateOf("") }
* TextField(
*     modifier = modifier,
*     value = inputString,,
*     onValueChange = {
*         textPredictor.stream(it)
*         inputString = it
*     },
*     singleLine = true,
* )
*
* private val results: ((List<String>) -> Unit) = { suggestions ->
*     // Handle UI changes that display `suggestions`
* }
* ```
*
* @param context The context of the caller `Activity`
* @param resultCallback The function that would provide suggestions as a `List<String>`
 */
class TextPredictor(
    private val context : Context ,
    private val resultCallback: ((List<String>) -> Unit)
    ) {

    // Instantiate components
    private val nextWordPredictor = NextWordPredictor( context )
    private val wordAutoCompletion = WordAutoCompletion( context )

    /**
     * Sends the incomplete [subString] and produce next-word predictions
     * and word-autocompletion(s) supplied through [resultCallback].
     * @param subString The incomplete string entered by the user.
     */
    fun stream( subString: String ) {
        if(subString.isNotBlank()) {
            if( subString.last() != ' ' ) {
                val token = if ( subString.contains( " " ) ) {
                    subString.split( " " ).last() }
                else { subString }
                wordAutoCompletion.predict(
                    token ,
                    onResult = {
                        resultCallback( it )
                    }
                )
            }
            else {
                val parts = subString.split( " " )
                val token = parts[ parts.size - 2 ]
                nextWordPredictor.predict(
                    token ,
                    onResult = {
                        resultCallback( it )
                    }
                )
            }
        }
        else {
            resultCallback( listOf() )
        }
    }

}
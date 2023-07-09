package com.shubham0204.text_predictor

/**
 * Common errors that are raised when an invalid input is received in
 * [WordAutoCompletion] and [WordAutoCompletion]
 */
enum class TextPredictorError( val message: String ) {
    ERROR_NO_WORD( "The input provided has more than one word, which is not allowed. " +
            "Check if the input contains a space." ),
    ERROR_CONTAINS_NUMBER( "The input provided contains number(s), which are not allowed." )
}
package com.shubham0204.text_predictor

/**
 Validates input using regular expressions
 It is used to report errors in the input provided to [WordAutoCompletion.predict] or [NextWordPredictor.predict].
**/
internal class InputValidators {

    companion object {

        private val wordRegex = "(?:[A-Za-z]+|\\d+)\\s+(?:[A-Za-z]+|\\d+)".toRegex()
        private val nonAlphabetRegex = "[^a-zA-Z ]+".toRegex()

        /**
         * Checks if the given [input] contains only a single word. This check is necessary,
         * as both [NextWordPredictor] and [WordAutoCompletion] operate only on words.
         * @param input The input string which has to be validated
         * @return Whether [input] contains only a single word
         */
        fun checkIfWord( input: String ) : Boolean {
            return !wordRegex.containsMatchIn( input.trim() )
        }

        fun stripNonAlphabet( input : String ) : String {
            return input.replace( nonAlphabetRegex , "" ) .trim()
        }

    }

}
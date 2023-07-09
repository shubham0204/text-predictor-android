package com.shubham0204.text_predictor

import java.util.regex.Pattern

/**
 Validates input using regular expressions
 It is used to report errors in the input provided to [WordAutoCompletion.predict] or [NextWordPredictor.predict].
**/
class InputValidators {

    companion object {

        private val checkWordRegex = Pattern.compile( "(?:[A-Za-z]+|\\d+)\\s+(?:[A-Za-z]+|\\d+)" )
        private val checkIfNumber = Pattern.compile( "" )

        /**
         *
         * @param input The input string which has to be validated
         * @return Whether [input] contains only a single word
         */
        fun checkIfWord( input: String ) : Boolean {
            return !checkWordRegex.matcher( input.trim() ).find()
        }

        /**
         * @param
         */
        fun checkIfContainsNumber( input: String ) : Boolean {
            return !checkIfNumber.matcher( input.trim() ).find()
        }

    }

}
package com.shubham0204.text_predictor

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.material3.TextFieldDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.shubham0204.text_predictor.ui.theme.TextpredictorandroidTheme
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {

    private lateinit var wordAutoCompletion : WordAutoCompletion
    private lateinit var nextWordPredictor : NextWordPredictor
    private val suggestionsListState = mutableStateOf<List<String>>( ArrayList() )

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        wordAutoCompletion = WordAutoCompletion( this )
        nextWordPredictor = NextWordPredictor( this )

        setContent {
            TextpredictorandroidTheme {
                ActivityUI()
            }
        }
    }

    @Preview
    @Composable
    fun ActivityUIPreview() {
        TextpredictorandroidTheme {
            ActivityUI()
        }
    }

    @Composable
    fun ActivityUI() {
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background
        ) {
            Column {
                TextInput(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding( vertical=24.dp , horizontal=24.dp ),
                    label = "Enter text here..." ,
                    onValueChange = textChangeListener )
                Suggestions(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding( horizontal=24.dp ) )
            }
        }
    }

    @OptIn(ExperimentalMaterial3Api::class)
    @Composable
    fun TextInput(
        label : String,
        onValueChange : ( (String) -> Unit ),
        modifier: Modifier = Modifier
    ) {
        var inputString by remember{ mutableStateOf("") }
        TextField(
            modifier = modifier,
            value = inputString,
            shape = RoundedCornerShape( 16.dp ) ,
            onValueChange = {
                onValueChange( it )
                inputString = it
            },
            textStyle = MaterialTheme.typography.headlineSmall ,
            colors = TextFieldDefaults.textFieldColors(
                focusedLabelColor = MaterialTheme.colorScheme.primary,
                unfocusedLabelColor = MaterialTheme.colorScheme.primary,
                textColor = Color.Black ,
                containerColor = MaterialTheme.colorScheme.secondaryContainer ,
                focusedIndicatorColor = Color.Transparent,
                unfocusedIndicatorColor = Color.Transparent,
                disabledIndicatorColor = Color.Transparent
            ),
            label = { Text(text = label , style = MaterialTheme.typography.labelSmall ) },
            singleLine = true,
        )
    }

    @Composable
    fun Suggestions( modifier: Modifier ) {
        val list by remember{ suggestionsListState }
        SuggestionsList( modifier, list )
    }


    @OptIn(ExperimentalMaterial3Api::class)
    @Composable
    fun SuggestionsList(
        modifier: Modifier ,
        predictions: List<String>
    ) {
        LazyRow( modifier ){
            items( predictions ) {
                Surface( modifier = Modifier.background(Color.White , shape= RoundedCornerShape(16.dp)) ,
                    onClick = {
                        // TODO: Apply suggestion here
                    }
                ) {
                    Text(
                        modifier = Modifier.padding(vertical = 16.dp, horizontal = 16.dp),
                        text = it ,
                        color = Color.Black,
                    )
                }

            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        nextWordPredictor.close()
        wordAutoCompletion.close()
    }


    private val textChangeListener : ((String) -> Unit) = { it ->
        if(it.isNotBlank()) {
            if( it.last() != ' ' ) {
                val token = if ( it.contains( " " ) ) {
                    it.split( " " ).last() }
                else { it }
                wordAutoCompletion.predict( token.lowercase() ) {
                    suggestionsListState.value = it
                }
            }
            else {
                val parts = it.split( " " )
                val token = parts[ parts.size - 2 ]
                nextWordPredictor.predict( token.lowercase() ) { it ->
                    suggestionsListState.value = it
                }
            }
        }
        else {
            suggestionsListState.value = ArrayList()
        }

    }


}







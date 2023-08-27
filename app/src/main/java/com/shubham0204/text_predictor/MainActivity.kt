package com.shubham0204.text_predictor

import android.os.Bundle
import android.widget.EditText
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
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
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.core.widget.addTextChangedListener
import com.shubham0204.text_predictor.ui.theme.TextpredictorandroidTheme
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.launch

class MainActivity : ComponentActivity() {

    private lateinit var textPredictor: TextPredictor
    private val suggestionsListState = mutableStateOf<List<String>>( ArrayList() )

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        textPredictor = TextPredictor( this , resultCallback )

        setContent {
            TextpredictorandroidTheme {
                ActivityUI()
            }
        }
    }

    private val resultCallback : ((List<String>) -> Unit) = {
        suggestionsListState.value = it
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
                        .padding(vertical = 24.dp, horizontal = 24.dp),
                    label = "Enter text here..." ,
                    onValueChange = textChangeListener )
                Suggestions(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 24.dp) )
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
        val list by suggestionsListState
        SuggestionsList( modifier, list )
    }


    @Composable
    fun SuggestionsList(
        modifier: Modifier ,
        predictions: List<String>
    ) {
        LazyRow( modifier , contentPadding = PaddingValues( 16.dp ) ){
            items( predictions ) {
                Box {
                    Text(
                        modifier = Modifier.background( Color.Blue , RoundedCornerShape(8.dp) ) ,
                        text = it ,
                        color = Color.White,
                    )
                }
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
    }


    private val textChangeListener : ((String) -> Unit) = { it ->
        textPredictor.stream( it )
    }


}







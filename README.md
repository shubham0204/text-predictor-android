# Text Prediction in Android - With Rust 🚀

Perform next-word prediction with text-autocompletion, all in Android. The library uses Rust internally, to bring 
text-suggestions within a few-milliseconds. See the [rust]() branch for more details.

* [Documentation](https://shubham0204.github.io/text-predictor-android/android/)

## Installation

1. Download `text-predictor.aar` from the latest release. (See [Releases](https://github.com/shubham0204/text-predictor-android/releases))
2. Move the AAR to `app/libs`.
3. In module-level `build.gradle`, add the dependency,

```groovy
dependencies {
    ...
    implementation files('libs/text-predictor.aar')
    ...
}
```

## Usage

To generate suggestions while text is being entered in a text-widget (could be a `EditText` or `TextField`), we use 
the `stream` method available in the `TextPredictor` class. The `stream` method must be called inside on a `onTextChanged` or a 
similar callback that provides a string entered by the user.

For a Compose `TextField`,

```kotlin
val textPredictor = TextPredictor( this , results )

var inputString by remember{ mutableStateOf("") }
TextField(
    modifier = modifier,
    value = inputString,,
    onValueChange = {
        textPredictor.stream(it)
        inputString = it
    },
    singleLine = true,
)

private val results: ((List<String>) -> Unit) = { suggestions ->
    // Handle UI changes that display `suggestions`
}
```

For a `View`-based `EditText`, 

```kotlin
val textPredictor = TextPredictor( this , results )

editText.addTextChangedListener { it ->
    textPredictor.stream( it.toString() )
}

private val results: ((List<String>) -> Unit) = { suggestions ->
    // Handle UI changes that display `suggestions`
} 
```

+++
title = "Modern Android Development with Kotlin"
date = "2023-04-05"
tags = ["kotlin", "android", "jetpack-compose"]
excerpt = "Explore modern Android development using Kotlin, Jetpack Compose, and coroutines. Build a note-taking app with Material 3 design principles."
+++

Kotlin has been Google's preferred language for Android development since 2019. Its concise syntax, null safety, and seamless Java interop make it ideal for building robust mobile applications.

## Why Kotlin Over Java?

Kotlin eliminates many pain points of Java development:

```kotlin
// Java-style
public class User {
    private String name;
    public User(String name) { this.name = name; }
    public String getName() { return name; }
}

// Kotlin
data class User(val name: String)
```

| Feature | Java | Kotlin |
|---------|------|--------|
| Null safety | Requires `Optional` | Built-in with `?` |
| Data classes | Boilerplate | One-liner |
| Extension functions | No | Yes |
| Coroutines | External lib | Native support |

## Jetpack Compose UI

Compose replaces XML layouts with declarative UI written in Kotlin:

```kotlin
@Composable
fun NoteCard(note: Note, onDelete: (Note) -> Unit) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(8.dp),
        elevation = CardDefaults.cardElevation(defaultElevation = 2.dp)
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Column(modifier = Modifier.weight(1f)) {
                Text(text = note.title, style = MaterialTheme.typography.titleMedium)
                Text(text = note.content, style = MaterialTheme.typography.bodyMedium)
            }
            IconButton(onClick = { onDelete(note) }) {
                Icon(Icons.Default.Delete, contentDescription = "Delete")
            }
        }
    }
}
```

## State Management with ViewModel

Use `ViewModel` to survive configuration changes and `StateFlow` to expose UI state:

```kotlin
class NoteViewModel : ViewModel() {

    private val _notes = MutableStateFlow<List<Note>>(emptyList())
    val notes: StateFlow<List<Note>> = _notes.asStateFlow()

    fun addNote(title: String, content: String) {
        _notes.update { it + Note(title = title, content = content) }
    }

    fun deleteNote(note: Note) {
        _notes.update { it - note }
    }
}
```

## Coroutines for Async Work

Coroutines provide structured concurrency without callback hell:

```kotlin
viewModelScope.launch(Dispatchers.IO) {
    val result = repository.fetchNotes()
    withContext(Dispatchers.Main) {
        _notes.value = result
    }
}
```

## Dependency Injection with Hilt

Hilt simplifies DI setup across your Android app:

```kotlin
@HiltAndroidApp
class NoteApplication : Application()

@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    @Inject lateinit var viewModelFactory: NoteViewModel.Factory
}
```

## Navigation

The Navigation Compose library handles screen transitions:

```kotlin
NavHost(navController, startDestination = "notes") {
    composable("notes") {
        NoteListScreen(
            onNoteClick = { id -> navController.navigate("note/$id") }
        )
    }
    composable("note/{id}") { backStackEntry ->
        NoteDetailScreen(noteId = backStackEntry.arguments?.getString("id"))
    }
}
```

Modern Android development with Kotlin, Compose, and coroutines is productive and enjoyable. The tooling continues to mature, and the ecosystem grows richer every quarter.

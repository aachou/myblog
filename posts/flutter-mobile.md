+++
title = "Flutter Mobile"
date = "2022-10-01"
tags = ["flutter", "dart", "mobile-development"]
excerpt = "A deep dive into Flutter development 鈥?from widgets and state management to platform channels and performance optimization."
+++

Flutter is Google's UI toolkit for building natively compiled applications for mobile, web, and desktop from a single codebase. It uses the Dart language and a reactive widget model.

## Widgets Are Everything

In Flutter, everything is a widget 鈥?from structural elements like buttons to stylistic elements like padding:

```dart
import 'package:flutter/material.dart';

void main() => runApp(const MyApp());

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      home: Scaffold(
        appBar: AppBar(title: const Text('Hello Flutter')),
        body: const Center(
          child: Text('Hello, World!'),
        ),
      ),
    );
  }
}
```

## State Management

Managing state is a central concern in Flutter apps. Popular approaches include:

| Approach | Package | Use Case |
|----------|---------|----------|
| setState | Built-in | Simple local state |
| Provider | provider | Medium complexity |
| Bloc | bloc | Complex apps |
| Riverpod | riverpod | Testable architecture |

### Example with Provider

```dart
class Counter with ChangeNotifier {
  int _count = 0;
  int get count => _count;

  void increment() {
    _count++;
    notifyListeners();
  }
}

// In widget
final counter = context.watch<Counter>();
Text('${counter.count}');
```

## Platform Channels

Access native APIs through platform channels:

```dart
import 'package:flutter/services.dart';

static const platform = MethodChannel('samples.flutter.dev/battery');

Future<String> getBatteryLevel() async {
  try {
    final result = await platform.invokeMethod<int>('getBatteryLevel');
    return 'Battery level: $result%';
  } on PlatformException catch (e) {
    return "Failed: '${e.message}'";
  }
}
```

## Layouts

Flutter uses a flexible layout system based on rows, columns, and stacks:

```dart
Column(
  mainAxisAlignment: MainAxisAlignment.center,
  children: [
    const Text('Welcome'),
    Row(
      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
      children: [
        Icon(Icons.star, color: Colors.yellow),
        Icon(Icons.star, color: Colors.yellow),
        Icon(Icons.star, color: Colors.yellow),
      ],
    ),
    ElevatedButton(
      onPressed: () {},
      child: const Text('Press me'),
    ),
  ],
)
```

## Performance Tips

- Use `const` constructors wherever possible
- Avoid rebuilding widgets unnecessarily
- Use `ListView.builder` for long lists
- Profile with the Flutter DevTools

```dart
// Good 鈥?lazily builds items
ListView.builder(
  itemCount: items.length,
  itemBuilder: (context, index) => ListTile(title: Text(items[index])),
)
```

## Testing

```dart
testWidgets('Counter increments', (tester) async {
  await tester.pumpWidget(const MyApp());
  await tester.tap(find.byIcon(Icons.add));
  await tester.pump();
  expect(find.text('1'), findsOneWidget);
});
```

Flutter's hot reload and expressive widget system make it a joy to build cross-platform apps.

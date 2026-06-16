+++
title = "TensorFlow Basics: Building Your First Neural Network"
date = "2025-01-28"
tags = ["machine-learning", "tensorflow", "python"]
excerpt = "Get started with TensorFlow by building, training, and evaluating a neural network for image classification on the Fashion MNIST dataset."
+++

TensorFlow is Google's open-source machine learning framework. In this guide, you will build a neural network that classifies clothing images.

## Installation

```bash
pip install tensorflow
```

Verify the installation:

```python
import tensorflow as tf
print(tf.__version__)
```

## Loading the Data

Fashion MNIST contains 70,000 grayscale images of clothing:

```python
fashion_mnist = tf.keras.datasets.fashion_mnist
(x_train, y_train), (x_test, y_test) = fashion_mnist.load_data()

# Normalize pixel values to [0, 1]
x_train = x_train / 255.0
x_test = x_test / 255.0
```

## Building the Model

```python
model = tf.keras.Sequential([
    tf.keras.layers.Flatten(input_shape=(28, 28)),
    tf.keras.layers.Dense(128, activation='relu'),
    tf.keras.layers.Dropout(0.2),
    tf.keras.layers.Dense(10, activation='softmax')
])
```

## Compiling the Model

```python
model.compile(
    optimizer='adam',
    loss='sparse_categorical_crossentropy',
    metrics=['accuracy']
)
```

## Training

```python
history = model.fit(
    x_train, y_train,
    epochs=10,
    validation_split=0.2,
    batch_size=32
)
```

## Evaluating

```python
test_loss, test_acc = model.evaluate(x_test, y_test)
print(f"Test accuracy: {test_acc:.4f}")
```

## Making Predictions

```python
predictions = model.predict(x_test)
predicted_class = tf.argmax(predictions[0]).numpy()
```

## Model Architecture Comparison

| Layer Type | Purpose | Parameters |
|------------|---------|------------|
| Flatten | Convert 2D to 1D | 0 |
| Dense (128) | Learn patterns | 100,480 |
| Dropout (0.2) | Prevent overfitting | 0 |
| Dense (10) | Output classes | 1,290 |

## Callbacks for Better Training

```python
checkpoint = tf.keras.callbacks.ModelCheckpoint(
    'model.weights.h5', save_best_only=True
)
early_stop = tf.keras.callbacks.EarlyStopping(
    patience=3, restore_best_weights=True
)

model.fit(x_train, y_train, epochs=50,
          callbacks=[checkpoint, early_stop])
```

## Saving and Loading

```python
model.save('fashion_classifier.keras')
loaded = tf.keras.models.load_model('fashion_classifier.keras')
```

## Common Pitfalls

1. Not normalizing inputs causes slow or failed convergence
2. Too many layers without regularization leads to overfitting
3. Using the wrong loss function for your problem
4. Training for too few or too many epochs
5. Ignoring the validation set during hyperparameter tuning

## Next Steps

Experiment with convolutional layers for better accuracy:

```python
model = tf.keras.Sequential([
    tf.keras.layers.Conv2D(32, (3, 3), activation='relu', input_shape=(28, 28, 1)),
    tf.keras.layers.MaxPooling2D((2, 2)),
    tf.keras.layers.Flatten(),
    tf.keras.layers.Dense(64, activation='relu'),
    tf.keras.layers.Dense(10, activation='softmax')
])
```

TensorFlow's high-level Keras API makes deep learning accessible to anyone with basic Python skills.

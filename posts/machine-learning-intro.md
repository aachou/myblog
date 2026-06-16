+++
title = "Machine Learning Intro"
date = "2022-02-20"
tags = ["machine-learning", "python", "data-science"]
excerpt = "A gentle introduction to machine learning concepts, types of learning, and a practical scikit-learn example to get you started."
+++

Machine learning is a subset of artificial intelligence that enables systems to learn and improve from experience without being explicitly programmed. It is everywhere 鈥?from spam filters to recommendation engines.

## Types of Machine Learning

There are three broad categories:

### Supervised Learning

The model is trained on labeled data. Each training example has an input and a known output. Common algorithms include linear regression, decision trees, and support vector machines.

```python
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import train_test_split

X_train, X_test, y_train, y_test = train_test_split(
    features, labels, test_size=0.2
)

clf = RandomForestClassifier(n_estimators=100)
clf.fit(X_train, y_train)
accuracy = clf.score(X_test, y_test)
print(f"Accuracy: {accuracy:.2f}")
```

### Unsupervised Learning

The model finds patterns in unlabeled data. Clustering and dimensionality reduction are the primary tasks. K-Means and PCA are popular choices.

### Reinforcement Learning

An agent learns by interacting with an environment and receiving rewards or penalties. This is how AlphaGo and self-driving cars are trained.

## The Machine Learning Pipeline

1. **Data collection** 鈥?gather raw data from various sources
2. **Data cleaning** 鈥?handle missing values, outliers, and inconsistencies
3. **Feature engineering** 鈥?create meaningful input variables
4. **Model selection** 鈥?choose an appropriate algorithm
5. **Training** 鈥?fit the model on training data
6. **Evaluation** 鈥?measure performance on unseen data
7. **Deployment** 鈥?serve the model in production

## Overfitting and Underfitting

Overfitting occurs when a model memorises the training data but fails to generalise. Underfitting happens when the model is too simple to capture patterns. Regularisation and cross-validation help strike a balance.

| Problem | Training Accuracy | Test Accuracy |
|---------|-----------------|---------------|
| Overfitting | High (~98%) | Low (~65%) |
| Underfitting | Low (~60%) | Low (~58%) |
| Good fit | High (~95%) | High (~92%) |

## Practical Advice

Start with simple models. A linear model often beats a neural network if the data is clean and the relationship is linear. Invest time in understanding your data 鈥?visualise distributions, correlations, and drifts.

## Tools of the Trade

- **scikit-learn** 鈥?general-purpose ML
- **TensorFlow / PyTorch** 鈥?deep learning
- **Pandas** 鈥?data manipulation
- **Matplotlib / Seaborn** 鈥?visualisation
- **Jupyter** 鈥?interactive notebooks

Machine learning is a powerful tool, but it requires disciplined experimentation and rigorous evaluation.

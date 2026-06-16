+++
title = "Data Analysis with Pandas: From Raw Data to Insights"
date = "2023-12-20"
tags = ["python", "pandas", "data-science"]
excerpt = "Pandas is the Swiss army knife of data analysis in Python. This guide covers DataFrames, group operations, time series, and visualization integration for real-world datasets."
+++

Pandas is the foundational library for data manipulation and analysis in Python. Built on top of NumPy, it provides the DataFrame — a two-dimensional, labeled data structure that makes working with structured data intuitive.

## Loading Data

Read from CSV, Excel, JSON, SQL, or Parquet with a single function call:

```python
import pandas as pd

# Load CSV with type inference
df = pd.read_csv("sales_2023.csv", parse_dates=["order_date"])

# Load Excel, specifying sheet
df_excel = pd.read_excel("inventory.xlsx", sheet_name="Q1")

# Load JSON with nested structure
df_json = pd.read_json("events.json", orient="records")
```

| Method | File Format |
|--------|-------------|
| `read_csv()` | CSV / TSV |
| `read_excel()` | Excel (.xlsx, .xls) |
| `read_json()` | JSON |
| `read_sql()` | SQL queries |
| `read_parquet()` | Apache Parquet |

## Data Inspection

First steps after loading data:

```python
# Quick overview
df.head(10)
df.info()
df.describe(include="all")

# Check for missing values
df.isnull().sum()

# Unique value counts
df["category"].value_counts()
```

Output of `df.info()`:

```
<class 'pandas.core.frame.DataFrame'>
RangeIndex: 50000 entries, 0 to 49999
Data columns (total 12 columns):
 #   Column       Non-Null Count  Dtype
---  ------       --------------  -----
 0   order_id     50000 non-null  int64
 1   order_date   50000 non-null  datetime64[ns]
 2   customer_id  49780 non-null  float64
 3   amount       50000 non-null  float64
 4   category     50000 non-null  object
 5   region       49500 non-null  object
dtypes: datetime64[ns](1), float64(2), int64(1), object(2)
```

## Cleaning and Transformation

Real-world data is messy. Pandas provides tools to clean it:

```python
# Fill missing values
df["customer_id"] = df["customer_id"].fillna(0).astype(int)
df["region"] = df["region"].fillna("Unknown")

# Remove duplicates
df = df.drop_duplicates(subset=["order_id"])

# Create derived columns
df["revenue"] = df["quantity"] * df["unit_price"]
df["month"] = df["order_date"].dt.to_period("M")
df["is_weekend"] = df["order_date"].dt.dayofweek.isin([5, 6])

# Filter outliers
df = df[df["amount"].between(0, df["amount"].quantile(0.99))]
```

## Group Operations (Split-Apply-Combine)

The `groupby` pattern splits data, applies a function, and combines results:

```python
# Total revenue by category
revenue_by_category = (
    df.groupby("category")["revenue"]
      .agg(["sum", "mean", "count", "std"])
      .round(2)
      .sort_values("sum", ascending=False)
)

print(revenue_by_category)
```

```
              sum      mean  count       std
category
Electronics  94210.50  234.56   402  145.32
Clothing     58320.75   89.12   654   67.89
Food         27180.25   12.45  2184    8.23
Books        18940.00   45.67   415   34.56
```

## Time Series Analysis

Pandas excels at time-indexed data:

```python
# Set datetime index
df = df.set_index("order_date")

# Resample to monthly frequency
monthly_sales = df["revenue"].resample("ME").sum()

# Rolling 3-month average
monthly_sales.rolling(window=3).mean()

# Year-over-year comparison
monthly_sales.pct_change(periods=12) * 100
```

## Merging and Joining

Combine multiple DataFrames:

```python
customers = pd.read_csv("customers.csv")
orders = pd.read_csv("orders.csv")

# Inner join
df_merged = orders.merge(
    customers,
    on="customer_id",
    how="inner",
    suffixes=("_order", "_customer")
)

# Concatenate rows
q1 = pd.read_csv("sales_q1.csv")
q2 = pd.read_csv("sales_q2.csv")
yearly = pd.concat([q1, q2], ignore_index=True)
```

## Visualization Integration

Visualize data with the built-in `.plot()` method (powered by Matplotlib):

```python
import matplotlib.pyplot as plt

# Monthly revenue trend
monthly_sales.plot(
    kind="line",
    figsize=(12, 6),
    title="Monthly Revenue 2023",
    ylabel="Revenue ($)",
    grid=True
)
plt.tight_layout()
plt.show()

# Category distribution
revenue_by_category["sum"].plot(kind="bar", title="Revenue by Category")
```

## Exporting Results

```python
# Write to different formats
df_merged.to_csv("report.csv", index=False)
df_merged.to_excel("report.xlsx", sheet_name="Analysis")
df_merged.to_parquet("report.parquet")
```

Pandas is not just a library — it is a language for data work. Combine it with NumPy for numerical operations, Matplotlib and Seaborn for visualization, and Scikit-learn for machine learning to build a complete data analysis pipeline.

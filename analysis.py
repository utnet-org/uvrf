import pandas as pd

# Replace with your CSV file path
file_path = 'output.csv'
data = pd.read_csv(file_path)

# Assuming the CSV has columns 'address', 'power', 'count'
print(data.head())  # To verify that data is loaded correctly
print(data.columns)


import statsmodels.api as sm

# Simple linear regression: 'count' as a function of 'power'
X = data['power']
y = data['count']
X = sm.add_constant(X)  # Adds a constant term to the predictor
model = sm.OLS(y, X).fit()

print(model.summary())


import matplotlib.pyplot as plt
import seaborn as sns

# Scatter plot with a regression line
sns.regplot(x='power', y='count', data=data)

plt.xlabel('Power')
plt.ylabel('Selection Count')
plt.title('Regression Fit Curve')
plt.show()


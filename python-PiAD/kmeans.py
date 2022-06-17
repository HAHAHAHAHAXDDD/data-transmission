import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from sklearn. cluster import KMeans

data = pd.read_csv('autos.csv')
data = data.loc[:, ['horsepower', 'engine-size']]
X = data.values
k=3

def distp(X, C, e=1):
    dist = np.sqrt((C[0]-X[0])**2+(C[1]-X[1])**2)
    return dist

def kmeans(X, k):
  cluster = np.zeros(X.shape[0])
  C = data.sample(k).values
  while True:
    for i, row in enumerate(X):
        mn_dist = float('inf')
        for idx, center in enumerate(C):
            d = distp(row, center)
            if mn_dist > d:
               mn_dist = d
               cluster[i] = idx
    new_C = pd.DataFrame(X).groupby(by=cluster).mean().values
    if np.count_nonzero(C-new_C) == 0:
        return C, cluster
    else:
        C = new_C

[C, CX] = kmeans(X, k)
plt.scatter(X[:, 0], X[:, 1], c=CX)
plt.scatter(C[:,0], C[:,1], s=100, color='red', marker='x')
plt.show()
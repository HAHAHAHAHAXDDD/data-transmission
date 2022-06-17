from dataclasses import dataclass
from json import load
from tkinter import N, Y
from turtle import distance
import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import scipy as sp
from scipy.misc import face
from sklearn.datasets import load_boston, load_iris, make_classification, make_regression
from math import sqrt
import seaborn as sns
from matplotlib import colors
from sklearn.decomposition import PCA
from sklearn.neighbors import KNeighborsClassifier
from sklearn.model_selection import LeaveOneOut
from sklearn.model_selection import cross_val_score
from sklearn.model_selection import KFold
from sklearn.neighbors import KNeighborsRegressor

class knn:
    def __init__(self, n_neighbors=1, useKDTree=False):
        self.n_neighbors = n_neighbors
    def fit(self, X, y):
        self.X_train = X
        self.y_train = y
    def predict(self, X):
        num_test = X.shape[0]
        num_train = self.X_train.shape[0]
        distances = np.zeros((num_test, num_train))
        for i in range(num_test):
            for j in range(num_train):
                distances[i,j]=np.sqrt(np.sum((X[i,:]-self.X_train[j,:])**2))
        dist_range = distances.shape[0]
        y_pred = np.zeros(dist_range)
        for i in range(dist_range):
            y_indices = np.argsort(distances[i,:])
            closest_neighbors = self.y_train[y_indices[: self.n_neighbors]].astype(int)
            y_pred[i] = np.argmax(np.bincount(closest_neighbors))
        return y_pred
    def score(self, X, y):
        predictions = self.predict(X)
        acc = np.sum(predictions == y)/len(y)
        return acc

X, y = make_classification(
    n_samples=100,
    n_features=2,
    n_informative=2,
    n_redundant=0,
    n_repeated=0,
    random_state=3
)

#dataset
clf = knn()
clf.fit(X, y)
print("Procentowa dokladnosc: ", clf.score(X,y))
x_min, x_max = X[:, 0].min() - 1, X[:, 0].max() + 1
y_min, y_max = X[:, 1].min() - 1, X[:, 1].max() + 1
xx, yy = np.meshgrid(np.arange(x_min, x_max, 0.1), np.arange(y_min, y_max, 0.1))
Z = clf.predict(np.c_[xx.ravel(), yy.ravel()])
Z = Z.reshape(xx.shape)
plt.figure(figsize=(8,6))
plt.contour(xx, yy, Z)
plt.scatter(X[:,0], X[:,1], c=y, cmap=plt.cm.get_cmap('gist_rainbow'), s=20)
plt.xlim(-3,3)
plt.ylim(-3,3)
plt.show()

#irysy
dataset = load_iris()
X = dataset.data
y = dataset.target

clf = knn()
clf.fit(X, y)
print("Procentowa dokladnosc: ", clf.score(X, y))
pca = PCA(n_components=2)
X_r = pca.fit_transform(X)
fig = plt.figure(figsize=(8, 6))
for i in [0, 1, 2]:
    plt.scatter(X_r[y == i, 0],X_r[y == i, 1], cmap=plt.cm.get_cmap('gist_rainbow'), s=20)

x_min, x_max = X_r[:, 0].min() - 1, X_r[:, 0].max() + 1
y_min, y_max = X_r[:, 1].min() - 1, X_r[:, 1].max() + 1
xx, yy = np.meshgrid(np.arange(x_min, x_max, 0.1), np.arange(y_min, y_max, 0.1))
clf.fit(X_r, y)
# inv_t = pca.inverse_transform(X_r)
Z = clf.predict(np.c_[xx.ravel(), yy.ravel()])
# Z = clf.predict(inv_t)
Z = Z.reshape(xx.shape)
plt.contour(xx, yy, Z)
plt.xlim(-3.2,3.2)
plt.ylim(-1,1)
plt.yticks([-1, -0.5, 0, 0.5, 1])
plt.show()

#leave one out
dataset = load_iris()
X = dataset.data
y = dataset.target

means = []
for i in range(1,40):
    cv = LeaveOneOut()
    model = KNeighborsClassifier(n_neighbors=i)
    scores = cross_val_score(model, X, y, scoring='neg_mean_squared_error', cv=cv, n_jobs=-1)
    means.append(np.sqrt(np.mean(np.absolute(scores))))
data = pd.DataFrame(means,columns=['mean error'])
print(data)

# regresja
class knnreg:
    def __init__(self, n_neighbors=1, useKDTree=False):
        self.n_neighbors = n_neighbors
    def fit(self, X, y):
        self.X_train = X
        self.y_train = y
    def predict(self, X):
        m = self.X_train.shape[0]
        n = X.shape[0]
        y_pred = []
        for i in range(n):
            distance = []
            for j in range(m):
                d = (np.sqrt(np.sum(np.square(X[i,:] - self.X_train[j,:]))))
                distance.append((d, self.y_train[j]))    
            distance = sorted(distance)
            neighbors = []
            for item in range(self.n_neighbors):
                neighbors.append(distance[item][1])
            y_pred.append(np.mean(neighbors))
        return y_pred
    def score(self, X, y):
        predictions = self.predict(X)
        differences = []
        for actual, predicted in zip(y, predictions):
            differences.append(actual - predicted)
        differences = np.array(differences)
        sqdiff = np.square(differences)
        return np.sqrt(np.mean(np.absolute(sqdiff)))

X, y = make_regression(
    n_samples=200,
    n_features=2, 
    n_informative=2,
    n_targets=2,
    random_state=3
)

reg = knnreg()
reg.fit(X,y)
print("Blad sredniokwadratowy: ", reg.score(X,y))
pred = reg.predict(X)
x_ax = np.linspace(np.min(X), np.max(X), len(pred))
test = np.linspace(np.min(X), np.max(X), len(y))
for i in [0,1]:
    plt.scatter(test, y[:,i], s=20, alpha=0.8, c='darkorange', label="oryginalne dane" if i == 0 else "")
plt.plot(x_ax, pred, lw=1.5, color='k', label='predicted dane')
plt.legend()
plt.show()

#boston
dataset = load_boston()
X = dataset.data
y = dataset.target
errors = []
for i in range(1,30):
    kf = KFold(n_splits=10)
    model = KNeighborsRegressor(n_neighbors=i)
    scores = cross_val_score(model, X, y, scoring='neg_mean_squared_error', cv=kf, n_jobs=-1)
    errors.append(np.sqrt(np.mean(np.absolute(scores))))
data = pd.DataFrame(errors,columns=['mean error'])
print(data)
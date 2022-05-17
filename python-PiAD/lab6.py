import numpy as np
import matplotlib.pyplot as plt
from sklearn import datasets
from sklearn.decomposition import PCA

# 1
# rng = np.random.RandomState(1)
# X = np.dot(rng.rand(2, 2), rng.randn(2, 200)).T
# print(X)
# pca = PCA(n_components=2)
# pca.fit(X)

# def draw_vector(v0, v1, ax=None):
#     ax = ax or plt.gca()
#     arrowprops=dict(arrowstyle='->',
#                     linewidth=2,
#                     shrinkA=0, shrinkB=0)
#     ax.annotate('', v1, v0, arrowprops=arrowprops)

# plt.scatter(X[:, 0], X[:, 1], c='darkgreen', alpha=0.5)
# for length, vector in zip(pca.explained_variance_, pca.components_):
#     v = vector * 3 * np.sqrt(length)
#     draw_vector(pca.mean_, pca.mean_ + v)

# pca = PCA(n_components=1)
# transformed = pca.fit_transform(X)
# otransf = pca.inverse_transform(transformed)
# plt.scatter(otransf[:, 0], otransf[:, 1], c='red', alpha=0.8)
# plt.axis('equal')
# plt.show()

# 2
# iris = datasets.load_iris()
# X = iris.data
# y = iris.target
# target_names = iris.target_names
# pca = PCA(n_components=2)
# X_r = pca.fit_transform(X)
# colors = ['red', 'green', 'blue']
# for color, i in zip(colors, [0, 1, 2]):
#     plt.scatter(X_r[y == i, 0], X_r[y == i, 1], color=color)
# plt.legend([target_names[0], target_names[1], target_names[2]])
# plt.show()

# 3
digits = datasets.load_digits()
X = digits.data
y = digits.target
# pca = PCA()
# pca = pca.fit(X)
# plt.plot(np.cumsum(pca.explained_variance_ratio_))
# plt.xlabel('numer składowej')
# plt.ylabel('skumulowana wariancja')
# plt.show()

pca = PCA(n_components=2)
transformed = pca.fit_transform(X)
plt.scatter(transformed[:,0], transformed[:,1], alpha=0.5, c=y, cmap=plt.cm.get_cmap('gist_rainbow'))
plt.xlabel('skladowa 1')
plt.ylabel('skladowa 2')
plt.colorbar()
plt.show()

# (e)

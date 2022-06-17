import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
from matplotlib import colors
from sklearn import datasets
from sklearn.cluster import AgglomerativeClustering, KMeans
from sklearn.decomposition import PCA
from scipy.spatial import ConvexHull
from scipy.stats import mode
from sklearn.metrics import jaccard_score
from scipy.cluster.hierarchy import dendrogram
from sklearn.mixture import GaussianMixture
from scipy.spatial import distance
from matplotlib.image import imread
from skimage.transform import rescale

# KLASTERYZACJA
def find_perm(clusters, Y_real, Y_pred):
    perm=[]
    for i in range(clusters):
        idx = Y_pred == i
        new_label = mode(Y_real[idx])[0][0]
        perm.append(new_label)
    return [perm[label] for label in Y_pred]

def plot_dendrogram(model, **kwargs):
    counts = np.zeros(model.children_.shape[0])
    n_samples = len(model.labels_)
    for i, merge in enumerate(model.children_):
        current_count = 0
        for child_idx in merge:
            if child_idx < n_samples:
                current_count += 1
            else:
                current_count += counts[child_idx - n_samples]
        counts[i] = current_count

    linkage_matrix = np.column_stack(
        [model.children_, model.distances_, counts]
    ).astype(float)
    dendrogram(linkage_matrix, **kwargs)

colors = colors.ListedColormap(['tab:orange', 'tab:blue', 'tab:green'])

iris = datasets.load_iris()
X = iris.data
y = iris.target

names = [
    'najblizsze_sasiedztwo',
    'srednie_polaczenia',
    'najdalsze_polaczenia',
    'ward',
    'KMeans',
    'GMM'
]

clusters = [
    AgglomerativeClustering(n_clusters=3, linkage='single', compute_distances=True),
    AgglomerativeClustering(n_clusters=3, linkage='average', compute_distances=True),
    AgglomerativeClustering(n_clusters=3, linkage='complete', compute_distances=True),
    AgglomerativeClustering(n_clusters=3, linkage='ward', compute_distances=True),
    KMeans(n_clusters=3),
    GaussianMixture(n_components=3)
]

for name, cluster in zip(names, clusters):
    y_pred = cluster.fit_predict(X)
    y_perm = find_perm(clusters=3, Y_real=y, Y_pred=y_pred)
    if name == 'KMeans' or name == 'GMM':
        print(f'[Funkcja: {name}, jaccard_score: {jaccard_score(y, y_perm, average=None).mean()}]')  
    else:  
        print(f'[Agglomerative: {name}, jaccard_score: {jaccard_score(y, y_perm, average=None).mean()}]')
    
# 2D
pca = PCA(n_components=2)
X_reduced = pca.fit_transform(X)
for name, cluster in zip(names, clusters):
    plt.figure(figsize=(16, 6))
    plt.suptitle(name)
    plt.subplot(131)
    plt.scatter(X_reduced[:, 0], X_reduced[:, 1], c=y, cmap=colors)
    for i in range(3):
        hull = ConvexHull(X_reduced[y == i])
        for simplex in hull.simplices:
            plt.plot(X_reduced[y == i][simplex, 0], X_reduced[y == i][simplex, 1], 'k')
    plt.title('oryginał')
    cluster = cluster.fit_predict(X_reduced)
    plt.subplot(132)
    for i in range(3):
        plt.scatter(X_reduced[cluster == i, 0], X_reduced[cluster == i, 1])
        if name == 'najblizsze_sasiedztwo':
            if i < 2:
                hull = ConvexHull(X_reduced[cluster == i])
                for simplex in hull.simplices:
                    plt.plot(X_reduced[cluster == i][simplex, 0], X_reduced[cluster == i][simplex, 1], 'k')
        else:
            hull = ConvexHull(X_reduced[cluster == i])
            for simplex in hull.simplices:
                plt.plot(X_reduced[cluster == i][simplex, 0], X_reduced[cluster == i][simplex, 1], 'k')
    plt.title('rezultat klasteryzacji')
    plt.subplot(133)
    y_perm = find_perm(clusters=3, Y_real=y, Y_pred=cluster)
    for j in range(len(X_reduced[:, 0])):
        if y[j] == y_perm[j]:
            plt.scatter(X_reduced[j, 0], X_reduced[j, 1], c='green')
        else:
            plt.scatter(X_reduced[j, 0], X_reduced[j, 1], c='red')
    plt.title('różnice')
plt.show()

# 3D
pca = PCA(n_components=3)
X_reduced = pca.fit_transform(X)
for name, cluster in zip(names, clusters):
    fig = plt.figure(figsize=(16, 6))
    fig.suptitle(name, fontsize=18, fontweight='bold')
    ax1 = fig.add_subplot(131, projection='3d')
    ax1.scatter(X_reduced[:, 0], X_reduced[:, 1], X_reduced[:, 2], c=y, cmap=colors)
    ax1.set_title('oryginał')
    cluster = cluster.fit_predict(X_reduced)
    ax2 = fig.add_subplot(132, projection='3d')
    for i in range(3):
        ax2.scatter(X_reduced[cluster == i, 0], X_reduced[cluster == i, 1], X_reduced[cluster == i, 2])
    ax2.set_title('rezultat klasteryzacji')
    ax3 = fig.add_subplot(133, projection='3d')
    y_perm = find_perm(clusters=3, Y_real=y, Y_pred=cluster)
    for j in range(len(X_reduced[:, 0])):
        if y[j] == y_perm[j]:
            ax3.scatter(X_reduced[j, 0], X_reduced[j, 1], X_reduced[j, 2], c='green')
        else:
            ax3.scatter(X_reduced[j, 0], X_reduced[j, 1], X_reduced[j, 2], c='red')
    plt.title('różnice')
plt.show()

# dendogramy Agglomerative
for name, cluster in zip(names, clusters):
    if name != 'KMeans' and name != 'GMM':
        cluster = cluster.fit(X)
        plt.figure(figsize=(16, 8))
        plt.title(f'Dendogram: {cluster}')
        plot_dendrogram(cluster, truncate_mode='level', p=4)
        plt.xlabel('Liczba punktów w węźle')
plt.show()

# KWANTYZACJA

names = [
    'KMeans',
    'GaussianMixture'
]

image = imread('image.jpg')
X = image.reshape((-1, 3))
for name in names:
    indx = 1
    plt.figure(figsize=(16, 8))
    plt.suptitle(f'porównanie dla {name}', fontsize=20, fontweight='bold')
    plt.subplots_adjust(
    left = 0.064,
    bottom = 0.036,
    right = 0.955,
    top = 0.924,
    wspace = 0.2,
    hspace=0.267
    )
    for i in [2, 3, 5, 10, 30, 100]:
        if name == 'KMeans':
            model = KMeans(n_clusters=i).fit(X)
            img_quant = model.cluster_centers_[model.labels_.flatten()]
        elif name == 'GaussianMixture':
            model = GaussianMixture(n_components=i).fit(X)
            img_quant = model.means_[model.predict(X).flatten()]
        img_quant = img_quant.reshape(480, 640, 3)
        plt.subplot(3, 4, indx)
        plt.axis('off')
        plt.imshow(image)
        plt.title(f'liczba klastrów: {i}', x=1.1, fontsize=18)
        indx += 1
        plt.subplot(3, 4, indx)
        plt.axis('off')
        indx += 1
        plt.imshow(img_quant.astype(np.uint32))
            # blad sredniokwadratowy
        plt.suptitle(f'MSE oraz rozkład błędu dla {name}', fontsize=20, fontweight='bold')
        err = np.sum((image.astype("float") - img_quant.astype("float")) ** 2)
        err /= float(image.shape[0] * image.shape[1])
        img_err = np.abs(image.astype('float')-img_quant.astype('float'))
        plt.subplot(3, 4, indx)
        plt.axis('off')
        plt.imshow(img_err[:, :, 1], cmap='gray')
        if i < 5:
            plt.title(f'{i}-klastry (MSE: {err:0.5f})', x= 1.1, fontsize=18)
        else:
            plt.title(f'{i}-klastrow (MSE: {err:0.5f})', x= 1.1, fontsize=18)
        indx += 1
        plt.subplot(3, 4, indx)
        plt.imshow(img_err[:, :, 1], cmap=plt.get_cmap('Greys'))
        indx += 1
plt.show()


# AgglomerativeClustering(ward)

# Przygotowanie (rescale bo dla oryginalnego to mi komputer przestal dzialac)
image = rescale(
    image,
    0.2,
    anti_aliasing=False,
    channel_axis=2
)
X = image.reshape((-1, 3))

# przygotowanie plota
indx = 1
idx = 1
fig1 = plt.figure(figsize=(16, 8))
fig2 = plt.figure(figsize=(16, 8))
fig1.suptitle('porównanie dla AgglomerativeClustering(linkage = ward)', fontsize=20, fontweight='bold')
fig2.suptitle('MSE oraz rozkład błędu dla AgglomerativeClustering(linkage = ward)', fontsize=20, fontweight='bold')
fig1.subplots_adjust(
left = 0.064,
bottom = 0.036,
right = 0.955,
top = 0.907,
wspace = 0.2,
hspace=0.267
)
fig2.subplots_adjust(
left = 0.064,
bottom = 0.036,
right = 0.955,
top = 0.907,
wspace = 0.2,
hspace=0.267
)
# przeliczanie
for i in [2, 3, 5, 10, 30, 100]:
    model = AgglomerativeClustering(n_clusters=i, linkage='ward', compute_distances=True).fit(X)
    # wyliczenie centrow
    centroids = []
    for j in range(model.labels_.min(), model.labels_.max()+1):
        centroids.append(X[model.labels_ == j].mean(0))
    centroids = np.vstack(centroids)

    img_quant = centroids[model.labels_.flatten()]
    img_quant = img_quant.reshape(96, 128, 3)

    ax = fig1.add_subplot(3, 4, indx)
    ax.axis('off')
    ax.imshow(image)
    ax.set_title(f'liczba klastrów: {i}', x=1.1, fontsize=18)
    indx += 1
    ax = fig1.add_subplot(3, 4, indx)
    ax.axis('off')
    indx += 1
    ax.imshow(img_quant)

    # blad sredniokwadratowy
    err = np.sum((image.astype("float") - img_quant.astype("float")) ** 2)
    err /= float(image.shape[0] * image.shape[1])
    img_err = np.abs(image.astype('float')-img_quant.astype('float'))
    ax = fig2.add_subplot(3, 4, idx)
    ax.axis('off')
    ax.imshow(img_err[:, :, 1], cmap='gray')
    if i < 5:
        ax.set_title(f'{i}-klastry (MSE: {err:0.5f})', x= 1.1, fontsize=18)
    else:
        ax.set_title(f'{i}-klastrow (MSE: {err:0.5f})', x= 1.1, fontsize=18)
    idx += 1
    ax = fig2.add_subplot(3, 4, idx)
    ax.imshow(img_err[:, :, 1], cmap=plt.get_cmap('Greys'))
    idx += 1
plt.show()

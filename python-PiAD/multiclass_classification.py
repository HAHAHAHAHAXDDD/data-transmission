import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from scipy import rand
from sklearn import svm
from sklearn.linear_model import LogisticRegression
from sklearn.linear_model import Perceptron
from sklearn.datasets import make_classification
from sklearn.model_selection import train_test_split
from sklearn.multiclass import OneVsOneClassifier
from sklearn.multiclass import OneVsRestClassifier
from sklearn import metrics
from sklearn.preprocessing import label_binarize

X, y = make_classification(
    n_samples = 1600,
    n_classes= 4,
    n_informative=8,
    random_state=3
)

X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.5)


names = [
    'SVC_linear',
    'SVC_rbf',
    'LogisticRegression',
    'Perceptron'
]

classifiers = [
    [OneVsOneClassifier(svm.SVC(kernel='linear', probability=True)), OneVsRestClassifier(svm.SVC(kernel='linear', probability=True))],
    [OneVsOneClassifier(svm.SVC(kernel='rbf', probability=True)), OneVsRestClassifier(svm.SVC(kernel='rbf', probability=True))],
    [OneVsOneClassifier(LogisticRegression()), OneVsRestClassifier(LogisticRegression())],
    [OneVsOneClassifier(Perceptron()), OneVsRestClassifier(Perceptron())]
]

results = pd.DataFrame(
    columns=['accuracy_score', 'recall_score', 'precision_score', 'f1_score'],
)

# pierwsze dane + wizualizacja wynikow klasyfikacji
for name, clf in zip(names, classifiers):
    for i in range(2):
        clf[i].fit(X_train, y_train)
        y_pred = clf[i].predict(X_test)
        plt.figure(figsize=(16, 8))
        if i == 0:
            plt.suptitle(f'OvO: {name}', fontsize=18, fontweight='bold')
        else: 
            plt.suptitle(f'OvR: {name}', fontsize=18, fontweight='bold')
        plt.subplot(131)
        plt.scatter(X_test[:, 0], X_test[:, 1], c=y_test, cmap=plt.cm.get_cmap('tab10'), alpha=0.8, edgecolors='k')
        plt.title('oczekiwane')
        plt.subplot(132)
        plt.scatter(X_test[:, 0], X_test[:, 1], c=y_pred, cmap=plt.cm.get_cmap('tab10'), alpha=0.8, edgecolors='k')
        plt.title('obliczone')
        plt.subplot(133)
        for j in range(len(X_test[:, 0])):
            if y_test[j] == y_pred[j]:
                plt.scatter(X_test[j, 0], X_test[j, 1], c='green')
            else:
                plt.scatter(X_test[j, 0], X_test[j, 1], c='red')
        plt.title('różnice')
        acc = metrics.accuracy_score(y_test, y_pred)
        rec = metrics.recall_score(y_test, y_pred, average=None)
        prec = metrics.precision_score(y_test, y_pred, average=None)
        F1 = metrics.f1_score(y_test, y_pred, average=None)
        results.loc[len(results)] = [acc, rec.mean(), prec.mean(), F1.mean()]
        X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.5)
        clf[i].fit(X_train, y_train)
        y_pred = clf[i].predict(X_test)

        x_min, x_max = X_test[:, 0].min() - 1, X_test[:, 0].max() + 1
        y_min, y_max = X_test[:, 1].min() - 1, X_test[:, 1].max() + 1
        xx, yy = np.meshgrid(np.arange(x_min, x_max, 0.1),
                            np.arange(y_min, y_max, 0.1))
        Z = clf[i].predict(np.c_[xx.ravel(), yy.ravel()])
        Z = Z.reshape(xx.shape)
        plt.contourf(xx, yy, Z, cmap=plt.cm.get_cmap('tab10'), alpha=0.4)
        plt.scatter(X_test[:, 0], X_test[:, 1], marker='o', c=y_pred, cmap=plt.cm.get_cmap('tab10'), s=25)
        plt.suptitle(clf[i], fontsize=18, fontweight='bold')
plt.show()

# krzywa ROC oraz jej pole dla OvR (OvO tworzy bledy?)
y = label_binarize(y, classes=[0,1,2,3])
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.5)
res = [0]*8
idx = 1
for name, clf in zip(names, classifiers):
    for i in range(2):
        if i == 1:
            y_pred = clf[i].fit(X_train, y_train).decision_function(X_test)
            fpr = dict()
            tpr = dict()
            roc_auc = dict()
            for x in range(4):
                fpr[x], tpr[x], thresholds = metrics.roc_curve(y_test[:, x], y_pred[:, x])
                roc_auc[x] = metrics.auc(fpr[x], tpr[x])
            plt.figure(figsize=(14, 9))
            plt.title(f'Krzywa ROC dla {clf[i]}')
            plt.xlabel('False Positive Rate')
            plt.ylabel('True Positive Rate')
            for z in range(4):
                plt.plot(fpr[z], tpr[z], label=f'Krzywa ROC (AUC = {roc_auc[z]:0.2f}) dla klasy {z}')
                plt.legend(loc='lower right')
                plt.plot([0, 1], [0, 1], color='k', linestyle='--')
            result = 0
            for val in roc_auc.values():
                result += val
            result /= len(roc_auc)
            res[idx] = result
            idx += 2
results['roc_auc'] = res
plt.show()

# dodanie do wynikow klasyfikacji srednich pol pod wykresem

# zestawienie wynikow klasyfikacji
results = results.rename(index=
    {
        0 : 'OvO: SVC(linear)',
        1 : 'OvR: SVC(linear)',
        2 : 'OvO: SVC(rbf)',
        3 : 'OvR: SVC(rbf)',
        4 : 'OvO: LogisticRegression',
        5 : 'OvR: LogisticRegression',
        6 : 'OvO: Perceptron',
        7 : 'OvR: Perceptron'
    }
)
results = results.T
ax = results.plot.bar(figsize=[12, 7])
ax.legend(loc='upper right')
plt.show()
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import time
import seaborn as sns
from sklearn.datasets import make_classification
from sklearn import naive_bayes
from sklearn import discriminant_analysis
from sklearn import neighbors
from sklearn import svm
from sklearn import tree
from sklearn.model_selection import GridSearchCV, train_test_split
from sklearn import metrics

# ZAD 1
X, y = make_classification(
    n_samples=200, 
    n_features=2,
    n_classes=2, 
    n_clusters_per_class=2, 
    n_redundant=0,
    random_state=3
)
cmap = matplotlib.colors.ListedColormap(['crimson', 'mediumslateblue'])
colormap = np.array(['crimson', 'mediumslateblue'])
# plt.scatter(X[:, 0], X[:, 1], marker='o', c=colormap[y], s=25)
# plt.show()

classifiers = [
    naive_bayes.GaussianNB(),
    discriminant_analysis.QuadraticDiscriminantAnalysis(),
    neighbors.KNeighborsClassifier(),
    svm.SVC(probability=True),
    tree.DecisionTreeClassifier(),
]

GaussianNB_scores = pd.DataFrame(columns=['accuracy_score', 'recall_score', 'precision_score', 'f1_score', 'roc_auc', 'train_time', 'test_time'])
QuadraticDiscriminantAnalysis_scores = pd.DataFrame(columns=['accuracy_score', 'recall_score', 'precision_score', 'f1_score', 'roc_auc', 'train_time', 'test_time'])
NearestNeighbors_scores = pd.DataFrame(columns=['accuracy_score', 'recall_score', 'precision_score', 'f1_score', 'roc_auc', 'train_time', 'test_time'])
SVM_scores = pd.DataFrame(columns=['accuracy_score', 'recall_score', 'precision_score', 'f1_score', 'roc_auc', 'train_time', 'test_time'])
DecisionTree_scores = pd.DataFrame(columns=['accuracy_score', 'recall_score', 'precision_score', 'f1_score', 'roc_auc', 'train_time', 'test_time'])

dataframes = {
    'GaussianNB': GaussianNB_scores,
    'QuadraticDiscriminantAnalysis': QuadraticDiscriminantAnalysis_scores,
    'NearestNeighbors': NearestNeighbors_scores,
    'SVM': SVM_scores,
    'DecisionTree': DecisionTree_scores
}

for i in range(100):
    X_train, X_test, y_train, y_test = train_test_split(X, y)
    for name, clf in zip(dataframes, classifiers):
        
        start_time = time.perf_counter()
        clf.fit(X_train, y_train)
        stop_time = time.perf_counter()
        train_time = (stop_time-start_time)*100

        start_time = time.perf_counter()
        y_pred = clf.predict(X_test)
        stop_time = time.perf_counter()
        test_time = (stop_time-start_time)*100
        
        acc = metrics.accuracy_score(y_test, y_pred)
        rec = metrics.recall_score(y_test, y_pred)
        prec = metrics.precision_score(y_test, y_pred)
        F1 = metrics.f1_score(y_test, y_pred)
        roc = metrics.roc_auc_score(y_test, y_pred)
        
        dataframes[name].loc[len(dataframes[name])] = [acc, rec, prec, F1, roc, train_time, test_time]
        if i == 99:
            plt.figure(figsize=(16, 8))
            plt.suptitle(name, fontsize=18, fontweight='bold')
            plt.subplot(231)
            plt.scatter(X_test[:, 0], X_test[:, 1], c=y_test, cmap=plt.cm.get_cmap('spring'), alpha=0.8, edgecolors='k')
            plt.title('oczekiwane')
            plt.subplot(232)
            plt.scatter(X_test[:, 0], X_test[:, 1], c=y_pred, cmap=plt.cm.get_cmap('spring'), alpha=0.8, edgecolors='k')
            plt.title('obliczone')
            plt.subplot(233)
            for j in range(len(X_test[:, 0])):
                if y_test[j] == y_pred[j]:
                    plt.scatter(X_test[j, 0], X_test[j, 1], c='green')
                else:
                    plt.scatter(X_test[j, 0], X_test[j, 1], c='red')
            plt.title('różnice')
            plt.subplot(223)
            probs = clf.predict_proba(X_test)
            preds = probs[:, 1]
            fpr, tpr, thresholds = metrics.roc_curve(y_test, preds)
            auc = metrics.auc(fpr, tpr)
            plt.plot(fpr, tpr, label = 'AUC = %0.2f' % auc, lw=2)
            plt.legend(loc = 'lower right')
            plt.plot([0, 1], [0, 1], color='red', linestyle='--')
            plt.title(f'ROC Curve of {name}')
            plt.xlabel('False Positive Rate')
            plt.ylabel('True Positive Rate')
            plt.subplot(224)
            x_min, x_max = X_test[:, 0].min() - 1, X_test[:, 0].max() + 1
            y_min, y_max = X_test[:, 1].min() - 1, X_test[:, 1].max() + 1
            xx, yy = np.meshgrid(np.arange(x_min, x_max, 0.1),
                                np.arange(y_min, y_max, 0.1))
            Z = clf.predict(np.c_[xx.ravel(), yy.ravel()])
            Z = Z.reshape(xx.shape)
            plt.contourf(xx, yy, Z, cmap=cmap, alpha=0.4)
            plt.scatter(X_test[:, 0], X_test[:, 1], marker='o', c=y_pred, cmap=cmap, s=25)
plt.show()

# zad 1 - podsumowanie wszystkich klasyfikatorów        
tests = [
    'accuracy_score', 
    'recall_score', 
    'precision_score', 
    'f1_score', 
    'roc_auc', 
    'train_time', 
    'test_time'
]

results = pd.DataFrame(
    columns=['accuracy_score', 'recall_score', 'precision_score', 'f1_score', 'roc_auc', 'train_time', 'test_time',],
    index=['GaussianNB', 'QuadraticDiscriminantAnalysis', 'NearestNeighbors', 'SVM', 'DecisionTree']
    )

for name in dataframes:
    scores = []
    for test in tests:
        scores.append(dataframes[name][test].mean())
    results.loc[name] = scores
results = results.T
ax = results.plot.bar(figsize=[12, 7])
plt.subplots_adjust(
    top = 0.97,
    bottom = 0.183
)
plt.show()




#ZAD 2
X, y = make_classification(
    n_samples=200, 
    n_features=2,
    n_classes=2, 
    n_clusters_per_class=2, 
    n_redundant=0,
    random_state=3
)

QDA = discriminant_analysis.QuadraticDiscriminantAnalysis()

dict = {
    'QuadraticDiscriminantAnalysis':{
        'reg_param' : [0.00001, 0.0001, 0.001, 0.01, 0.1],
        'tol': [0.00001, 0.0001, 0.001, 0.01, 0.1],
    }
}

clf = GridSearchCV(
    estimator = QDA,
    param_grid = dict['QuadraticDiscriminantAnalysis'],
    scoring = 'accuracy',
    return_train_score = True
)

clf.fit(X, y)
print(clf.best_params_)
df = pd.DataFrame(clf.cv_results_)
df = df[['param_reg_param', 'param_tol', 'mean_test_score']]
lin = np.linspace(0.00001, 0.1, len(df['mean_test_score'].values))
plt.figure(figsize=(16,8))
plt.subplot(121)
plt.plot(lin, df['mean_test_score'].values)
plt.xlabel('parametry')
plt.ylabel('accuracy')
plt.subplot(122)
scores = clf.cv_results_['mean_test_score'].reshape(
    len(dict['QuadraticDiscriminantAnalysis']['reg_param']),
    len(dict['QuadraticDiscriminantAnalysis']['tol'])
)
plt.imshow(scores, interpolation='nearest', cmap=plt.cm.get_cmap('autumn'))
plt.colorbar(label='score')
plt.xlabel('reg_param')
plt.ylabel('tol')
plt.xticks(np.arange(len(dict['QuadraticDiscriminantAnalysis']['reg_param'])), dict['QuadraticDiscriminantAnalysis']['reg_param'])
plt.yticks(np.arange(len(dict['QuadraticDiscriminantAnalysis']['tol'])), dict['QuadraticDiscriminantAnalysis']['tol'])
plt.title('score')
plt.show()


clf = discriminant_analysis.QuadraticDiscriminantAnalysis(reg_param=0.00001, tol=0.00001, store_covariance=True)
qda_scores = pd.DataFrame(columns=['accuracy_score', 'recall_score', 'precision_score', 'f1_score', 'roc_auc', 'train_time', 'test_time'])
for i in range(100):
    X_train, X_test, y_train, y_test = train_test_split(X, y)
    
    start_time = time.perf_counter()
    clf.fit(X_train, y_train)
    stop_time = time.perf_counter()
    train_time = (stop_time-start_time)*100

    start_time = time.perf_counter()
    y_pred = clf.predict(X_test)
    stop_time = time.perf_counter()
    test_time = (stop_time-start_time)*100
    
    acc = metrics.accuracy_score(y_test, y_pred)
    rec = metrics.recall_score(y_test, y_pred)
    prec = metrics.precision_score(y_test, y_pred)
    F1 = metrics.f1_score(y_test, y_pred)
    roc = metrics.roc_auc_score(y_test, y_pred)
    qda_scores.loc[len(qda_scores)] = [acc, rec, prec, F1, roc, train_time, test_time]
    if i == 99:
        plt.figure(figsize=(16, 8))
        plt.subplot(121)
        probs = clf.predict_proba(X_test)
        preds = probs[:, 1]
        fpr, tpr, thresholds = metrics.roc_curve(y_test, preds)
        auc = metrics.auc(fpr, tpr)
        plt.plot(fpr, tpr, label = 'AUC = %0.2f' % auc, lw=2)
        plt.legend(loc = 'lower right')
        plt.plot([0, 1], [0, 1], color='red', linestyle='--')
        plt.title(f'ROC Curve of QuadraticDiscriminantAnalysis')
        plt.xlabel('False Positive Rate')
        plt.ylabel('True Positive Rate')
        plt.subplot(122)
        x_min, x_max = X_test[:, 0].min() - 1, X_test[:, 0].max() + 1
        y_min, y_max = X_test[:, 1].min() - 1, X_test[:, 1].max() + 1
        xx, yy = np.meshgrid(np.arange(x_min, x_max, 0.1),
                            np.arange(y_min, y_max, 0.1))
        Z = clf.predict(np.c_[xx.ravel(), yy.ravel()])
        Z = Z.reshape(xx.shape)
        plt.contourf(xx, yy, Z, cmap=cmap, alpha=0.4)
        plt.scatter(X_test[:, 0], X_test[:, 1], marker='o', c=y_pred, cmap=cmap, s=25)
plt.show()

qda_scores = qda_scores.mean(axis=0)
print(qda_scores)
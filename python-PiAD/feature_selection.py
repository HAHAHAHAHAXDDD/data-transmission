import numpy as np
import pandas as pd
from scipy import sparse
import sklearn as sk
from sklearn import feature_selection as test

zoo = pd.read_csv('zoo.csv')

#1
def freq(x, prob):   
    xi = []
    ni = []
    for i in x:
        if i not in xi:
            xi.append(i)
            ni.append(1)
        elif i in xi:
            ni[xi.index(i)] = ni[xi.index(i)]+1
    if prob:
        ni = [1/len(xi) for _ in ni] # prawdopodobienstwo?
    else:
        ni = [i/len(x) for i in ni] # czestosci czyli przez ich liczbe trzeba podzielic
    return xi, ni

#2
def freq2(x,y,prob):
    xi = []
    yi = []
    ni = []
    xi = x.unique()
    yi = y.unique()
    ni = [x.value_counts(), y.value_counts()]
    rows, cols = np.shape(ni)
    brzegir = []
    for i in range(rows):
        brzegir.append(np.sum(ni[i][:]))
    brzegir = np.c_[brzegir]
    ni = np.append(ni, brzegir, axis = 1)
    brzegiw = [sum(x) for x in zip(*ni)]
    ni = np.vstack([ni, brzegiw])
    if prob:
        ni = ni/ni[-1,-1]
        return xi, yi, ni
    return xi, yi, ni

#3
def entropy(x):
    entr = 0
    # [xi, ni] = freq(x, prob=False)
    for i in range(len(x)):
        entr += x[i]*np.log2(x[i])    
    return -entr

def infogain(x, y):
    inf = 0
    [xi, yi, ni] = freq2(x,y,prob=True)
    xv = ni[:,-1]
    xv = np.delete(xv, -1)
    hx = entropy(xv)
    yv = ni[-1,:]
    yv = np.delete(yv, -1)
    hy = entropy(yv)
    ni = np.delete(ni, -1, axis=1)
    ni = np.delete(ni, -1, axis=0)
    hxy = entropy(ni)
    hxy = np.sum(hxy)
    entrop_war = hxy - hx
    inf = hy - entrop_war
    return inf

#4
zoo = zoo.iloc[:, :-1]
zoo = zoo.iloc[:, 1:]
scores = test.mutual_info_classif(zoo, zoo['hair'], random_state=0)
scores = pd.Series(scores)
scores.index = zoo.columns
scores = scores.sort_values(ascending=False)
zoo = zoo.iloc[:, :-1]
zoo = zoo.iloc[:, 1:]

#4
scores=[]
zoo = zoo.loc[:, zoo.columns != "animal"]
zoo = zoo.loc[:, zoo.columns != "legs"]
zoo = zoo.loc[:, zoo.columns != "type"]
for column in zoo:
    scores.append(infogain(zoo[column], zoo['hair']))
scores = np.array(scores)
scores = pd.Series(scores)
scores.index = zoo.columns
scores = scores.sort_values(ascending=False)
print(scores)

    

#5
sparsehair = sparse.csc_matrix(zoo['hair'])
sparsefeathers = sparse.csc_matrix(zoo['feathers'])
print(sparsehair)
[xi, ni] = freq(sparsehair, prob=True)
print(xi, ni)
print(sparsefeathers)
[xi2, yi2, ni2] = freq2(sparsehair, sparsefeathers, prob=True)
print(xi2, yi2, ni2)
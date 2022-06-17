import pandas as pd
import numpy as np
import scipy.stats as sp
import matplotlib.pyplot as mpl

df = pd.DataFrame({"x": [1,2,3,4,5], 'y': ['a', 'b', 'a', 'b', 'b']})
print(df)

# 1
print(df.groupby('y').mean())

# 2
print(df['y'].value_counts())

# 3
print(np.loadtxt('autos.csv', dtype=(str), delimiter=','))
print(pd.read_csv("autos.csv"))

df = pd.read_csv('autos.csv')
#4
grupa = df.groupby('make')
print(grupa['normalized-losses'].mean())

#5
grupa = df.groupby('make')
print(grupa['fuel-type'].value_counts())

#6
print(np.polyfit(df['city-mpg'], df['length'], 1))
print(np.polyfit(df['city-mpg'], df['length'], 2))

#7
print(sp.pearsonr(df['city-mpg'], df['length']))

#8
w1 = np.polyfit(df['city-mpg'], df['length'], 1)
w2 = np.polyfit(df['city-mpg'], df['length'], 2)
polyw1 = np.poly1d(w1)
polyw2 = np.polyval(w2, df['city-mpg'])
mpl.plot(df['city-mpg'], df['length'], '.')
trendpolyw1 = np.poly1d(polyw1)
mpl.plot(df['city-mpg'], trendpolyw1(df['city-mpg']), '--')
mpl.plot(df['city-mpg'], polyw2, '-')
mpl.xlabel('city-mpg')
mpl.ylabel('length')
mpl.legend(['probki', 'w1', 'w2'])
mpl.show()

#9
kernel = sp.gaussian_kde(df['length'])
v = np.linspace(df['length'].min(), df['length'].max())
u = kernel.evaluate(v)
mpl.plot(v, u)
mpl.hist(df['length'], density = True, edgecolor = 'Black')
mpl.legend(['estymator', 'probki'])
mpl.show()

#10
kernel = sp.gaussian_kde(df['length'])
kernel2 = sp.gaussian_kde(df['width'])
v = np.linspace(df['length'].min(), df['length'].max())
v2 = np.linspace(df['width'].min(), df['width'].max())
u = kernel.evaluate(v)
u2 = kernel2.evaluate(v2)
fig, ax = mpl.subplots(2)
ax[0].plot(v, u)
ax[0].hist(df['length'], density = True, edgecolor = 'Black')
ax[0].legend(['estymator', 'probki'])
ax[0].title.set_text('length')
ax[1].plot(v2, u2)
ax[1].hist(df['width'], density = True, edgecolor = 'Black')
ax[1].legend(['estymator', 'probki'])
ax[1].title.set_text('width')
mpl.show()

#11
values = np.vstack([df['width'], df['length']])
kernel = sp.gaussian_kde(values)
X, Y = np.mgrid[df['width'].min():df['width'].max(), df['length'].min():df['length'].max()]
# X, Y = np.meshgrid(df['width'], df['length'])
positions = np.vstack([X.ravel(), Y.ravel()])
Z = np.reshape(kernel(positions).T, X.shape)
mpl.contourf(X, Y, Z, cmap='Blues')
# mpl.contour(X, Y, Z, colors='k')
mpl.plot(df['width'], df['length'], '.', color='red')
mpl.savefig('lab2plot.png')
mpl.savefig('lab2plot.pdf')
mpl.show()
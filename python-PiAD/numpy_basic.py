import numpy as np
from numpy.lib.stride_tricks import as_strided

#1
a = np.random.randint(0, 100, size=(10,5))
print(a)
print(np.trace(a))
print(np.diag(a))

#2
a = np.random.normal(size=(5,5))
b = np.random.normal(size=(5,5))
print(a)
print(b)
print(a*b)

#3
a = np.random.randint(1,100, size=(10))
b = np.random.randint(1,100, size=(10))
a = a.reshape(-1,5)
b = b.reshape(-1,5)
print(a)
print(b)
print(np.add(a,b))

#4
a = np.random.randint(1,100, size=(4,5))
b = np.random.randint(1,100, size=(5,4))
print(a)
print(b)
b = np.transpose(b)
print(np.add(a,b))

#5
a = np.random.randint(1,100, size=(4,5))
b = np.random.randint(1,100, size=(5,4))
print(a)
print(b)
b = np.transpose(b)
print(a[:,2]*b[:,3])

#6
a = np.random.normal(size=(4,4))
b = np.random.uniform(size=(4,4))
amean = np.mean(a)
adev = np.std(a)
avar = np.var(a)
asum = np.sum(a)
maxa = np.max(a)
mina = np.min(a)
bmean = np.mean(b)
bdev = np.std(b)
bvar = np.var(b)
bsum = np.sum(b)
maxb = np.max(b)
minb = np.min(b)
print(a)
print(f'r.normalny:\nsrednia: {amean}\nodchylenie standardowe: {adev}\nwariancja: {avar}\nsuma: {asum}\nmax: {maxa}\nmin: {mina}\n')
print(b)
print(f'r.jednostajny:\nsrednia: {bmean}\nodchylenie standardowe: {bdev}\nwariancja: {bvar}\nsuma: {bsum}\nmax: {maxb}\nmin: {minb}')

#7
a = np.random.randint(1,100, size=(5,5))
b = np.random.randint(1,100, size=(5,5))
first = a*b
second = np.dot(a,b)
print(f'{a}\n{b}\na*b:\n{first}\ndot:\n{second}')
# Roznica jest taka ze mnozenie a*b mnozy liczby w danych komórkach
# np. c[1,1] = a[1,1] * b[1,1] itd. (gdzie c to macierz wynikowa)
# natomiast funkcja dot wykonuje klasyczne mnozenie macierzowe czyli wiersz z macierzy a
# jest mnozony przez kolumne macierzy b, itd. czego wynikiem jest macierz c
# Funkcje dot warto wykorzystac wlasnie przy mnozeniu macierzy, ponieważ wykonuje ona
# poprawne mnozenie macierzy, a nie mnożenie poszczególnych komórek

#8
a = np.arange(24, dtype=np.int64).reshape(4,6)
print(a)
x = as_strided(a, shape=(3,5), strides=(48,8))
print(x)

#9
a = np.arange(6)
b = np.arange(6,12)
print(a)
print(b)
print(np.vstack((a,b)))
print(np.stack((a,b)))
# Funkcja vstack przyjmuje tylko jeden argument jakim są tablice do połączenia oraz łączy
# je tylko w sposób wertykalny?
# natomiast stack przyjmuje dodatkowe opcjonalne funkcje jak np "axis", które określa
# w jaki sposób dane są łączone do macierzy wynikowej
# vstack warto stosować jeśli jest pewność, że połączenie ma nastąpić w sposób wertykalny
# natomiast stack jest bardziej uniwersalny dzięki temu, że można zadecydować w jaki sposób
# dane będą połączone

#10
a = np.arange(24, dtype=np.int64).reshape(4,6)
print(a)
result = as_strided(a, shape=(2,2,2,3), strides=(96,24,48,8))
print(result)
print(np.max(result[0][0]))
print(np.max(result[0][1]))
print(np.max(result[1][0]))
print(np.max(result[1][1]))
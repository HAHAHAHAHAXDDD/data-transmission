from cmath import pi
from configparser import Interpolation
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
import matplotlib.image as mpl


# Dyskretyzacja
# 1,2
def function(f, Fs): 
    t = np.arange(0, 1, 1/Fs)
    s = []
    for i in t:
        s.append(np.sin(2*pi*f*i))
    return t,s
# #3
f = 10
Fs = [20,21,30,45,50,100,150,200,250,1000]
fig, ax = plt.subplots(nrows=3, ncols=4)
it = 0  
for i in range(3):
    for j in range(3):
        x, y = function(f, Fs[it])
        ax[i][j].plot(x, y)
        ax[i][j].set_title('{}Hz'.format(Fs[it]))
        if (it < 10):
            it = it+1
x,y = function(f, Fs[-1])
ax[0][3].plot(x,y)
ax[0][3].set_title('{}Hz'.format(Fs[-1]))
fig.delaxes(ax[1][3])
fig.delaxes(ax[2][3])
plt.show()
# #4
# Istnieje takie twierdzenie, jest to twierdzenie Shannona-Kotielnikowa
#5
# Zjawisko to nosi nazwę Aliasing
#6
# to fotka w folderze
#7
img = mpl.imread('aliasing.png')
methods = [None, 'none', 'nearest', 'bilinear', 'bicubic', 'spline16',
           'spline36', 'hanning', 'hamming', 'hermite', 'kaiser', 'quadric',
           'catrom', 'gaussian', 'bessel', 'mitchell', 'sinc', 'lanczos']
fig, axs = plt.subplots(nrows=3, ncols=6, figsize=(9,6),
subplot_kw={'xticks': [], 'yticks': []})
for ax, interp_method in zip(axs.flat, methods):
    # plt.imshow(img, cmap='viridis')
    ax.imshow(img, interpolation=interp_method, cmap='viridis')
    ax.set_title(str(interp_method))

plt.tight_layout()
plt.show()
# nie radzi sobie

# Kwantyzacja
#1
img = plt.imread('aliasing.png')
# #2
print(img.ndim)
# #3
print(img[:,:,3].shape)
# #4
R, G, B= img[:, :, 0], img[:, :, 1], img[:, :, 2]
imgGray1 = np.dot(img, (R.min()+R.max()/2, G.min()+G.max()/2, B.min()+B.max()/2, -1))
imgGray2 = (R+G+B)/3
imgGray3 = 0.21*R + 0.72*G + 0.07*B
fig, axs = plt.subplots(3)
axs[0].imshow(imgGray1, cmap='gray')
axs[0].set_title('Wyznaczenie jasnosci piksela')
axs[1].imshow(imgGray2, cmap='gray')
axs[1].set_title('Usrednienie wartosci piksela')
axs[2].imshow(imgGray3, cmap='gray')
axs[2].set_title('Wyznaczenie luminacji piksela')
print(img.shape)
plt.show()
#5
hist1, bin_edges1 = np.histogram(imgGray1, bins='auto', density=True)
hist2, bin_edges2 = np.histogram(imgGray2, bins='auto', density=True)
hist3, bin_edges3 = np.histogram(imgGray3, bins='auto', density=True)
fig, axis = plt.subplots(3)
axis[0].plot(bin_edges1[0:-1], hist1)
axis[0].set_title('Wyznaczenie jasnosci piksela')
axis[1].plot(bin_edges2[0:-1], hist2)
axis[1].set_title('Usrednienie wartosci piksela')
axis[2].plot(bin_edges3[0:-1], hist3)
axis[2].set_title('Wyznaczenie luminacji piksela')
plt.show()
#6
hist, bin_edges = np.histogram(imgGray3, bins=16, density=True)
print(hist)
print('{}-{}'.format(hist.min(), hist.max()))
plt.plot(bin_edges[0:-1], hist)
plt.show()

# #7
hist.sort()
reducedImg = np.zeros_like(imgGray3)
reducedImg = imgGray3/(hist[-1]/2)
plt.imshow(reducedImg, cmap='gray')
plt.show()
fig, axis = plt.subplots(nrows=4, ncols=2)
axis[0][0].imshow(imgGray1, cmap='gray')
axis[0][1].plot(bin_edges1[0:-1], hist1)
axis[0][0].set_title("Wyznaczenie jasności piksela + histogram")
axis[0][0].axis('off')

axis[1][0].imshow(imgGray2, cmap='gray')
axis[1][1].plot(bin_edges2[0:-1], hist2)
axis[1][0].set_title("Usrednienie wartosci piksela + histogram")
axis[1][0].axis('off')

axis[2][0].imshow(imgGray3, cmap='gray')
axis[2][1].plot(bin_edges3[0:-1], hist3)
axis[2][0].set_title("Wyznaczenie luminancji piksela + histogram")
axis[2][0].axis('off')

axis[3][0].imshow(reducedImg, cmap='gray')
axis[3][1].plot(bin_edges[0:-1], hist)
axis[3][0].set_title("Zredukowana liczba kolorów + histogram")
axis[3][0].axis('off')

# plt.show()


# Binaryzacja
#1 
# 'binar.png' to fotka
#2
img = plt.imread('binar.png')
R, G, B= img[:, :, 0], img[:, :, 1], img[:, :, 2]
imgGray = 0.21*R + 0.72*G + 0.07*B
hist, bin_edges = np.histogram(imgGray, bins=256, density=True)
fig, axis = plt.subplots(3)
axis[0].imshow(img)
axis[0].set_title('oryginal')
axis[0].axis('off')
axis[1].imshow(imgGray, cmap='gray')
axis[1].set_title('skala szarosci')
axis[1].axis('off')
axis[2].plot(bin_edges[0:-1], hist)
axis[2].set_title('histogram')
plt.show()
# # #3
# # # na podstawie wyswietlonego histogramu mozna stwierdzic
# # # ze tam przy ~0.5 sie cos dziac zaczyna dlatego tak
mask = imgGray < 0.56
selection = np.zeros_like(img)
selection[mask] = img[mask]
plt.imshow(selection)
plt.show()
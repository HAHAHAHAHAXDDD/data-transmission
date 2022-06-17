import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from scipy.fft import fftfreq, fftshift
import sounddevice as sd
import soundfile as sf
from sklearn import preprocessing
import scipy.fftpack

def normalize(lst):
    arr = np.array(lst).reshape(-1, 1)
    scaler = preprocessing.MinMaxScaler(feature_range=(0, 1))
    arr = scaler.fit_transform(arr)
    return arr



# Zad 1

# 1, 2
s, fs = sf.read('audio.wav', dtype='float32')
inf = sf.SoundFile('audio.wav') #informacje o pliku
n = len(s) # dlugosc tej tablicy s
print(f'Czas trwania {round(n/fs, 2)} s')
print(f'Czestotliwosc probkowania: {fs} Hz')
print(f'Rozdzielczosc bitowa: {inf.subtype_info}')
print(f'Liczba kanalow: {inf.channels}')

# 3
l_channel = s[:, 0]
time = np.linspace(0, (n/fs)*1000, n)
plt.figure(figsize=(15,5))
plt.plot(time, l_channel, 'lime')
plt.title('Signal')
plt.xlabel(f'time in ms ({n} samples)')
plt.ylabel('signal value')
plt.axhline(y=0, color='black')

# Zad 2
# 1, 2
E = []
Z = []
mstime = (n/fs)*1000
for i in range(0, int(mstime), 10):
    summ = 0
    for j in range(i+1, i+9):
        summ += (l_channel[j]**2)
    E.append(summ)
    summ = 0
    for j in range(i+1, i+8):
        if (l_channel[j]*l_channel[j+1]) >= 0:
            summ += 0
        else:
            summ += 1 
    Z.append(summ)

# normalizacja
E = normalize(E)
Z = normalize(Z)

# linspace dla E, Z i plotowanie ich
zlin = np.linspace(0, mstime, len(Z))
elin = np.linspace(0, mstime, len(E))
plt.plot(zlin, Z, 'b', alpha=0.7)
plt.plot(elin, E, 'r', alpha=0.7)

# 4
def zad2(frame):
    E = []
    Z = []
    mstime = (n/fs)*1000
    for i in range(0, int(mstime), frame):
        summ = 0
        for j in range(i+1, i+(frame-1)):
            summ += (l_channel[j]**2)
        E.append(summ)
        summ = 0
        for j in range(i+1, i+(frame-2)):
            if (l_channel[j]*l_channel[j+1]) >= 0:
                summ += 0
            else:
                summ += 1 
        Z.append(summ)
    return E, Z

(E5, Z5) = zad2(frame=5)
(E20, Z20) = zad2(frame=20)
(E50, Z50) = zad2(frame=50)
E5 = normalize(E5)
Z5 = normalize(Z5)
z5lin = np.linspace(0, (n/fs)*1000, len(Z5))
e5lin = np.linspace(0, (n/fs)*1000, len(E5))
E20 = normalize(E20)
Z20 = normalize(Z20)
z20lin = np.linspace(0, (n/fs)*1000, len(Z20))
e20lin = np.linspace(0, (n/fs)*1000, len(E20))
E50 = normalize(E50)
Z50 = normalize(Z50)
z50lin = np.linspace(0, (n/fs)*1000, len(Z50))
e50lin = np.linspace(0, (n/fs)*1000, len(E50))


fig, (ax1, ax2, ax3) = plt.subplots(3)
fig.set_figheight(8)
fig.set_figwidth(17)
ax1.plot(time, l_channel, 'lime', alpha=0.9)
ax1.plot(z5lin, Z5, 'b', alpha=0.7)
ax1.plot(e5lin, E5, 'r', alpha=0.7)
ax1.set_title('5ms')
ax1.set_xlabel(f'time in ms ({n} samples)')
ax1.set_ylabel('signal value')
ax2.plot(time, l_channel, 'lime', alpha=0.9)
ax2.plot(z20lin, Z20, 'b', alpha=0.7)
ax2.plot(e20lin, E20, 'r', alpha=0.7)
ax2.set_title('20ms')
ax2.set_xlabel(f'time in ms ({n} samples)')
ax2.set_ylabel('signal value')
ax3.plot(time, l_channel, 'lime', alpha=0.9)
ax3.plot(z50lin, Z50, 'b', alpha=0.7)
ax3.plot(e50lin, E50, 'r', alpha=0.7)
ax3.set_title('50ms')
ax3.set_xlabel(f'time in ms ({n} samples)')
ax3.set_ylabel('signal value')
fig.legend(['sygnal', 'przejscia przez zero', 'funkcja energii'])
plt.subplots_adjust(left=0.064,
                    bottom=0.055,
                    right=0.957,
                    top=0.971,
                    wspace=0.202,
                    hspace=0.355)                   


def zad5(frame):
    E = []
    Z = []
    mstime = (n/fs)*1000
    for i in range(0, int(mstime), frame):
        summ = 0
        for j in range(int(i*0.5), int(i*0.5)+frame):
            summ += (l_channel[j]**2)
        E.append(summ)
        summ = 0
        for j in range(int(i*0.5), int(i*0.5)+frame):
            if (l_channel[j]*l_channel[j+1]) >= 0:
                summ += 0
            else:
                summ += 1 
        Z.append(summ)
    return E, Z

(E5, Z5) = zad5(frame=5)
(E20, Z20) = zad5(frame=20)
(E50, Z50) = zad5(frame=50)
E5 = normalize(E5)
Z5 = normalize(Z5)
z5lin = np.linspace(0, (n/fs)*1000, len(Z5))
e5lin = np.linspace(0, (n/fs)*1000, len(E5))
E20 = normalize(E20)
Z20 = normalize(Z20)
z20lin = np.linspace(0, (n/fs)*1000, len(Z20))
e20lin = np.linspace(0, (n/fs)*1000, len(E20))
E50 = normalize(E50)
Z50 = normalize(Z50)
z50lin = np.linspace(0, (n/fs)*1000, len(Z50))
e50lin = np.linspace(0, (n/fs)*1000, len(E50))


fig, (ax1, ax2, ax3) = plt.subplots(3)
fig.set_figheight(8)
fig.set_figwidth(17)
ax1.plot(time, l_channel, 'lime', alpha=0.9)
ax1.plot(z5lin, Z5, 'b', alpha=0.7)
ax1.plot(e5lin, E5, 'r', alpha=0.7)
ax1.set_title('5ms')
ax1.set_xlabel(f'time in ms ({n} samples)')
ax1.set_ylabel('signal value')
ax2.plot(time, l_channel, 'lime', alpha=0.9)
ax2.plot(z20lin, Z20, 'b', alpha=0.7)
ax2.plot(e20lin, E20, 'r', alpha=0.7)
ax2.set_title('20ms')
ax2.set_xlabel(f'time in ms ({n} samples)')
ax2.set_ylabel('signal value')
ax3.plot(time, l_channel, 'lime', alpha=0.9)
ax3.plot(z50lin, Z50, 'b', alpha=0.7)
ax3.plot(e50lin, E50, 'r', alpha=0.7)
ax3.set_title('50ms')
ax3.set_xlabel(f'time in ms ({n} samples)')
ax3.set_ylabel('signal value')
fig.legend(['sygnal', 'przejscia przez zero', 'funkcja energii'])
plt.subplots_adjust(left=0.064,
                    bottom=0.055,
                    right=0.957,
                    top=0.971,
                    wspace=0.202,
                    hspace=0.355) 
plt.show()

# Zad 3
# caly sygnal
bs, bfs = sf.read('audio.wav', dtype='float32')
bn = len(bs)
bl_channel = bs[:, 0]
btime = np.linspace(0, (bn/bfs)*1000, bn)
E = []
Z = []
mstime = (bn/bfs)*1000
for i in range(0, int(mstime), 50):
    summ = 0
    for j in range(i+1, i+49):
        summ += (bl_channel[j]**2)
    E.append(summ)
    summ = 0
    for j in range(i+1, i+48):
        if (bl_channel[j]*bl_channel[j+1]) >= 0:
            summ += 0
        else:
            summ += 1 
    Z.append(summ)

E = normalize(E)
Z = normalize(Z)
zlin = np.linspace(0, mstime, len(Z))
elin = np.linspace(0, mstime, len(E))

# samogloska
s, fs = sf.read('e.wav', dtype='float32')
l_channel = s[:, 0]
W = l_channel[17893:19941]
n = len(W)
time = np.linspace(0, (n/fs)*1000, n)

H = np.hamming(2048)

wh = W*H

yf = scipy.fftpack.fft(wh)
ampl = np.log(np.abs(yf))
ampl_lin = np.linspace(0, 4, len(ampl))

ampl1s = ampl[0:370]
ampl1s_lin = np.linspace(0, 10000, len(ampl1s))

# sprawdzenie czestotliwosci F0 (bo na wykresie nie widac)
# czyli jest to ok 135/136 Hz
print(np.interp(135, ampl1s_lin, ampl1s))
for i in range(0, len(ampl1s)):
    if ampl1s[i] > 4.5:
        print(ampl1s_lin[i])

plt.figure(figsize=(16,8))
plt.subplot(311)
plt.plot(btime, bl_channel, 'lime')
plt.title('Signal')
plt.xlabel(f'time in ms ({bn} samples)')
plt.ylabel('signal value')
plt.axhline(y=0, color='black')
plt.plot(zlin, Z, 'b', alpha=0.7)
plt.plot(elin, E, 'r', alpha=0.7)

plt.subplot(345)
plt.plot(time, W, 'lime')
plt.xlabel(f'time in ms ({n} samples)')
plt.ylabel('signal value')
plt.title('Signal (analysed window) W')

plt.subplot(346)
plt.plot(H)
plt.xlabel(f'{n} samples')
plt.ylabel('signal value')
plt.title('Hamming window H')

plt.subplot(347)
plt.plot(wh)
plt.xlabel(f'{n} samples')
plt.ylabel('signal value')
plt.title('W*H')

plt.subplot(348)
plt.plot(ampl_lin, ampl, 'r')
plt.title('Amplitude spectrum')

plt.subplot(313)
plt.plot(ampl1s_lin, ampl1s, 'r')
plt.title('Amplitude spectrum: log(abs(fft(W*H)))')
plt.xlabel('Frequency')
plt.ylabel('Values')
plt.xticks([0, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000])

plt.subplots_adjust(left=0.079,
                    bottom=0.074,
                    right=0.938,
                    top=0.97,
                    wspace=0.283,
                    hspace=0.407) 

plt.show()
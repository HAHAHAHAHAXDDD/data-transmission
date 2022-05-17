#include <iostream>
#include <complex>
#include <iomanip>
#include <time.h>
#include <Windows.h>
#define _USE_MATH_DEFINES
#include "math.h"

const std::complex<double> i(0, 1);

std::complex<double> *DFT(double *& f, const int N)
{
	std::complex<double>* C = new std::complex<double>[N];
	for (int k = 0; k < N; k++)
	{
		for (int n = 0; n < N; n++)
			C[k] += f[n] * pow(M_E, ((-2 * M_PI * i * static_cast<std::complex<double>>(k) * ((double)n / (double)(N)))));
	}
	return C;
}

std::complex<double> *FFT(double*& f, const int N)
{
	std::complex<double>* C = new std::complex<double>[N];
	if (N == 1)
	{
		C[0] = f[0];
		return C;
	}
	const int h = N / 2;
	double* etemp = new double[h];
	double* otemp = new double[h];
	int oindex = 0;
	int eindex = 0;
	for (int n = 0; n < N; n++)
	{
		if (n % 2 == 0)
		{
			etemp[eindex] = f[n];
			eindex++;
		}
		else
		{
			otemp[oindex] = f[n];
			oindex++;
		}
	}

	std::complex<double>* E = FFT(etemp, h);
	std::complex<double>* O = FFT(otemp, h);

	for (int k = 0; k < h; k++)
	{
		C[k] = E[k] + pow(M_E, -2 * M_PI * ((double)k / (double)N) * i) * O[k];
		C[k + h] = E[k] - pow(M_E, -2 * M_PI * (((double)k) / (double)(N)) * i) * O[k];
	}
	return C;
}

int main()
{
	const int MAX_ORDER = 10;
	for (int o = 1; o <= MAX_ORDER; o++)
	{
		const int N = 1 << o;
		printf("N: %i\n", N);

		double* f = new double[N];
		for (int n = 0; n < N; n++)
			f[n] = n / (double)N;

		//pomiar dft
		clock_t t1 = clock();
		std::complex<double>* cDFT = DFT(f, N);
		clock_t t2 = clock();
		double dft_time = (t2 - t1) / (double)CLOCKS_PER_SEC * 1000.0;
		//wypisanie dft
		for (int n = 0; n < 10; n++)
			std::cout << std::setprecision(10) << cDFT[n] << std::endl;
		std::cout << "------" << std::endl;
		printf("DFT time [ms]: %f\n", dft_time);

		//pomiar fft
		t1 = clock();
		std::complex<double>* cFFT = FFT(f, N);
		t2 = clock();
		double fft_time = (t2 - t1) / (double)CLOCKS_PER_SEC * 1000.0;
		std::cout << std::endl;
		//wypisanie fft
		for (int n = 0; n < 10; n++)
			std::cout << std::setprecision(10) << cFFT[n] << std::endl;
		std::cout << "------" << std::endl;
		printf("FFT time [ms]: %f\n", fft_time);
			Sleep(1000);
		if (o < MAX_ORDER)
			system("cls");

		delete[] f;
		delete[] cDFT;
		delete[] cFFT;
	}
}

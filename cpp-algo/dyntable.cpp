#include <iostream>
#include <string>
#include <time.h>
#include <iomanip>

using namespace std;

template <class T>
class dynamicArray
{
public:
	T* arr;
	int numerOfElements;
	int maxSize;
	dynamicArray()
	{
		maxSize = 1;
		numerOfElements = 0;
		arr = new T[maxSize];
	}
	~dynamicArray();
	void Expand();
	void pushBack(T _data);
	void to_string(int _number);
	void findByIndex(int _index);
	void findAndChange(int _index, T _data);
	void deleteArray();
	void sortArray();
};


int main()
{
	dynamicArray <int> newArray;
	clock_t start, stop, START_ELEMENT, STOP_ELEMENT;
	const int power = 7;
	const int dataSize = pow(10, power);
	float worstTime = 0;
	float elementTime = 0;
	float fullTime = 0;
	float amortizedTime = 0;
	int _data = 0;
	start = clock();
	for (size_t i = 0; i < dataSize; i++)
	{
		START_ELEMENT = clock();
		_data = rand() % 10000 + 1;
		newArray.pushBack(_data);
		STOP_ELEMENT = clock();
		elementTime = (float)(STOP_ELEMENT - START_ELEMENT)/CLOCKS_PER_SEC;
		if (elementTime > worstTime)
		{
			worstTime = elementTime;
			cout << "Nowy najgorszy czas: (" << worstTime << "s) wystapil przy elemencie o indeksie " << i << endl;
		}
	}
	stop = clock();
	cout << "========================" << endl;
	newArray.to_string(10);
	fullTime = (float)(stop - start) / CLOCKS_PER_SEC;
	cout << "Dodanie " << dataSize << " elementow zajelo: (" << fullTime << " sekund)";
	amortizedTime = fullTime / dataSize;
	cout << endl << "Zamortyzowany czas wynosi: (" << amortizedTime << " sekund)" << endl;
}

 template<class T>
void dynamicArray<T>::Expand()
{
	maxSize = maxSize *2;
	T* temp = new T[maxSize];
	for (size_t i = 0; i < numerOfElements; i++)
	{
		temp[i] = arr[i];
	}
	delete[] arr;
	arr = temp;
}

template<class T>
void dynamicArray<T>::pushBack(T _data)
{
	if (numerOfElements >= maxSize)
	{
		Expand();
	}
	arr[numerOfElements] = _data;
	numerOfElements++;
}

template<class T>
void dynamicArray<T>::to_string(int _number)
{
	if (numerOfElements > 0)
	{
		for (int i = 0; i < _number; i++)
		{
			cout << "ID: " << i + 1 << " Dane: " << arr[i] << endl;
		}
		cout << "W tablicy znajduje sie aktualnie " << numerOfElements << " elementow. Powyzej wypisano " << _number << " z nich." << endl;
	}
	else
	{
		cout << "Lista jest pusta!" << endl;
	}
}

template<class T>
void dynamicArray<T>::findByIndex(int _index)
{
	for (size_t i = 0; i < numerOfElements; i++)
	{
		if (i == _index)
		{
			cout << arr[i] << endl;
			return;
		}
	}
	cout << "Blad elementu o podanym indeksie!" << endl;
}

template<class T>
void dynamicArray<T>::findAndChange(int _index, T _data)
{
	for (size_t i = 0; i < numerOfElements; i++)
	{
		if (i == _index)
		{
			arr[i] = _data;
			return;
		}
	}
	cout << "Blad elementu o podanym indeksie!" << endl;
}

template<class T>
void dynamicArray<T>::deleteArray()
{
	if (numerOfElements > 0)
	{
		numerOfElements = 0;
	}
	maxSize = 1;
	delete[] arr;
}

template<class T>
dynamicArray<T>::~dynamicArray()
{
	if (numerOfElements > 0)
	{
		numerOfElements = 0;
		maxSize = 1;
		delete[] arr;
	}
}

template<class T>
void dynamicArray<T>::sortArray()
{
	T temp;
	for (size_t j = 0; j < numerOfElements - 1; j++)
	{
		for (size_t i = 0; i < numerOfElements - 1; i++)
		{
			if (arr[i] > arr[i + 1])
			{
				temp = arr[i + 1];
				arr[i + 1] = arr[i];
				arr[i] = temp;
			}
		}
	}
}

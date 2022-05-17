#include <iostream>
#include <string>
#include <time.h>
#include <Windows.h>
#include <random>
#include <iomanip>

template <typename T>
class Item
{
public:
	T Value;
	Item* next;
	Item(T value) :
		Value(value),
		next(NULL) {}
};

template <typename T>
class binaryHeap
{
private:
	T* tab;
	int Size;
	int Elements;
	int p(int i) { return (i - 1) / 2; }
	int l(int i) { return (2 * i + 1); }
	int r(int i) { return (2 * i + 2); }
	void heapUp(int i, int* _tab, int _size);
	void heapDown(int i, int* _tab, int _size);
public:
	binaryHeap()
	{
		Size = 1;
		Elements = 0;
		tab = new T[Size];
	}
	binaryHeap(int*& _tab, int _size)
	{
		int i;
		for (i = 0; i < _size; i++)
			heapUp(i, _tab, _size);
	}
	~binaryHeap()
	{
		Size = 1;
		Elements = 0;
		delete[] tab;
	}
	void Expand();
	void Insert(T data);
	int Pop();
	void Print(int length);
	void Clear();
	void heapSort(int* _tab, int _size);
};

template<typename T>
T xMAX(T* _tab, int n)
{
	T max = 0;
	for (int i = 0; i < n; i++)
	{
		if (_tab[i] > max)
			max = _tab[i];
	}
	return max;
}

template <typename T>
T xMIN(T* _tab, T max, int n)
{
	T min = max;
	for (int i = 0; i < n; i++)
	{
		if (_tab[i] < min)
			min = _tab[i];
	}
	return min;
}

void countingSort(int*& _tab, int _size, const int m)
{
	int* count = new int[m];
	int index;
	for (int i = 0; i < m; i++)
		count[i] = 0;
	for (int i = 0; i < _size; i++)
	{
		index = _tab[i];
		count[index]++;
	}
	index = 0;
	for (int i = 0; i < m; i++)
	{
		for (int j = 1; j <= count[i]; j++)
		{
			_tab[index] = i;
			index++;
		}
	}
	delete[] count;
}

template<typename T>
void binaryHeap<T>::heapSort(int* _tab, int _size)
{
	for (int i = _size - 1; i > 0; i--)
	{
		int exRoot = _tab[0];
		_tab[0] = _tab[i];
		_tab[i] = exRoot;
		heapDown(0, _tab, i - 1);
	}
}

//bucketsort dla T
template <typename T>
void bucketSort(T*& _tab, const int n, const int m)
{
	//Obliczenie liczby kubelkow oraz rozpietosci przedzialow w kubelkach
	int numberOfBuckets = sqrt(n);
	T max = xMAX(_tab, n);
	T min = xMIN(_tab, max, n);
	T gap = (max - min) / (numberOfBuckets - 1);

	//Wyzerowanie kubelkow
	Item<T>** buckets = new Item<T>*[numberOfBuckets];
	for (int i = 0; i < numberOfBuckets; i++)
		buckets[i] = NULL;
	
	//Wstawianie elementow do kubelkow(od razu je sortujac)
	for (int i = 0; i < n; i++)
	{
		Item<T>* prev = NULL;
		Item<T>* element = buckets[(int)(_tab[i] / gap)];
		if (element == NULL)
		{
			element = new Item<T>(_tab[i]);
			buckets[(int)(_tab[i] / gap)] = element;
		}
		else
		{
			while (element != NULL && _tab[i] > element->Value)
			{
				prev = element;
				element = element->next;
			}
			if (prev == NULL)
			{
				element = new Item<T>(_tab[i]);
				element->next = buckets[(int)(_tab[i] / gap)];
				buckets[(int)(_tab[i] / gap)] = element;
			}
			else
			{
				element = new Item<T>(_tab[i]);
				element->next = prev->next;
				prev->next = element;
			}
		}
	}
	//Przepisanie kubelkow do tablicy
	int ind = 0;
	for (int i = 0; i < numberOfBuckets; i++)
	{
		while (buckets[i] != NULL)
		{
			_tab[ind] = buckets[i]->Value;
			ind++;
			buckets[i] = buckets[i]->next;
		}
	}

	//Zwolnienie pamieci
	for (int i = 0; i < numberOfBuckets; i++)
	{
		while (buckets[i] != NULL)
		{
			Item<T>* temp = buckets[i];
			buckets[i] = temp->next;
			delete temp;
		}
	}
	delete[] buckets;
}

int main()
{
	srand(time(NULL));	
	const int n = 10;
	const int m = 80;
	int* tab = new int[n];
	int* tab1 = new int[n];
	int* tab2 = new int[n];
	for (int i = 0; i < n; i++)
		tab[i] = ((rand() << 15) + rand()) % m;
	memcpy(tab1, tab, n * sizeof(int));
	memcpy(tab2, tab, n * sizeof(int));

	//COUNTINGSORT
	std::cout << "---COUNTINGSORT---\n";
	std::cout << "\n///przed///\n";
	for (int i = 0; i < n; i++)
		std::cout << tab[i] << " ";
	countingSort(tab, n, m);
	std::cout << "\n";
	std::cout << "\n///po///\n";
	for (int i = 0; i < n; i++)
		std::cout << tab[i] << " ";
	std::cout << "\n--------------------------";

	//HEAPSORT
	//kopiec z tablicy
	std::cout << "\n\n---HEAPSORT---\n";
	std::cout << "\n///przed///\n";
	for (int i = 0; i < n; i++)
		std::cout << tab1[i] << " ";
	binaryHeap<int>heap(tab1, n);
	//print kopca z tablicy
	std::cout << "\n\n///kopiec z tablicy///\n";
	for (int i = 0; i < n; i++)
		std::cout << tab1[i] << " ";
	//sortowanko
	heap.heapSort(tab1, n);
	std::cout << "\n";
	//print posortowanego
	std::cout << "\n///po///\n";
	for (int i = 0; i < n; i++)
		std::cout << tab1[i] << " ";
	std::cout << "\n--------------------------\n";

	//BUCKETSORT
	std::cout << "\n\n---BUCKETSORT---\n";
	std::cout << "\n///przed///\n";
	for (int i = 0; i < n; i++)
		std::cout << tab2[i] << " ";
	bucketSort(tab2, n, m);
	std::cout << "\n///po///\n";
	for (int i = 0; i < n; i++)
		std::cout << tab2[i] << " ";
	std::cout << "\n--------------------------\n";

	//BUCKETSORT FLOAT NUMBERS
	float* ftab = new float[n];
	for (int i = 0; i < n; i++)
		ftab[i] = rand() / (double)RAND_MAX;
	std::cout << "\n\n---BUCKETSORT---\n";
	std::cout << "\n///przed///\n";
	for (int i = 0; i < n; i++)
		std::cout << ftab[i] << " ";
	bucketSort(ftab, n, m);
	std::cout << "\n///po///\n";
	for (int i = 0; i < n; i++)
		std::cout << ftab[i] << " ";
	std::cout << "\n--------------------------\n";
	delete[] ftab;
	delete[] tab2;
	delete[] tab1;
	delete[] tab;
}





//METODY KOPCA BINARNEGO
template<typename T>
void binaryHeap<T>::heapUp(int i, int* _tab, int _size)
{
	int j = p(i);
	if (i > 0 && _tab[j] < _tab[i])
	{
		T temp = _tab[i];
		_tab[i] = _tab[j];
		_tab[j] = temp;
		heapUp(p(i), _tab, _size);
	}
}

template<typename T>
void binaryHeap<T>::heapDown(int i, int* _tab, int _size)
{
	int left, right, maxElem;
	left = l(i);
	right = r(i);
	maxElem = i;
	if (left <= _size && _tab[left] > _tab[maxElem])
		maxElem = left;
	if (right <= _size && _tab[right] > _tab[maxElem])
		maxElem = right;
	if (maxElem != i)
	{
		int temp = _tab[i];
		_tab[i] = _tab[maxElem];
		_tab[maxElem] = temp;
		heapDown(maxElem, _tab, _size);
	}
}

template<typename T>
void binaryHeap<T>::Expand()
{
	Size *= 2;
	T* temp = new T[Size];
	for (size_t i = 0; i < Elements; i++)
	{
		temp[i] = tab[i];
	}
	delete[] tab;
	tab = temp;
}

template<typename T>
void binaryHeap<T>::Insert(T data)
{
	if (Elements >= Size)
		Expand();
	tab[Elements] = data;
	Elements++;
	heapUp(Elements - 1);
}

template<typename T>
int binaryHeap<T>::Pop()
{
	if (Elements == 0)
		return NULL;
	int popElem = tab[0];
	tab[0] = tab[Elements - 1];
	Elements--;
	heapDown(0);
	return popElem;
}

template<typename T>
void binaryHeap<T>::Print(int length)
{
	for (size_t i = 0; i < Elements; i++)
	{
		if (i == length)
			break;
		std::cout << i + 1 << ". " << tab[i] << std::endl;
	}
}

template<typename T>
void binaryHeap<T>::Clear()
{
	Size = 1;
	if (Elements != 0)
		Elements = 0;
	tab = new T[Size];
}


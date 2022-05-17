#include <iostream>
#include <string>
#include <time.h>
#include <Windows.h>
#include <random>

using namespace std;

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
	void heapUp(int i);
	void heapDown(int i);
public:
	binaryHeap()
	{
		Size = 1;
		Elements = 0;
		tab = new T[Size];
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
};

int main()
{
	int MAX = 7;
	for (int i = 0; i < MAX; i++)
	{
	    binaryHeap<int>heap;
		const int n = pow(10, i);
		//DODAWANIE
		//losowe liczby
		random_device rd;
		mt19937 mt(rd());
		uniform_int_distribution<int> dist(1, 10000000);
		clock_t START_ADD = clock();
		for (int j = 0; j < n; j++)
		{
			heap.Insert(dist(mt));
		}
		clock_t STOP_ADD = clock();
		double ADD_TIME = (double)(STOP_ADD - START_ADD) / CLOCKS_PER_SEC;
		double AVG_ADD_TIME = (double)(ADD_TIME / n);
		system("cls");
		cout << "Dodano: " << n << " elementow" << endl;
		cout << "Calkowity czas: " << ADD_TIME << " sekund" << endl;
		cout << "Sredni czas na element: " << AVG_ADD_TIME << " sekund" << endl;
		cout << "Aktualna postac kopca (pierwsze 10 elementow): " << endl;
		heap.Print(10);
		//USUWANIE
		clock_t START_POP = clock();
		for (int k = 0; k < n; k++)
		{
			int temp = heap.Pop();
		}
		clock_t STOP_POP = clock();
		double POP_TIME = (double)(STOP_POP - START_POP) / CLOCKS_PER_SEC;
		double AVG_POP_TIME = (double)(POP_TIME / n);
		cout << "Calkowity czas pobrania calosci kopca: " << POP_TIME << endl;
		cout << "Sredni czas pobrania elementu: " << AVG_POP_TIME << endl;
		Sleep(2000);
		heap.Clear();
	}
	
}

template<typename T>
void binaryHeap<T>::heapUp(int i)
{
	int j = p(i);
	if (i > 0 && tab[j] < tab[i])
	{
		T temp = tab[i];
		tab[i] = tab[j];
		tab[j] = temp;
		heapUp(p(i));
	}
}

template<typename T>
void binaryHeap<T>::heapDown(int i)
{
	int left, right, maxElem;
	left = l(i);
	right = r(i);
	maxElem = i;
	if (left <= Elements && tab[left] > tab[maxElem])
		maxElem = left;
	if (right <= Elements && tab[right] > tab[maxElem])
		maxElem = right;
	if (maxElem != i)
	{
		T temp = tab[i];
		tab[i] = tab[maxElem];
		tab[maxElem] = temp;
		heapDown(maxElem);
	}
}

template<typename T>
void binaryHeap<T>::Expand()
{
	Size*=2;
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
		cout << i+1 << ". " << tab[i] << endl;
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

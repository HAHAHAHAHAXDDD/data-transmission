#include <iostream>
#include <time.h>
#include <string>
#include <iterator>
#include <Windows.h>

using namespace std;

string keyGenerator()
{
	string key;
	const char let[] = { "abcdefghijklmnopqrstuvwxyz" };
	for (int i = 0; i < 6; i++)
		key += let[rand() % (sizeof(let) - 1)];
	return key;
}

template<typename K, typename V>
class Item
{
private:
	K Key;
	V Value;
	Item* next;
public:
	Item(K key, V value) :
		Key(key),
		Value(value),
		next(NULL) {}
	template<class K, class V>
	friend class hashTable;
};

template<class K, class V>
class hashTable
{
private:
	Item<K, V>** items;
	int Size;
	int Elements;
public:
	hashTable()
	{
		Size = 1;
		Elements = 0;
		items = new Item<K, V>*[Size];
		for (int i = 0; i < Size; i++)
			items[i] = NULL;
	}
	void rehash();
	int hashFunction(string key);
	void addElement(string key, int value);
	Item<K, V>* findElement(string key)
	{
		int hash = hashFunction(key);
		Item<K, V>* item = items[hash];
		if (item == NULL)
			return NULL;
		while (item != NULL)
		{
			if (item->Key == key)
				return item;
			item = item->next;
			if (item == NULL)
				return NULL;
		}
	}
	bool removeElement(string key);
	void printTable(int length);
	void removeTable();
	void listStats();
};

int main()
{
	srand(time(NULL));
	hashTable<string, int> newTable;
	const int LIMIT = 6;
	string key;
	for (int i = 0; i <= LIMIT; i++)
	{
		//DODAWANIE ELEMENTOW
		int n = pow(10, i);
		clock_t START_ADD = clock();
		for (int j = 0; j < n; j++)
		{
			newTable.addElement(keyGenerator(), j);
		}
		clock_t STOP_ADD = clock();
		double ADD_TIME = (double)(STOP_ADD - START_ADD) / CLOCKS_PER_SEC;
		double AVG_ADD_TIME = (double)(ADD_TIME / n);
		system("cls");
		cout << "Dodano: " << n << " elementow" << endl;
		cout << "Calkowity czas: " << ADD_TIME << " sekund" << endl;
		cout << "Sredni czas na element: " << AVG_ADD_TIME << " sekund" << endl;
		cout << "Aktualna postac drzewa (pierwsze 10 elementow): " << endl;
		newTable.printTable(10);

		//WYSZUKIWANIE ELEMENTOW
		int m = pow(10, 4);
		int hits = 0;
		clock_t START_SEARCH = clock();
		for (int p = 0; p < m; p++)
		{
			if (newTable.findElement(keyGenerator()))
				hits++;
		}
		clock_t STOP_SEARCH = clock();
		double SEARCH_TIME = (double)(STOP_SEARCH - START_SEARCH) / CLOCKS_PER_SEC;
		double AVG_SEARCH_TIME = (double)(SEARCH_TIME / hits);
		cout << "Liczba prob znalezienia elementu: " << m << endl;
		cout << "Liczba trafien: " << hits << endl;
		cout << "Calkowity czas: " << SEARCH_TIME << " sekund" << endl;
		cout << "Srednio na element " << AVG_SEARCH_TIME << " sekund" << endl;
		Sleep(2000);
		newTable.removeTable();
	}
}

template<class K, class V>
int hashTable<K, V>::hashFunction(string key)
{
	size_t elem = 0;
	size_t hash = 0;
	for (int i = 0; i < key.size(); i++)
	{
		elem = pow(key[i] * 31, key.size() - (i + 1));
		hash += elem;
	}
	return hash % Size;
}

template<class K, class V>
void hashTable<K, V>::rehash()
{
	Item<K, V>** temp = items;
	int oldSize = Size;
	Item<K, V>* item;
	Size *= 2;
	items = new Item<K, V>*[Size];
	for (int i = 0; i < Size; i++)
		items[i] = NULL;
	Elements = 0;
	for (int i = 0; i < oldSize; i++)
	{
		item = temp[i];
		if (item != NULL)
		{
			if (item->next != NULL)
			{
				while (item != NULL)
				{
					addElement(item->Key, item->Value);
					item = item->next;
				}
			}
			else
				addElement(item->Key, item->Value);
		}
	}
}

template<class K, class V>
void hashTable<K, V>::addElement(string key, int value)
{
	if (Elements >= 0.75 * Size)
		rehash();
	int hashValue = hashFunction(key);
	Item<K, V>* prev = NULL;
	Item<K, V>* item = items[hashValue];
	while (item != NULL && item->Key != key)
	{
		prev = item;
		item = item->next;
	}
	if (item == NULL)
	{
		item = new Item<K, V>(key, value);
		if (prev == NULL)
			items[hashValue] = item;
		else
			prev->next = item;
		Elements++;
	}
	else
		item->Value = value;
}

template<class K, class V>
void hashTable<K, V>::printTable(int length)
{
	int check = 0;
	Item<K, V>* item;
	cout << "hash table: " << endl;
	cout << "   Number of elements: " << Elements << endl;
	cout << "   Size: " << Size << endl;
	cout << "   table: " << endl << "   {" << endl;
	for (int i = 0; i < Size; i++)
	{
		item = items[i];
		if (item != NULL)
		{
			if (item->next != NULL)
			{
				cout << "\t" << i << ": " << item->Key << " -> " << item->Value << ", ";
				check++;
				if (check >= length)
				{
					cout << endl << "\t..." << endl;
					break;
				}
				item = item->next;
				while (item != NULL)
				{
					if (item->next != NULL)
					{
						cout << item->Key << " -> " << item->Value << ", ";
						check++;
						if (check >= length)
						{
							cout << endl << "\t..." << endl;
							break;
						}
					}
					else
					{
						cout << item->Key << " -> " << item->Value << ";" << endl;
						check++;
						if (check >= length)
						{
							cout << "\t..." << endl;
							break;
						}
					}
					item = item->next;
				}
			}
			else
			{
				cout << "\t" << i << ": " << item->Key << " -> " << item->Value << ";" << endl;
				check++;
				if (check >= length)
				{
					cout << "\t..." << endl;
					break;
				}
			}
		}
		if (check >= length)
			break;
	}
	cout << "   }" << endl;
	cout << "   stats:" << endl;
	listStats();
}

template<class K, class V>
bool hashTable<K, V>::removeElement(string key)
{
	if (!findElement(key))
		return false;

	int hash = hashFunction(key);
	Item<K, V>* item = items[hash];
	Item<K, V>* prev = NULL;
	while (item != NULL)
	{
		if (item->Key == key)
		{
			//Element pierwszy na liscie
			if (prev == NULL)
			{
				items[hash] = item->next;
				delete item;
				item = NULL;
				Elements--;
				return true;
			}
			//Element w srodku listy
			if (prev != NULL && item->next != NULL)
			{
				prev->next = item->next;
				delete item;
				item = NULL;
				Elements--;
				return true;
			}
			//Element na koncu listy
			if (item->next == NULL)
			{
				prev->next = NULL;
				delete item;
				item = NULL;
				Elements--;
				return true;
			}
		}
		prev = item;
		item = item->next;
		if (item == NULL)
			return false;
	}
}

template<class K, class V>
void hashTable<K, V>::removeTable()
{
	for (int i = 0; i < Size; i++)
	{
		while (items[i] != NULL)
		{
			Item<K, V>* temp = items[i];
			items[i] = temp->next;
			delete temp;
			Elements--;
		}
	}
	if (Size != 1) Size = 1;
	if (Elements > 0) Elements = 0;
}

template<class K, class V>
void hashTable<K, V>::listStats()
{
	int nonNull = 0;
	int maxListSize = 0;
	int minListSize = 0;
	double avgListSize = 0;
	int p = 0;
	int k = 0;

	Item<K, V>* item;
	for (int i = 0; i < Size; i++)
	{
		item = items[i];
		if (item != NULL)
		{
			if (item->next != NULL)
			{
				while (item != NULL)
				{
					item = item->next;
					p++;
				}
				if (p > maxListSize)
					maxListSize = p;
			}
			else
				k++;
			if (maxListSize < 1 && Size > 0)
				maxListSize = 1;
			if (k > 0)
				minListSize = 1;
			nonNull++;
			p = 0;
		}
	}
	avgListSize = ((double)Elements) / ((double)nonNull);
	if (minListSize == 1)
		cout << "     list min size: " << minListSize << endl;
	else
		cout << "     list min size: >1" << endl;
	cout << "     list max size: " << maxListSize << endl;
	cout << "     non-null lists: " << nonNull << endl;
	cout << "     list avg size: " << avgListSize << endl;
}
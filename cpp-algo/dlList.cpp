#include <stdio.h>
#include <iostream>
#include <stdbool.h>
#include <string.h>
#include <time.h>

using namespace std;

class List
{
public:
    int id;
    int data;
    char ch;
    List* next = NULL;
    List* prev = NULL;
    List* head = NULL;
    List();
    ~List();
    void pushFront(int _data, char _ch);
    void to_String(int _number);
    void pushBack(int _data, char _ch);
    void popBack();
    void popFront();
    void deleteList();
    void findByIndex(int _id);
    void changeDataByIndex(int _id, int _data, char _ch);
    List* findElement(int _data, char _ch);
    bool findAndDelete(int _data, char _ch);
    void addElementByIndex(int _id, int _data, char _ch);
};

int main()
{
    List newList;
    clock_t start, stop;
    float time, timePerElement;
    const int ADD_ELEMENTS = 100;
    const int NUMBER_OF_TESTS = 100;
    int _data;
    int n = 0;
    char _ch;
    for (int p = 0; p <= NUMBER_OF_TESTS; p++)
    {
        // == WCZYTYWANIE DO LISTY CORAZ WIEKSZEJ ILOSCI ELEMENTOW ==
        start = clock();
        for (int i = 0; i < n; i++)
        {
            _ch = 97 + rand() % 26;
            _data = rand() % 10000 + 1;
            newList.pushBack(_data, _ch);
        }
        stop = clock();
        time = (float)(stop - start) / CLOCKS_PER_SEC;
        timePerElement = time / n;
        newList.to_String(100);
        cout << "Wczytanie " << n << " plikow do listy zajelo (" << time << " sekund)" << "\nCo daje srednio: (" << timePerElement << " sekund) na element" << endl << endl;
        n = n + ADD_ELEMENTS;

        // == PROBA ZNALEZIENIA I USUNIECIA ELEMENTOW Z POMIAREM CZASOWYM ==
        const int NUMBER_OF_TRIES = 10;
        bool isDeleted = false;
        int numberOfDeletedElements = 0;
        clock_t start2 = clock();
        for (int x = 0; x < NUMBER_OF_TRIES; x++)
        {
            if (newList.findAndDelete(rand() % 10000 + 1, 97 + rand() % 26) == true)
            {
                isDeleted = true;
            }
            if (isDeleted == true)
            {
                numberOfDeletedElements++;
            }
        }
        clock_t stop2 = clock();
        time = (float)(stop2 - start2) / CLOCKS_PER_SEC;
        timePerElement = time / numberOfDeletedElements;
        if (numberOfDeletedElements > 0)
        {
            cout << "Znaleziono i usunieto lacznie " << numberOfDeletedElements << " elementow, zajelo to: (" << time << " sekund)\nCo daje srednio: (" << timePerElement << " sekund) na element" << endl << endl;
        }
        newList.deleteList();
    }
    newList.deleteList();
}

List::List()
    : id(0)
    , data(0)
    , ch('c')
{}

List::~List()
{
    if (head != NULL)
    {
        List* current = head->next;
        List* temp;
        while (current != NULL)
        {
            temp = current->next;
            delete current;
            current = temp;
        }
        delete head;
        head = NULL;
    }
}

void List::pushFront(int _data, char _ch)
{
    List* newElement = new List();
    newElement->data = _data;
    newElement->ch = _ch;

    newElement->next = head;
    newElement->prev = NULL;
    newElement->id = 0;

    if (head != NULL)
    {
        head->prev = newElement;
        List* current = head;
        while (current != NULL)
        {
            current->id++;
            current = current->next;
        }
    }
    head = newElement;

}

void List::to_String(int _number)
{
    int numberOfElements = 0;
    List* current = head;
    if (head != NULL && _number>0)
    {
        while (current != NULL)
        {
            if (numberOfElements < _number)
            {
                cout << "ID: " << current->id + 1 << " " << current->data << " " << current->ch << endl;
            }
            if (numberOfElements == _number)
            {               
                cout << "..." << endl;
            }
            current = current->next;
            numberOfElements++;
        }
        cout << "Cala lista zawiera " << numberOfElements << " elementow powyzej przedstawiono " << _number << " z nich" << endl;
    }
    else
    {
        cout << "Lista jest pusta!" << endl;
    }
}

void List::pushBack(int _data, char _ch)
{
    List* newElement = new List();
    List* current = head;
    newElement->data = _data;
    newElement->ch = _ch;

    if (head == NULL)
    {
        newElement->prev = NULL;
        newElement->next = NULL;
        head = newElement;
        newElement->id = 0;
    }
    else
    {
        while (current->next != NULL)
        {
            current = current->next;
        }
        newElement->next = NULL;
        newElement->id = current->id + 1;
        current->next = newElement;
        newElement->prev = current;
    }
}

void List::popBack()
{
    if (head != NULL)
    {
        List* current = head;

        while (current->next != NULL)
        {
            current = current->next;
        }
        current->prev->next = NULL;
        delete current;
        current = NULL;
    }
}

void List::popFront()
{
    List* current;
    if (head != NULL)
    {
        if (head->next != NULL)
        {
            current = head;
            head = current->next;
            head->prev = NULL;
            delete current;
            current = NULL;
            current = head;
            while (current != NULL)
            {
                current->id--;
                current = current->next;
            }
        }
        if (head->next == NULL)
        {
            deleteList();
        }

    }
}

void List::deleteList()
{
    if (head != NULL)
    {
        List* current = head->next;
        List* temp;
        while (current != NULL)
        {
            temp = current->next;
            delete current;
            current = temp;
        }
        delete head;
        head = NULL;
    }
}

void List::findByIndex(int _id)
{
    List* current = head;
    while (current != NULL)
    {
        if (current->id == _id)
        {
            cout << "ID: " << current->id << " " << current->data << " " << current->ch << endl;
            return;
        }
        current = current->next;
    }
    cout << "Brak elementu o podanym indeksie!" << endl;
}

void List::changeDataByIndex(int _id, int _data, char _ch)
{
    List* current = head;
    while (current != NULL)
    {
        if (current->id == _id)
        {
            current->data = _data;
            current->ch = _ch;
            return;
        }
        current = current->next;
    }
    cout << "Brak elementu o podanym indeksie!" << endl;
}

List* List::findElement(int _data, char _ch)
{
    List* current = head;
    while (current != NULL)
    {
        if ((current->data == _data) && (current->ch == _ch))
        {
            return current;
        }
        current = current->next;
    }
    return NULL;
}

bool List::findAndDelete(int _data, char _ch)
{
    List* current = head;
    List* temp, * sort;
   // bool isDeleted = false;
    while (current != NULL)
    {
        if ((current->data == _data) && (current->ch == _ch))
        {
            if (current->prev == NULL)
            {
                popFront();
               // isDeleted = true;
                return true;
            }
            if (current->next == NULL)
            {
                popBack();
               // isDeleted = true;
                return true;
            }
            temp = current;
            sort = current;
            while (sort != NULL)
            {
                sort->id--;
                sort = sort->next;
            }
            (current->prev)->next = current->next;
            (current->next)->prev = current->prev;
            delete temp;
            delete sort;
            temp = NULL;
            sort = NULL;
           // isDeleted = true;
            current = head;
            return true;
        }
        current = current->next;
    }
    return false;
}

void List::addElementByIndex(int _id, int _data, char _ch)
{
    List* current = head;
    List* sort;
    if (head == NULL || current->prev == NULL)
    {
        pushFront(_data, _ch);
        return;
    }
    while (current != NULL)
    {
        if (current->id == _id)
        {
            List* newElement = new List();
            newElement->data = _data;
            newElement->ch = _ch;
            newElement->prev = current->prev;
            newElement->next = current;
            current->prev = newElement;
            (newElement->prev)->next = newElement;
            newElement->id = current->id;
            sort = current;
            while (sort != NULL)
            {
                sort->id++;
                sort = sort->next;
            }
            return;
        }
        current = current->next;
    }
    if (current == NULL)
    {
        pushBack(_data, _ch);
        return;
    }
}

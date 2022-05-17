#include <iostream>
#include <time.h>
#include <string>

using namespace std;
enum Color
{
	BLACK,
	RED
};

string toString(Color col)
{
	if (col == BLACK)
		return "black";
	if (col == RED)
		return "red";
}

template <typename T>
struct Node
{
	Node* left;
	Node* right;
	Node* up;
	Color color;
	T data;
};

template <class T>
class BRT
{
private:
	Node<T>* root = NULL;
	int size = 0;
	void rotateLeft(Node<T>* p, Node<T>* elem);
	void rotateRight(Node<T>* p, Node<T>* elem);
public:
	BRT()
	{
		root = NULL;
		size = 0;
	}
	Node<T>* findElement(T _data)
	{
		bool found = false;
		Node<T>* current = root;
		while (current != NULL && !found)
		{
			if (current->data == _data)
				found = true;
			else if (current->data < _data)
				current = current->right;
			else
				current = current->left;
		}
		return current;
	}
	void addElement(T _data);
	void printTree();
	void preOrder(Node<T>* root);
	void inOrder(Node<T>* root);
	Node<T>* getRoot() { return root; }
	void deleteTree(Node<T>* root);
	string to_String(Node<T>* element);
	int treeHeight(Node<T>* root);
};


int main()
{
	BRT <int> newTree;
	srand(time(NULL));
	const int MAX_ORDER = 7;
	double time_;
	for (int i = 0; i <= MAX_ORDER; i++)
	{
		int n = pow(10, i);

		clock_t s1 = clock();
		int _data;
		for (int j = 0; j < n; j++)
		{
			_data = rand() % 9999999 + 1;
			newTree.addElement(_data);
		}
		clock_t f1 = clock();
		time_ = (double)(f1 - s1) / CLOCKS_PER_SEC;
		double timePerElem = time_ / n;
		cout << "Dodanie " << n << " elementow do drzewa zajelo: (" << time_ << " sekund, " << timePerElem << "s srednio na element)" << endl;
		cout << "Wysokosc drzewa " << newTree.treeHeight(newTree.getRoot()) << endl;
	    int w = pow(10, 4);
		int hits = 0;
		int _data2;
		srand(time(NULL));
		clock_t s2 = clock();
		for (int p = 0; p < w; p++)
		{
			_data2 = rand() % 9999999 + 1;
			if (newTree.findElement(_data2))
				hits++;
		}
		clock_t f2 = clock();
		double timeS = (double)(f2 - s2) / CLOCKS_PER_SEC;
		cout << "Czas przeszukiwania drzewa: " << timeS << endl;
		cout << "Znalezionych elementow: " << hits << endl << endl;
	}
	newTree.deleteTree(newTree.getRoot());
}

template <class T>
void BRT<T>::addElement(T _data)
{

	Node<T>* newElement = new Node<T>();
	newElement->data = _data;
	newElement->left = NULL;
	newElement->right = NULL;
	if (root == NULL)
	{
		root = newElement;
		newElement->up = NULL;
		size++;
		return;
	}
	Node<T>* current = root;
	while (current != NULL)
	{
		if (_data < current->data)
		{
			if (current->left == NULL)
			{
				newElement->up = current;
				current->left = newElement;
				size++;
				break;
			}
			current = current->left;
		}
		else if (_data > current->data)
		{
			if (current->right == NULL)
			{
				newElement->up = current;
				current->right = newElement;
				size++;
				break;
			}
			current = current->right;
		}
		else if (_data == current->data)
		{
			return;
		}
	}
	newElement->color = RED;
	Node<T>* temp;
	Node<T>* temp2;
	while ((newElement->data != root->data) && (newElement->up->color == RED))
	{
		if (newElement->up == newElement->up->up->left)
		{
			temp = newElement->up->up->right;
			if (temp != NULL)
			{
				if (temp->color == RED)
				{
					newElement->up->color = BLACK;
					temp->color = BLACK;
					newElement->up->up->color = RED;
					newElement = newElement->up->up;
				}
				else if (temp->color == BLACK)
				{
					if (newElement == newElement->up->right)
					{
						temp2 = newElement->up;
						rotateLeft(newElement->up, newElement);
						newElement = temp2;
					}
					newElement->up->color = BLACK;
					newElement->up->up->color = RED;
					rotateRight(newElement->up->up, newElement->up);
				}
			}
			else if (temp == NULL)
			{
				if (newElement == newElement->up->right)
				{
					temp2 = newElement->up;
					rotateLeft(newElement->up, newElement);
					newElement = temp2;
				}
				newElement->up->color = BLACK;
				newElement->up->up->color = RED;
				rotateRight(newElement->up->up, newElement->up);
			}
		}
		else if (newElement->up == newElement->up->up->right)
		{
			temp = newElement->up->up->left;
			if (temp != NULL)
			{
				if (temp->color == RED)
				{
					newElement->up->color = BLACK;
					temp->color = BLACK;
					newElement->up->up->color = RED;
					newElement = newElement->up->up;
				}
				else if (temp->color == BLACK)
				{
					if (newElement == newElement->up->left)
					{
						temp2 = newElement->up;
						rotateRight(newElement->up, newElement);
						newElement = temp2;
					}
					newElement->up->color = BLACK;
					newElement->up->up->color = RED;
					rotateLeft(newElement->up->up, newElement->up);
				}
			}
			else if (temp == NULL)
			{
				if (newElement == newElement->up->left)
				{
					temp2 = newElement->up;
					rotateRight(newElement->up, newElement);
					newElement = temp2;
				}
				newElement->up->color = BLACK;
				newElement->up->up->color = RED;
				rotateLeft(newElement->up->up, newElement->up);
			}
		}
	}

	root->color = BLACK;
}

template <class T>
void BRT<T>::printTree()
{
	cout << "red black tree: " << endl;
	cout << "   size: " << size << endl << "{" << endl;
	if (size > 0)
	{
		cout << "([" << toString(root->color) << ", p: " << to_String(root->up) << ", l: " << to_String(root->left) << ", r: " << to_String(root->right) << "], " << root->data << ")" << endl;
		preOrder(root->left);
		preOrder(root->right);
	}
	cout << "}" << endl;
}

template <class T>
void BRT<T>::preOrder(Node<T>* root)
{
	if (root != NULL)
	{
		cout << "([" << toString(root->color) << ", p: " << to_String(root->up) << ", l: " << to_String(root->left) << ", r: " << to_String(root->right) << "], " << root->data << ")" << endl;
		preOrder(root->left);
		preOrder(root->right);
	}
	else
		return;
}

template <class T>
void BRT<T>::inOrder(Node<T>* root)
{
	if (root != NULL)
	{
		inOrder(root->left);
		cout << root->data << endl;
		inOrder(root->right);
	}
	else
		return;
}

template<class T>
void BRT<T>::rotateLeft(Node<T>* p, Node<T>* elem)
{
	if (elem->left != NULL)
	{
		elem->left->up = p;
		p->right = elem->left;
	}
	if (p->up == NULL)
	{
		elem->up = NULL;
		root = elem;
	}
	else if (p->up->left == p)
	{
		elem->up = p->up;
		p->up->left = elem;
	}
	else
	{
		elem->up = p->up;
		p->up->right = elem;
	}
	elem->left = p;
	p->up = elem;
	if (p->right == p->up)
		p->right = NULL;
	if (p->left == p->up)
		p->left = NULL;
}

template<class T>
void BRT<T>::rotateRight(Node<T>* p, Node<T>* elem)
{
	if (elem->right != NULL)
	{
		elem->right->up = p;
		p->left = elem->right;
	}
	if (p->up == NULL)
	{
		elem->up = NULL;
		root = elem;
	}
	else if (p->up->right == p)
	{
		elem->up = p->up;
		p->up->right = elem;
	}
	else
	{
		elem->up = p->up;
		p->up->left = elem;
	}
	elem->right = p;
	p->up = elem;
	if (p->right == p->up)
		p->right = NULL;
	if (p->left == p->up)
		p->left = NULL;
}

template <class T>
void BRT<T>::deleteTree(Node<T>* root)
{
	if (root == NULL)
		return;
	deleteTree(root->left);
	deleteTree(root->right);
	size--;
	delete root;
}

template<class T>
string BRT<T>::to_String(Node<T>* element)
{
	if (element == NULL)
		return "NULL";
	else
		return to_string(element->data);
}

template <class T>
int BRT<T>::treeHeight(Node<T>* root)
{
	if (root != NULL)
	{
		int leftSubtree, rightSubtree;
		leftSubtree = treeHeight(root->left);
		rightSubtree = treeHeight(root->right);
		return 1 + (leftSubtree > rightSubtree ? leftSubtree : rightSubtree);
	}
	else
		return 0;
}
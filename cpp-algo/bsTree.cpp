#include <iostream>
#include <string>
#include <time.h>

using namespace std;

template <class T>
class BST
{
private:
	T data;
	int size = 0;
	BST* left = NULL;
	BST* right = NULL;
	BST* up = NULL;
	BST* root = NULL;
public:
	void addElement(T _data);
	void printTree();
	BST* getRoot() { return root; }
	BST* findElement(T _data)
	{
		bool found = false;
		BST* current = root;
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
	bool deleteElement(BST* element);
	void deleteTree(BST* root);
	int treeHeight(BST* root);
	void preOrder(BST* root);
	void inOrder(BST* root);
};



int main()
{
	BST <int> newTree;
	const int pot = 7;
	double time;
	for (int i = 0; i <= pot; i++)
	{
		const int n = pow(10, i);
		//DODAWANIE ELEMENTOW DO DRZEWA
		clock_t s1 = clock();
		for (int j = 1; j < n; j++)
		{
			newTree.addElement(rand() % 10000 + 1);
		}
		clock_t f1 = clock();
		time = (double)(f1 - s1) / CLOCKS_PER_SEC;
		double timePerElement = time / n;
		cout << "Dodanie " << n << " elementow do drzewa zajelo: (" << time << " sekund, " << timePerElement << "s srednio na element)" << endl;
		cout << "Wysokosc drzewa " << newTree.treeHeight(newTree.getRoot()) << endl;
		//WYSZUKIWANIE I USUWANIE ELEMENTOW W DRZEWIE
		const int w = pow(10, 4);
		int hits = 0;
		int _data;
		clock_t s2 = clock();
		for (int p = 0; p < w; p++)
		{
			_data = rand() % 10000 + 1;
			if (newTree.findElement(_data))
			{
				hits++;
				newTree.deleteElement(newTree.findElement(_data));
			}
		}
		clock_t f2 = clock();
		double timeS = (double)(f2 - s2) / CLOCKS_PER_SEC;
		cout << "Czas przeszukiwania drzewa: " << timeS << endl;
		cout << "Znalezionych elementow: " << hits << endl << endl;
	}
	newTree.deleteTree(newTree.getRoot());
}


template<class T>
void BST<T>::printTree()
{
	if (size > 0)
	{
		cout << "binary search tree:" << endl;
		cout << "\tsize: " << size << endl;
		cout << "\theight: " << treeHeight(root) << endl;
		cout << "{" << endl;

		if (root->right != NULL && root->left != NULL)
			cout << "( [p: NULL, l: " << root->left->data << ", r: " << root->right->data << "], data: " << root->data << ")," << endl;
		if (root->right == NULL && root->left != NULL)
			cout << "( [p: NULL, l: " << root->left->data << ", r: NULL], data: " << root->data << ")," << endl;
		if (root->right != NULL && root->left == NULL)
			cout << "( [p: NULL, l: NULL, r: " << root->right->data << "], data: " << root->data << ")," << endl;
		if (root->right == NULL && root->left == NULL)
			cout << "( [p: NULL, l: NULL, r: NULL], data: " << root->data << ")," << endl;
		preOrder(root->left);
		preOrder(root->right);
		cout << "}" << endl;
	}
	else
	{
		return;
	}
}

template<class T>
void BST<T>::addElement(T _data)
{
	BST* newElement = new BST();
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
	else
	{
		BST* current = root;
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
				break;
			}
		}
	}
}

template<class T>
bool BST<T>::deleteElement(BST* element)
{
	bool deleted = false;
	BST* current = element;
	if (element != NULL)
	{
		// USUWANY ELEMENT JEST LISCIEM
		if (element->right == NULL && element->left == NULL)
		{
			if (element->data > element->up->data)
			{
				(current->up)->right = NULL;
				size--;
				delete element;
				element = NULL;
				deleted = true;
				return deleted;
			}
			if (element->data < element->up->data)
			{
				(current->up)->left = NULL;
				size--;
				delete element;
				element = NULL;
				deleted = true;
				return deleted;
			}
		}
		// USUWANY ELEMENT 1 STOPNIA
		if ((element->right != NULL || element->left != NULL) && element->data != root->data)
		{
			current = element->up;
			if (element->data < current->data)
			{
				if (element->left == NULL && element->right != NULL)
				{
					current->left = element->right;
					element->right->up = current;
					size--;
					delete element;
					element = NULL;
					deleted = true;
					return deleted;
				}
				if (element->left != NULL && element->right == NULL)
				{
					current->left = element->left;
					element->left->up = current;
					size--;
					delete element;
					element = NULL;
					deleted = true;
					return deleted;
				}
				else
				{
					BST* temp = element->right;
					while (temp->left != NULL)
					{
						temp = temp->left;
					}
					temp->left = element->left;
					element->left->up = temp;
					current->left = element->right;
					element->right->up = current;
					size--;
					delete element;
					element = NULL;
					deleted = true;
					return deleted;
				}
			}
			if (element->data > current->data)
			{
				if (element->left == NULL && element->right != NULL)
				{
					current->right = element->right;
					element->right->up = current;
					size--;
					delete element;
					element = NULL;
					deleted = true;
					return deleted;
				}
				if (element->left != NULL && element->right == NULL)
				{
					current->right = element->left;
					element->left->up = current;
					size--;
					delete element;
					element = NULL;
					deleted = true;
					return deleted;
				}
				else
				{
					BST* temp = element->right;
					while (temp->left != NULL)
					{
						temp = temp->left;
					}
					temp->left = element->left;
					element->left->up = temp;
					current->right = element->right;
					element->right->up = current;
					size--;
					delete element;
					element = NULL;
					deleted = true;
					return deleted;
				}
			}
		}
		// USUWANY ELEMENT JEST KORZENIEM
		if (element->data == root->data)
		{
			current = root->right;
			while (current->left != NULL)
			{
				current = current->left;
			}
			current->left = root->left;
			root->left->up = current;
			current = root->right;
			current->up = NULL;
			size--;
			root = current;
			delete element;
			element = NULL;
			deleted = true;
			return deleted;
		}
	}
	else
		return deleted;
}

template <class T>
void BST<T>::deleteTree(BST* root)
{
	if (root == NULL)
		return;

	if (root->left != NULL)
	{
		size--;
		deleteTree(root->left);
	}
	if (root->right != NULL)
	{
		size--;
		deleteTree(root->right);
	}
	delete root;
}

template <class T>
int BST<T>::treeHeight(BST* root)
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

template <class T>
void BST<T>::preOrder(BST* root)
{
	if (root != NULL)
	{
		if (root->right != NULL && root->left != NULL && root->up == NULL)
			cout << "( [p: NULL, l: " << root->left->data << ", r: " << root->right->data << "], data: " << root->data << ")," << endl;
		if (root->right != NULL && root->left != NULL && root->up != NULL)
			cout << "( [p: " << root->up->data << ", l: " << root->left->data << ", r: " << root->right->data << "], data: " << root->data << ")," << endl;
		if (root->right == NULL && root->left != NULL && root->up == NULL)
			cout << "( [p: NULL, l: " << root->left->data << ", r: NULL], data: " << root->data << ")," << endl;
		if (root->right == NULL && root->left != NULL && root->up != NULL)
			cout << "( [p: " << root->up->data << ", l: " << root->left->data << ", r: NULL], data: " << root->data << ")," << endl;
		if (root->right != NULL && root->left == NULL && root->up == NULL)
			cout << "( [p: NULL, l: NULL, r: " << root->right->data << "], data: " << root->data << ")," << endl;
		if (root->right != NULL && root->left == NULL && root->up != NULL)
			cout << "( [p: " << root->up->data << ", l: NULL, r: " << root->right->data << "], data: " << root->data << ")," << endl;
		if (root->right == NULL && root->left == NULL && root->up == NULL)
			cout << "( [p: NULL, l: NULL, r: NULL], data: " << root->data << ")," << endl;
		if (root->right == NULL && root->left == NULL && root->up != NULL)
			cout << "( [p: " << root->up->data << ", l: NULL, r: NULL], data: " << root->data << ")," << endl;
		preOrder(root->left);
		preOrder(root->right);
	}
	else
		return;
}

template <class T>
void BST<T>::inOrder(BST* root)
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

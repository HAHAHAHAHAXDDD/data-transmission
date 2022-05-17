#include <iostream>
#include <string>
#include <iomanip>
#include <sstream>
#include <fstream>

template <typename T>
class Item
{
public:
	T Src;
	T Dest;
	T Weight;
	Item* next;
	Item(T src, T dest, T weight) :
		Src(src),
		Dest(dest),
		Weight(weight),
		next(NULL) {}
};

template <typename T>
class Node
{
public:
    T x;
    T y;
};

template <typename T>
class Edge
{
public:
	T source;
	T destination;
	T weight;
};

template <typename T>
class Graph
{
public:
    Node<T>* nodes;
	Edge<T>* edges;
	int numberOfNodes;
	int numberOfEdges;
	int nodesSize;
	int edgesSize;
	Graph()
	{
		edgesSize = 1;
		nodesSize = 1;
        numberOfNodes = 0;
		numberOfEdges = 0;
		nodes = new Node<T>[nodesSize];
		edges = new Edge<T>[edgesSize];
	}
    void addNode(T x_, T y_);
    void printNodes();
	void readNodes(std::string fileName, Graph<T> *collect);
	void addEdge(T src, T dest, T weight_);
	void printEdges();
	void readEdges(std::string filename, Graph<T>* ecollect);
	T minWeight();
	T maxWeight();
	void Sort();
};

template <typename T>
T Graph<T>::maxWeight()
{
	T max = 0;
	for (int i = 0; i < numberOfEdges; i++)
	{
		if (edges[i].weight > max)
			max = edges[i].weight;
	}
	return max;
}

template <typename T>
T Graph<T>::minWeight()
{
	T min = edges[0].weight;
	for (int i = 0; i < numberOfEdges; i++)
	{
		if (edges[i].weight < min)
			min = edges[i].weight;
	}
	return min;
}

template <typename T>
void Graph<T>::Sort()
{
	//Obliczenie liczby kubelkow oraz rozpietosci przedzialow w kubelkach
	int numberOfBuckets = sqrt(numberOfEdges);
	T max = maxWeight();
	T min = minWeight();
	T gap = (max - min) / (numberOfBuckets - 1);

	//Wyzerowanie kubelkow
	Item<T>** buckets = new Item<T>*[numberOfBuckets];
	for (int i = 0; i < numberOfBuckets; i++)
		buckets[i] = NULL;
	//Wstawianie elementow do kubelkow(od razu je sortujac)
	for (int i = 0; i < numberOfEdges; i++)
	{
		Item<T>* prev = NULL;
		Item<T>* element = buckets[(int)((edges[i].weight - min) / gap)];
		if (element == NULL)
		{
			element = new Item<T>(edges[i].source, edges[i].destination, edges[i].weight);
			buckets[(int)((edges[i].weight - min) / gap)] = element;
		}
		else
		{
			while (element != NULL && edges[i].weight > element->Weight)
			{
				prev = element;
				element = element->next;
			}
			if (prev == NULL)
			{
				element = new Item<T>(edges[i].source, edges[i].destination, edges[i].weight);
				element->next = buckets[(int)((edges[i].weight - min) / gap)];
				buckets[(int)((edges[i].weight - min) / gap)] = element;
			}
			else
			{
				element = new Item<T>(edges[i].source, edges[i].destination, edges[i].weight);
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
			edges[ind].source = buckets[i]->Src;
			edges[ind].destination = buckets[i]->Dest;
			edges[ind].weight = buckets[i]->Weight;
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

template <typename T>
void Graph<T>::readEdges(std::string filename, Graph<T>* ecollect)
{
	std::string src, dest, weight_;
	std::ifstream infile(filename);
	if (infile.is_open())
	{
		std::string line;
		getline(infile, line);
		int count = std::stoi(line);
		int i = 1;
		while (i <= count+1)
		{
			getline(infile, line);
			i++;
		}
		while (!infile.eof())
		{
			getline(infile, src, ' ');
			if (src.empty())
				break;
			getline(infile, dest, ' ');
			getline(infile, weight_, '\n');
			ecollect->addEdge(std::stoi(src), std::stoi(dest), std::stod(weight_));
		}
		infile.close();
	}
	else
		std::cout << "open file failed" << std::endl;
}

class UnionFind
{
public:
	int* parent;
	int* rank;
	int nodes;
	UnionFind(int n)
	{
		nodes = n;
		parent = new int[n];
		rank = new int[n];
		for (int i = 0; i < n; i++)
		{
			parent[i] = i;
			rank[i] = 0;
		}
	}
	void print();
	void unionByRank(int i, int j);
	int Find(int i);
};

void UnionFind::print()
{
	std::cout << "Rodzice: " << std::endl;
	for (int i = 0; i < nodes; i++)
	{
		std::cout << parent[i] << " ";
	}
	std::cout << std::endl;
	std::cout << "Rangi: " << std::endl;
	for (int i = 0; i < nodes; i++)
	{
		std::cout << rank[i] << " ";
	}
	std::cout << std::endl;
}

void UnionFind::unionByRank(int i, int j)
{
	if (rank[i] < rank[j])
		parent[i] = j;
	else if (rank[j] < rank[i])
		parent[j] = i;
	else {
		parent[i] = j;
		rank[j]++;
	}
}

int UnionFind::Find(int i)
{
	if (i == parent[i])
		return i;
	int root = Find(parent[i]);
	if (root != parent[i])
		parent[i] = root;
	return root;
}

template <typename T>
void Graph<T>::printEdges()
{
	if (numberOfEdges > 0)
		for (int i = 0; i < numberOfEdges; i++)
			std::cout << "#" << i << " source: " << edges[i].source << ", dest: " << edges[i].destination << ", weight: " << edges[i].weight << std::endl;
}

template <typename T>
void Graph<T>::addEdge(T src, T dest, T weight_)
{
	if (numberOfEdges >= edgesSize)
	{
		edgesSize = edgesSize * 2;
		Edge<T>* temp = new Edge<T>[edgesSize];
		for (size_t i = 0; i < numberOfEdges; i++)
		{
			temp[i] = edges[i];
		}
		delete[] edges;
		edges = temp;
	}
	edges[numberOfEdges].source = src;
	edges[numberOfEdges].destination = dest;
	edges[numberOfEdges].weight = weight_;
	numberOfEdges++;
}

template <typename T>
void Graph<T>::readNodes(std::string filename, Graph<T> *collect)
{
	std::string X, Y;
	std::ifstream infile(filename);
	if (infile.is_open())
	{
		std::string line;
		getline(infile, line);
		int count = std::stoi(line);
		int i = 1;
		while (i <= count)
		{
			getline(infile, X, ' ');
			getline(infile, Y, '\n');
			collect->addNode(std::stod(X), std::stod(Y));
			i++;
		}
		infile.close();
	}
	else
		std::cout << "open file failed" << std::endl;
}

template <typename T>
void Graph<T>::printNodes()
{
	if (numberOfNodes > 0)
	{
		std::cout << "Liczba punktow: " << numberOfNodes << std::endl;;
		for (int i = 0; i < numberOfNodes; i++)
			std::cout << "#" << i << " x: " << nodes[i].x << ", y: " << nodes[i].y << std::endl;
	}
}

template <typename T>
void Graph<T>::addNode(T x_, T y_)
{
	if (numberOfNodes >= nodesSize)
	{
		nodesSize = nodesSize * 2;
		Node<T>* temp = new Node<T>[nodesSize];
		for (size_t i = 0; i < numberOfNodes; i++)
		{
			temp[i] = nodes[i];
		}
		delete[] nodes;
		nodes = temp;
	}
	nodes[numberOfNodes].x = x_;
	nodes[numberOfNodes].y = y_;
	numberOfNodes++;
}

template <typename T>
void Kruskal(Graph<T> graph)
{
	UnionFind UF(graph.numberOfNodes);
	Graph <double> mst;
	int i_root, j_root, count;
	double weightSum = 0;
	int findSum = 0;
	count = 0;
	graph.Sort();
	for (int i = 0; i < graph.numberOfEdges; i++)
	{
		i_root = UF.Find((graph.edges[i]).source);
		findSum++;
		j_root = UF.Find((graph.edges[i]).destination);
		findSum++;
		if (i_root != j_root)
		{
			UF.unionByRank(i_root, j_root);
			mst.addEdge((graph.edges[i]).source, (graph.edges[i]).destination, (graph.edges[i]).weight);
			count++;
			weightSum += graph.edges[i].weight;
			if (count == graph.numberOfNodes - 1)
				break;
		}
	}

	//Wypis wynikow
	mst.printEdges();
	std::cout << "-----" << std::endl;
	std::cout << "number of edges: " << mst.numberOfEdges << std::endl;
	std::cout << "weight sum: " << weightSum << std::endl;
	std::cout << "find sum: " << findSum << std::endl;
}

int main()
{
	
	Graph <double> data;
	data.readNodes("g1.txt", &data);
	data.readEdges("g1.txt", &data);
	Kruskal(data);
}

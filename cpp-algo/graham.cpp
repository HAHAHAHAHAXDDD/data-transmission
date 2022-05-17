#include <iostream>
#include <string>
#include <iomanip>
#include <sstream>
#include <fstream>
#include <math.h>

class Item
{
public:
	double X;
	double Y;
	double Deg;
	Item* next;
	Item(double x, double y, double deg) :
		X(x),
		Y(y),
		Deg(deg),
		next(NULL) {}
};

class Point
{
public:
	double x;
	double y;
	double deg;
};

class Table
{
public:
	Point* points;
	int numberOfPoints;
	int Size;
	Table()
	{
		numberOfPoints = 0;
		Size = 1;
		points = new Point[Size];
	}
	void addPoint(double X, double Y, double Deg);
	void Print();
	void readPoints(std::string filename, Table* tab);
	double deg(Point a, Point b);
	double maxDeg();
	double minDeg();
	void Sort();
	void deleteInd(int index);
	int cmp(double a, double b);
	bool tescik(Point a, Point b, Point c);
};

double Table::maxDeg()
{
	double max = 0;
	for (int i = 0; i < numberOfPoints; i++)
	{
		if (points[i].deg > max)
			max = points[i].deg;
	}
	return max;
}

double Table::minDeg()
{
	double min = points[0].deg;
	for (int i = 0; i < numberOfPoints; i++)
	{
		if (points[i].deg < min)
			min = points[i].deg;
	}
	return min;
}

void Table::addPoint(double X, double Y, double Deg)
{
	if (numberOfPoints >= Size)
	{
		Size = Size * 2;
		Point* temp = new Point[Size];
		for (size_t i = 0; i < numberOfPoints; i++)
		{
			temp[i] = points[i];
		}
		delete[] points;
		points = temp;
	}
	points[numberOfPoints].x = X;
	points[numberOfPoints].y = Y;
	points[numberOfPoints].deg = Deg;
	numberOfPoints++;
}

void Table::Print()
{
	std::cout << "Liczba punktow: " << numberOfPoints << std::endl;
	if (numberOfPoints > 0)
		for (int i = 0; i < numberOfPoints; i++)
			std::cout << "X: " << std::setprecision(15) << points[i].x << " Y: " << std::setprecision(15) << points[i].y << std::endl;
}

void Table::readPoints(std::string filename, Table* tab)
{
	std::string X, Y;
	std::ifstream infile(filename);
	if (infile.is_open())
	{
		std::string line;
		getline(infile, line);
		while (!infile.eof())
		{
			getline(infile, X, ' ');
			if (X.empty())
				break;
			getline(infile, Y, '\n');
			tab->addPoint(std::stod(X), std::stod(Y), 0);
		}
		infile.close();
	}
	else
		std::cout << "open file failed" << std::endl;
}

double Table::deg(Point a, Point b)
{
	double y = a.y - b.y;
	double x = a.x - b.x;
	return atan2(y, x);
	//return (b.x * a.y) - (a.x * b.y);
}

void Table::Sort()
{
	//Obliczenie liczby kubelkow oraz rozpietosci przedzialow w kubelkach
	int numberOfBuckets = sqrt(numberOfPoints);
	double max = maxDeg();
	double min = minDeg();
	double gap = (max - min) / (numberOfBuckets - 1);

	//Wyzerowanie kubelkow
	Item** buckets = new Item * [numberOfBuckets];
	for (int i = 0; i < numberOfBuckets; i++)
		buckets[i] = NULL;
	//Wstawianie elementow do kubelkow(od razu je sortujac)
	for (int i = 0; i < numberOfPoints; i++)
	{
		Item* prev = NULL;
		Item* element = buckets[(int)((points[i].deg - min) / gap)];
		if (element == NULL)
		{
			element = new Item(points[i].x, points[i].y, points[i].deg);
			buckets[(int)((points[i].deg - min) / gap)] = element;
		}
		else
		{
			while (element != NULL && points[i].deg > element->Deg)
			{
				prev = element;
				element = element->next;
			}
			if (prev == NULL)
			{
				element = new Item(points[i].x, points[i].y, points[i].deg);
				element->next = buckets[(int)((points[i].deg - min) / gap)];
				buckets[(int)((points[i].deg - min) / gap)] = element;
			}
			else
			{
				element = new Item(points[i].x, points[i].y, points[i].deg);
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
			points[ind].x = buckets[i]->X;
			points[ind].y = buckets[i]->Y;
			points[ind].deg = buckets[i]->Deg;
			ind++;
			buckets[i] = buckets[i]->next;
		}
	}

	//Zwolnienie pamieci
	for (int i = 0; i < numberOfBuckets; i++)
	{
		while (buckets[i] != NULL)
		{
			Item* temp = buckets[i];
			buckets[i] = temp->next;
			delete temp;
		}
	}
	delete[] buckets;
}

void Table::deleteInd(int index)
{
	for (int i = index; i < numberOfPoints; i++)
		points[i] = points[i + 1];
	numberOfPoints--;
}

int Table::cmp(double a, double b)
{
	double val = a - b;
	if (val > 0)
		return 1;
	else if (val < 0)
		return -1;
	else
		return 0;

}

void Graham(Table* tab)
{
	Table CH;
	double min = (tab->points[0]).y;
	int minind = 0;
	for (int i = 1; i < tab->numberOfPoints; i++)
	{
		if ((tab->points[i]).y < min)
		{
			min = (tab->points[i]).y;
			minind = i;
		}
		else if ((tab->points[i]).y == min)
		{
			if ((tab->points[i]).x < (tab->points[minind]).x)
			{
				min = (tab->points[i]).y;
				minind = i;
			}
		}
	}
	CH.addPoint((tab->points[minind]).x, (tab->points[minind]).y, (tab->points[minind]).deg);
	for (int i = 0; i < tab->numberOfPoints; i++)
	{
		if (i == minind)
			continue;
		tab->points[i].deg = tab->deg(tab->points[minind], tab->points[i]);
	}
	tab->deleteInd(minind);
	tab->Sort();
	CH.addPoint((tab->points[0]).x, (tab->points[0]).y, (tab->points[0]).deg);
	for (int i = 1; i < tab->numberOfPoints; i++)
	{
		CH.addPoint(tab->points[i].x, tab->points[i].y, tab->points[i].deg);
		while (CH.numberOfPoints > 2 && CH.cmp(((CH.points[CH.numberOfPoints - 1].x - CH.points[CH.numberOfPoints - 3].x) * (CH.points[CH.numberOfPoints - 2].y - CH.points[CH.numberOfPoints - 3].y)), ((CH.points[CH.numberOfPoints - 1].y - CH.points[CH.numberOfPoints - 3].y) * (CH.points[CH.numberOfPoints - 2].x - CH.points[CH.numberOfPoints - 3].x))) == 1)
		{
			CH.deleteInd(CH.numberOfPoints - 2);
		}
	}
	CH.Print();
}

int main()
{
	Table test;
	test.readPoints("points1.txt", &test);
	Graham(&test);
}

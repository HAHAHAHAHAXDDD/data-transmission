package connect4;

import sac.State;
import sac.StateFunction;

public class Oceniacz extends StateFunction{
	@Override
	public double calculate(State state) {
		Connect4 c4 = (Connect4) state;
		if (c4.isWin())
			return c4.isMaximizingTurnNow() ? Double.NEGATIVE_INFINITY : Double.POSITIVE_INFINITY;
		else {
			double val = 0.;
			
			// otwarcie centrum > boki
			val += openingCenter(c4);
			
			
			// Przy wyliczaniu blokow 2/3 elementowych
			// tworzenie struktur w centrum > tworzenie struktur na bokach planszy
			// z pominieciem struktur diagonalnych
			// (Blad polegajacy na tym ze preferowane jest zrobienie "slabej trojki?")
			// Bardzo duze wyplaty po pierwszym ruchu w przypadku MinMax?
			
			
			// zliczanie 2 elementowych blokow
			// wiersz
			val += twoRow(c4);
			// kolumna
			val += twoColumn(c4);
			// skos
			val += twoDiagonal(c4);
			
				
			// zliczanie 3 elementowych blokow w wierszu
			// wiersz
			val += threeRow(c4);
			// kolumna
			val += threeColumn(c4);
			// skos
			val += threeDiagonal(c4);
			
			return val;
		}
	}
	
	public double openingCenter(Connect4 c4) {
		double val = 0.;
		int countO = 0;
		int countX = 0;
		for (int j = 3; j < c4.n-3; j++) {
			if (c4.board[c4.m-1][j] == 1)
				countX++;
			else if (c4.board[c4.m-1][j] == 2)
				countO++;
		}
		if (countX > countO)
			val += 0.1;
		else
			val -= 0.1;
		return val;
	}
	
	public double twoRow(Connect4 c4) {
		double val = 0.;
		for (int i = c4.m-1; i >= 0; i--) {
			int cell = -1;
			int points = 0;
			for (int j = 0; j < c4.n; j++) {					
				if (c4.board[i][j] == 0) {
					points = 0;
					continue;
				}
				if (c4.board[i][j] != cell) {
					cell = c4.board[i][j];
					points = 1;
				}
				else if(c4.board[i][j] == cell) {
					points++;
					if (points >= 2 && cell == 1){
						if (j >=3 && j <= c4.n-4)
							val += 3.0;
						else
							val += 2.0;
						points = 0;
						cell = -1;
					}	
					else if (points >= 2 && cell == 2) {							
						if (j >=3 && j <= c4.n-4)
							val -= 3.0;
						else
							val -= 2.0;
						points = 0;
						cell = -1;
					}
						
				}
			}
		}
		return val;
	}
	
	public double threeRow(Connect4 c4) {
		double val = 0.;
		
		for (int i = c4.m-1; i >= 0; i--) {
			int cell = -1;
			int points = 0;
			for (int j = 0; j < c4.n; j++) {					
				if (c4.board[i][j] == 0) {
					points = 0;
					continue;
				}
				if (c4.board[i][j] != cell) {
					cell = c4.board[i][j];
					points = 1;
				}
				else if(c4.board[i][j] == cell) {
					points++;
					if (points >= 3 && cell == 1){
						if (j >=3 && j <= c4.n-4)
							val += 10.0;
						else
							val += 9.0;
						points = 0;
						cell = -1;
					}	
					else if (points >= 3 && cell == 2) {							
						if (j >=3 && j <= c4.n-4)
							val -= 10.0;
						else
							val -= 9.0;
						points = 0;
						cell = -1;
					}
						
				}
			}
		}	
		return val;
	}

	public double twoColumn(Connect4 c4) {
		double val = 0.;
		int points = 0;
		for (int j = 0; j < c4.n; j++) {
			int cell = -1;
			if (c4.board[c4.m-1][j] != 0) {
				cell = c4.board[c4.m-1][j];
				points++;
			}
			else
				continue;
			for (int i = c4.m-2; i >= 0; i--) {
				if (cell == c4.board[i][j]){
					points++;
					if (points >= 2 && cell == 1){
						if (j >=3 && j <= c4.n-4)
							val += 3.0;
						else
							val += 2.0;
						points = 0;
						cell = -1;
						continue;
					}	
					else if (points >= 2 && cell == 2) {							
						if (j >=3 && j <= c4.n-4)
							val -= 3.0;
						else
							val -= 2.0;
						points = 0;
						cell = -1;
						continue;
					}
				}					
				else if (cell != c4.board[i][j] && c4.board[i][j] != 0){
					cell = c4.board[i][j];
					points = 1;
				}
				else{
					points = 0;
					break;
				}
			}
		}
		return val;
	}

	public double threeColumn(Connect4 c4) {
		double val = 0.;
		int points = 0;
		for (int j = 0; j < c4.n; j++) {
			int cell = -1;
			if (c4.board[c4.m-1][j] != 0) {
				cell = c4.board[c4.m-1][j];
				points++;
			}
			else
				continue;
			for (int i = c4.m-2; i >= 0; i--) {
				if (cell == c4.board[i][j]){
					points++;
					if (points >= 3 && cell == 1){
						if (j >=3 && j <= c4.n-4)
							val += 10.0;
						else
							val += 9.0;
						points = 0;
						cell = -1;
						continue;
					}	
					else if (points >= 3 && cell == 2) {							
						if (j >=3 && j <= c4.n-4)
							val -= 10.0;
						else
							val -= 9.0;
						points = 0;
						cell = -1;
						continue;
					}
				}					
				else if (cell != c4.board[i][j] && c4.board[i][j] != 0){
					cell = c4.board[i][j];
					points = 1;
				}
				else{
					points = 0;
					break;
				}
			}
		}
		return val;
	}

	public double twoDiagonal(Connect4 c4) {
		
		double val = 0.;
		
		// prawy skos
		for (int i = c4.m-1; i > 2; i--)
			for (int j = 0; j < c4.n-3; j++)
				if (c4.board[i][j] != 0 && c4.board[i][j] == 1)
					if (c4.board[i][j] == c4.board[i-1][j+1])
						val += 3.5;
				else if (c4.board[i][j] != 0 && c4.board[i][j] == 2)
					if (c4.board[i][j] == c4.board[i-1][j+1])
						val -= 3.5;
					
						
		// lewy skos
		for (int i = c4.m-1; i > 2; i--)
			for (int j = c4.n-1; j > 2; j--)
				if (c4.board[i][j] != 0 && c4.board[i][j] == 1)
					if (c4.board[i][j] == c4.board[i-1][j-1])
						val += 3.5;
				else if (c4.board[i][j] != 0 && c4.board[i][j] == 2)
					if (c4.board[i][j] == c4.board[i-1][j-1])
						val -= 3.5;
						
		
		return val;
	}
	
	public double threeDiagonal(Connect4 c4) {
		
		double val = 0.;
		
		// prawy skos
		for (int i = c4.m-1; i > 2; i--)
			for (int j = 0; j < c4.n-3; j++)
				if (c4.board[i][j] != 0 && c4.board[i][j] == 1)
					if ((c4.board[i][j] == c4.board[i-1][j+1]) && (c4.board[i][j] == c4.board[i-2][j+2]))
						val += 10.5;
				else if (c4.board[i][j] != 0 && c4.board[i][j] == 2)
					if ((c4.board[i][j] == c4.board[i-1][j+1]) && (c4.board[i][j] == c4.board[i-2][j+2]))
						val -= 10.5;
					
						
		// lewy skos
		for (int i = c4.m-1; i > 2; i--)
			for (int j = c4.n-1; j > 2; j--)
				if (c4.board[i][j] != 0 && c4.board[i][j] == 1)
					if ((c4.board[i][j] == c4.board[i-1][j-1]) && (c4.board[i][j] == c4.board[i-2][j-2]))
						val += 10.5;
				else if (c4.board[i][j] != 0 && c4.board[i][j] == 2)
					if ((c4.board[i][j] == c4.board[i-1][j-1]) && (c4.board[i][j] == c4.board[i-2][j-2]))
						val -= 10.5;
						
		
		return val;
	}

}

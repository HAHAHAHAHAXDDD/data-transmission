package connect4;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.Scanner;

import sac.State;
import sac.game.AlphaBetaPruning;
import sac.game.GameSearchAlgorithm;
import sac.game.GameState;
import sac.game.GameStateImpl;
import sac.game.MinMax;
import sac.graph.GraphState;

public class Connect4 extends GameStateImpl{
	public static final int m = 6;
	public static final int n = 7;
	public byte[][] board;
	
	public Connect4() {
		board = new byte[m][n];
		for (int i = 0; i < m; i++) {
			for (int j = 0; j < n; j++)
				board[i][j] = 0;
		}
	}
	
	public Connect4(Connect4 parent) {
		board = new byte[m][n];
		for (int i = 0; i < m; i++) {
			for (int j = 0; j < n; j++) {
				board[i][j] = parent.board[i][j];
			}
		}
		setMaximizingTurnNow(parent.isMaximizingTurnNow());
	}
	
	@Override
	public int hashCode() {
		byte[] connect4Flat = new byte[m*n];
		int k = 0;
		for(int i = 0; i < m; i++)
			for(int j = 0; j < n; j++)
				connect4Flat[k++] = board[i][j];
		return Arrays.hashCode(connect4Flat);
	}
	
	
	public void makeMove(int column) {
		for (int i = m-1; i >= 0; i--) {
			if (board[i][column] == 0) {
				if (isMaximizingTurnNow()) {					
					board[i][column] = 1;
					break;
				}
				else {
					board[i][column] = 2;
					break;
				}
			}
		}
		setMaximizingTurnNow(!isMaximizingTurnNow());
	}
	
	@Override
	public String toString() {
		StringBuilder txt = new StringBuilder();
		for (int i = 0; i < m; i++) {
			txt.append("|");
			for (int j = 0; j < n; j++) {
				if (board[i][j] == 0)
					txt.append(" ");
				else if (board[i][j] == 1)
					txt.append("X");
				else
					txt.append("O");
				txt.append("|");
			}
			txt.append("\n");
		}
		txt.append("---------------");
		return txt.toString();
	}
	
	public boolean isWin() {
		// wygrana w kolumnie
		int points = 0;
		for (int j = 0; j < n; j++) {
			int val = -1;
			if (board[m-1][j] != 0) {
				val = board[m-1][j];
				points++;
			}
			else
				continue;
			for (int i = m-2; i >= 0; i--) {
				if (val == board[i][j]){
					points++;
					if (points >= 4)
						return true;
				}					
				else if (val != board[i][j] && board[i][j] != 0){
					val = board[i][j];
					points = 1;
				}
				else{
					points = 0;
					break;
				}
			}
		}
		
		// wygrana w wierszu
		for (int i = m-1; i >= 0; i--) {
			int val = -1;
			points = 0;
			for (int j = 0; j < n; j++) {					
				if (board[i][j] == 0) {
					points = 0;
					continue;
				}
				if (board[i][j] != val) {
					val = board[i][j];
					points = 1;
				}
				else if(board[i][j] == val) {
					points++;
					if (points >= 4)
						return true;
				}
			}
		}
		
		// wygrana skos		
		// prawy skos
		for (int i = m-1; i > 2; i--)
			for (int j = 0; j < n-3; j++)
				if (board[i][j] != 0)
					if ((board[i][j] == board[i-1][j+1]) && (board[i][j] == board[i-2][j+2]) && (board[i][j] == board[i-3][j+3]))
						return true;
		// lewy skos
		for (int i = m-1; i > 2; i--)
			for (int j = n-1; j > 2; j--)
				if (board[i][j] != 0)
					if ((board[i][j] == board[i-1][j-1]) && (board[i][j] == board[i-2][j-2]) && (board[i][j] == board[i-3][j-3]))
						return true;
				
		// sufit
		for (int i = 0; i < n; i++) {
			if (board[0][i] != 0)
				return true;
		}
				
		return false;
	}
	
	public static void main(String[] args) {
		Connect4 game = new Connect4();
		Scanner scan = new Scanner(System.in);
		int move = 0;
		System.out.println("Gra Connect4");
		System.out.println("Wpisz numer kolumny (numerujac od 0) w ktorej chcesz wykonac ruch");
		System.out.println("Kto zaczyna? (1 - czlowiek, 2 - nie czlowiek): ");
		move = scan.nextInt();
		if (move == 1)
			game.setMaximizingTurnNow(!game.isMaximizingTurnNow());
		System.out.println("---------------");
		Connect4.setHFunction(new Oceniacz());
		System.out.println(game);
		while (true) {
			if (!game.isMaximizingTurnNow()) {
				System.out.println("Numer kolumny: ");
				move = scan.nextInt();
				game.makeMove(move);
				System.out.println(game);
				if (game.isWin()){
					System.out.println("Wygral czlowiek");
					break;
				}
			}
			else {
//				GameSearchAlgorithm algo = new AlphaBetaPruning(game);
				GameSearchAlgorithm algo = new MinMax(game);
				algo.execute();
				System.out.println(algo.getMovesScores());
				move = Integer.parseInt(algo.getFirstBestMove());
				game.makeMove(move);
				System.out.println(game);
				if (game.isWin()){
					System.out.println("Wygral nie czlowiek");
					break;
				}
			}
			
		}
	}

	@Override
	public List<GameState> generateChildren() {
		List<GameState> children = new ArrayList<>();
		for (int i = 0; i < n; i++) {
			Connect4 child = new Connect4(this);
			child.makeMove(i);
			children.add(child);
			child.setMoveName(Integer.toString(i));
		}
		
		return children;
	}
	
	
	
}

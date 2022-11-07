import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Random;

import sac.StateFunction;
import sac.StateImpl;
import sac.graph.AStar;
import sac.graph.GraphSearchAlgorithm;
import sac.graph.GraphState;
import sac.graph.GraphStateImpl;

public class Puzzle extends GraphStateImpl{
	public static final int n = 3;
	public byte[][] board;
	public byte[] coords = {0, 0};
	Random generateMove = new Random(69420);
	
	public Puzzle() {
		board = new byte[n][n];
		byte value = 0;
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++)
				board[i][j] = value++;
		}
	}
	
	public Puzzle(Puzzle parent) {
		board = new byte[n][n];
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				board[i][j] = parent.board[i][j];
			}
		}
		coords[0] = parent.coords[0];
		coords[1] = parent.coords[1];
	}
	
	public boolean makeMove(int move) {
		if(move == 0){
			if(coords[1] - 1 < 0)
				return false;
			board[coords[0]][coords[1]] = board[coords[0]][--coords[1]];
			board[coords[0]][coords[1]] = 0;
		}
		else if(move == 1){
			if(coords[0] - 1 < 0)
				return false;
			board[coords[0]][coords[1]] = board[--coords[0]][coords[1]];
			board[coords[0]][coords[1]] = 0;
		}	
		else if(move == 2){
			if(coords[1] + 1 > 2)
				return false;
			board[coords[0]][coords[1]] = board[coords[0]][++coords[1]];
			board[coords[0]][coords[1]] = 0;
		}
		else {
			if(coords[0] + 1 > 2)
				return false;
			board[coords[0]][coords[1]] = board[++coords[0]][coords[1]];
			board[coords[0]][coords[1]] = 0;
		}

		return true;
	}
	
	public void shuffle(int totalMoves) {
		int legalMoves = 0;
		while(legalMoves < totalMoves)
			if(makeMove(generateMove.nextInt(0, 4)))
				legalMoves++;
	}
	
	@Override
	public String toString() {
		StringBuilder txt = new StringBuilder();
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				txt.append(board[i][j]);
				txt.append(" ");
			}
			txt.append("\n");
		}
		return txt.toString();
	}
	
	
	
	public static void main(String[] args) {
		
		Puzzle p = new Puzzle();
		p.shuffle(1000);
		System.out.println("MIXED PUZZLE: \n" + p);
		
		StateFunction[] heuristics = {new MisplacedTiles(), new Manhattan()};
		
		for (StateFunction h : heuristics) {
			
			Puzzle.setHFunction(h);
			GraphSearchAlgorithm algo = new AStar(p);
			algo.execute();
			Puzzle solution = (Puzzle) algo.getSolutions().get(0);
			
			System.out.println("---------------------");
			System.out.println("HEURISTICS: " + h);
			System.out.println("PATH LENGTH: " + solution.getPath().size());
			System.out.println("MOVES ALONG PATH: " + solution.getMovesAlongPath());
			System.out.println("CLOSED STATES: " + algo.getClosedStatesCount());
			System.out.println("OPEN STATES: " + algo.getOpenSet().size());
			System.out.println("DURATION TIME: " + algo.getDurationTime() + "ms");
			System.out.println("SOLUTION:\n" + solution);
			
		}	
	}

	@Override
	public List<GraphState> generateChildren() {
		List<GraphState> children = new ArrayList<>();
		
		for (int move = 0; move < 4; move++) {
			Puzzle child = new Puzzle(this);
			if (child.makeMove(move)) {
				String moveName = "L";
				if (move == 1)
					moveName = "U";
				else if (move == 2)
					moveName = "R";
				else if (move == 3)
					moveName = "D";
				child.setMoveName(moveName);
				children.add(child);
			}
			else {
				child = null;
			}
		}
		return children;
	}

	@Override
	public boolean isSolution() {
		int isOk = 0;
		for (int i = 0; i < n; i++)
			for (int j = 0; j < n; j++) {
				if (board[i][j] != isOk)
					return false;
				isOk++;
			}
		return true;
	}
	
	@Override
	public int hashCode() {
		byte[] sudokuFlat = new byte[n*n];
		int k = 0;
		for(int i = 0; i < n; i++)
			for(int j = 0; j < n; j++)
				sudokuFlat[k++] = board[i][j];
		return Arrays.hashCode(sudokuFlat);
	}
}

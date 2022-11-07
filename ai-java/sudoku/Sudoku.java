package sanocki.sudoku;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

import sac.graph.BestFirstSearch;
import sac.graph.GraphSearchAlgorithm;
import sac.graph.GraphSearchConfigurator;
import sac.graph.GraphState;
import sac.graph.GraphStateImpl;

public class Sudoku extends GraphStateImpl{
	public static final int n = 3;
	public static final int n2 = n*n;
	private byte[][] board;
	private int zeros = n2*n2;
	
	public Sudoku() {
		board = new byte[n2][n2];
		for (int i = 0; i < n2; i++) {
			for (int j = 0; j < n2; j++)
				board[i][j] = 0;
		}
	}
	
	public int getZeros() {
		return zeros;
	}
	
	public Sudoku(Sudoku parent) {
		board = new byte[n2][n2];
		for (int i = 0; i < n2; i++) {
			for (int j = 0; j < n2; j++) {
				board[i][j] = parent.board[i][j];
			}
		}
		zeros = parent.zeros;
	}
	
	@Override
	public String toString() {
		StringBuilder txt = new StringBuilder();
		for (int i = 0; i < n2; i++) {
			for (int j = 0; j < n2; j++) {
				txt.append(board[i][j]);
				txt.append(" ");
			}
			txt.append("\n");
		}
		return txt.toString();
	}

	public void fromString(String txt) {
		int k = 0;
		for (int i = 0; i < n2; i++) {
			for (int j = 0; j < n2; j++, k++)
				board[i][j] = Byte.valueOf(txt.substring(k, k+1));
		}
		refreshZeros();
	}
	
	public boolean isLegal() {
		byte[] group = new byte[n2];
		//wiersze
		for (int i = 0; i < n2; i++) {
			for (int j = 0; j < n2; j++)
				group[j] = board[i][j];
			if (!isGroupLegal(group))
				return false;
		}
		//kolumny
		for (int i = 0; i < n2; i++) {
			for (int j = 0; j < n2; j++)
				group[j] = board[j][i];
			if (!isGroupLegal(group))
				return false;
		}
		//kwadraty
		for (int i = 0; i < n; i++)
			for (int j = 0; j < n; j++) {
				int q = 0;
				for (int k = 0; k < n; k++)
					for (int l = 0; l < n; l++)
						group[q++] = board[i*n+k][j*n+l];
				if (!isGroupLegal(group))
					return false;
			}
		
		return true;
	}
	
	private boolean isGroupLegal(byte[] group) {
		boolean[] visited = new boolean[n2+1]; //mozna tez group.length+1
		for (int i = 1; i < n2 + 1; i++)
			visited[i] = false;
		for (int i = 0; i < n2; i++)
			if (group[i] > 0) {
				if (visited[group[i]])
					return false;
				visited[group[i]] = true;
			}
		return true;
	}

	private void refreshZeros() {
		zeros = 0;
		for (int i = 0; i < n2; i++)
			for (int j =  0; j < n2; j++)
				if (board[i][j] == 0)
					zeros++;
	}
	
	public static void main(String[] args) {
		String sudokuAsTxT = "043080250"
				+ "600000000"
				+ "000001094"
				+ "900004070"
				+ "000000000"
				+ "010200003"
				+ "820500000"
				+ "000000005"
				+ "034090710";
		Sudoku s = new Sudoku();
		s.fromString(sudokuAsTxT);
		System.out.println(s);
		Sudoku.setHFunction(new EmptyPlacesHeuristics());
		GraphSearchConfigurator conf = new GraphSearchConfigurator();
		conf.setWantedNumberOfSolutions(Integer.MAX_VALUE);
		GraphSearchAlgorithm algo = new BestFirstSearch(s, conf);
		algo.execute();
		List<GraphState> solutions = algo.getSolutions();
		for(GraphState solution : solutions)
			System.out.println(solution);
		
		System.out.println("Time [ms]: " + algo.getDurationTime());
		System.out.println("Closed: " + algo.getClosedStatesCount());
		System.out.println("Open: " + algo.getOpenSet().size());
		System.out.println("Solutions: " + solutions.size());
}
	@Override
	public List<GraphState> generateChildren() {

		List<GraphState> children = new ArrayList<>();
		int i = 0, j = 0;
		
		zeroFinder:
		for (; i < n2; i++)
			for (j = 0; j < n2; j++)
				if(board[i][j] == 0)
					break zeroFinder;
		
		if (i == n2)
			return children;
		
		for (int k = 1; k <= n2; k++) {
			Sudoku child = new Sudoku(this);
			child.board[i][j] = (byte)k;
			if(child.isLegal()) {
				children.add(child);
				child.zeros--;
			}
		}
		return children;
	}
	
	@Override
	public boolean isSolution() {
		return zeros == 0 && isLegal();
	}
	
	@Override
	public int hashCode() {
		byte[] sudokuFlat = new byte[n2*n2];
		int k = 0;
		for(int i = 0; i < n2; i++)
			for(int j = 0; j < n2; j++)
				sudokuFlat[k++] = board[i][j];
		return Arrays.hashCode(sudokuFlat);
//		return Arrays.deepHashCode(board);
	}
}
/// ---
//  wejsciowka przeszukiwanie grafow (tych algorytmow)
/// ---
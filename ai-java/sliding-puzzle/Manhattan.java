import sac.State;
import sac.StateFunction;

public class Manhattan extends StateFunction{
	
	@Override
	public double calculate(State state) {
		Puzzle p  = (Puzzle) state;
		double result = 0.;
		for (int i = 0; i < p.n; i++)
			for (int j = 0; j < p.n; j++)
				if (i != p.coords[0] || j != p.coords[1])
					result += Math.abs((p.board[i][j] / p.n) - i) + Math.abs((p.board[i][j] % p.n) - j);		
		return result;
	}
	
	@Override
	public String toString() {
		return "Manhattan";
	}
}

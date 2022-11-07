import sac.State;
import sac.StateFunction;

public class MisplacedTiles extends StateFunction {
	
	@Override
	public double calculate(State state) {
		
		Puzzle p = (Puzzle) state;
		double result = 0.;
		int check = 0;
		for (int i = 0; i < p.n; i++)
			for (int j = 0; j < p.n; j++){
				if ((i != p.coords[0] || j != p.coords[1]) && p.board[i][j] != check)
					result = result + 1.;
				check++;
			}		
		return result;
		
	}
	
	@Override
	public String toString() {
		return "MisplacedTiles";
	}
}

import java.util.ArrayList;

public class LevAutomaton {
    public String string;
    public int maxEdits;

    public LevAutomaton(String _string, int _maxEdits) {
        string = _string;
        maxEdits = _maxEdits;
    }

    public boolean isMatch(int[] state) {
        return state[state.length - 1] <= maxEdits;
    }

    public boolean canMatch(int[] state) {
        int min = 999999;
        for (int x : state) {
            min = Math.min(x, min);
        }
        return min <= maxEdits;
    }

    public int[] init() {
        var list = new int[string.length()+1];
        for (int i = 0; i <= string.length(); i++) {
            list[i] = i;
        }
        return list;
    }

    public int[] step(int[] state, char c) {
        int[] next = new int[state.length];
        next[0] = state[0] + 1;
        for (int i = 1; i <= state.length; i++) {
            int cost = string.charAt(i-1) == c ? 0 : 1;
            next[i] = Math.min(state[i] + 1,
                               Math.min(next[i - 1] + 1,
                                        state[i - 1] + cost));
        }
        return next;
    }

    public ArrayList<Character> transitions(int[] state) {
        ArrayList<Character> list = new ArrayList<Character>();
        for (int i = 0; i < string.length(); i++) {
            if (state[i+1] <= maxEdits) {
                list.add(string.charAt(i));
            }
        }
        return list;
    }
}

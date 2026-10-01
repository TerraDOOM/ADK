public class LevState {
    public String pattern;
    public int c, e;
    public int maxDist;

    public LevState(String pat, int _c, int _e, int _maxDist) {
        pattern = pat;
        c = _c;
        e = _e;
        maxDist = _maxDist;
    }

    public bool isAccepting() {
        return c + (maxDist - e) >= pattern.length();
    }
}

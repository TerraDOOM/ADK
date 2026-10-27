import java.util.HashMap;

class BkNode {
    public String word;
    // public HashMap<Integer, BkNode> children = new HashMap<Integer, BkNode>();
    public BkNode[] children = new BkNode[]{};
    public int[] distances = new int[]{};

    public BkNode(String _word) {
        word = _word;
    }

    int dist(String other) {
        return levDynamic(word, other);
    }

    public void addChild(int dist, BkNode node) {
        if (distances.length == 0) {
            distances = new int[]{dist};
            children = new BkNode[]{node};
            return;
        }

        int len = distances.length;
        var newDistances = new int[len + 1];
        var newChildren = new BkNode[len + 1];
        System.arraycopy(distances, 0, newDistances, 0, len);
        System.arraycopy(children, 0, newChildren, 0, len);
        newDistances[len] = dist;
        newChildren[len] = node;
        distances = newDistances;
        children = newChildren;
    }

    public BkNode getDistance(int dist) {
        for (int i = 0; i < distances.length; i++) {
            if (distances[i] == dist) {
                return children[i];
            }
        }
        return null;
    }

    static int levDynamic(String w1, String w2) {
        int m = w1.length();
        int n = w2.length();

        if (n > m) {
            return levDynamic(w2, w1);
        }

        int[] last = new int[n + 1];
        int[] cur = new int[n + 1];

        for (int i = 0; i <= n; i++) {
            last[i] = i;
            cur[i] = 0;
        }

        for (int i = 1; i <= m; i++) {
            cur[0] = i;
            for (int j = 1; j <= n; j++) {
                int cost = w1.charAt(i - 1) == w2.charAt(j - 1) ? 0 : 1;
                cur[j] = Math.min(last[j] + 1,
                        Math.min(cur[j - 1] + 1,
                                last[j - 1] + cost));
            }
            int[] tmp = last;
            last = cur;
            cur = tmp;
        }
        return last[n];
    }

    void insert(String search) {
        var dist = levDynamic(word, search);
        if (dist == 0) {
            return;
        } else {
            var child = getDistance(dist);
            if (child != null) {
                child.insert(search);
            } else {
                addChild(dist, new BkNode(search));
            }
        }
    }
}

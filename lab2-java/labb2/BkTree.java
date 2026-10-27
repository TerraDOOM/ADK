import java.util.ArrayList;

class BkTree {
    BkNode root;

    public BkTree() {
        root = null;
    }

    public void insert(String word) {
        if (root == null) {
            root = new BkNode(word);
        } else {
            root.insert(word);
        }
    }

    public ArrayList<String> search(String word, int max) {
        var remaining = new ArrayList<BkNode>();
        var candidates = new ArrayList<String>();
        remaining.add(root);
        var minDist = max;


        while (!remaining.isEmpty()) {
            var cur = remaining.removeLast();
            var dist = cur.dist(word);
            // System.err.println("dist(" + cur.word + ", " + word + ") = " + dist);
            if (dist < minDist) {
                candidates.clear();
                candidates.add(cur.word);
                minDist = dist;
            } else if (dist <= minDist) {
                candidates.add(cur.word);
            }
            for (int i = 0; i < cur.distances.length; i++) {
                // System.err.println("dist to child " + cur.children[i] + ": " + Math.abs(dist - cur.distances[i]));
                if (Math.abs(dist - cur.distances[i]) <= minDist) {
                    remaining.add(cur.children[i]);
                }
            }
        }

        // System.out.println("search(" + word + ", " + max + "): " + candidates);

        return candidates;
    }
}

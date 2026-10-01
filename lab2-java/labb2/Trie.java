import java.util.ArrayList;

public class Trie {
    ArrayList<TrieNode> roots = new ArrayList<TrieNode>();

    public Trie(ArrayList<String> words) {
        for (String word : words) {
            add(word);
        }
    }

    public void add(String s) {
        char c = s.charAt(0);
        for (TrieNode root : roots) {
            if (root.prefix == c) {
                root.add(s.substring(1), word);
                return;
            }
        }
        roots.add(new TrieNode(s, word));
    }

    public ArrayList<TrieNode> get(String prefix) {
        if (prefix.isEmpty()) {
            return roots;
        }
        char c = prefix.charAt(0);
        for (var root : roots) {
            if (root.prefix == c) {
                return root.get(prefix.substring(1));
            }
        }
        return null;
    }
}

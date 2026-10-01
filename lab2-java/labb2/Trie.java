import java.util.ArrayList;

public class Trie {
    TrieNode root;

    public Trie(ArrayList<String> words) {
        root = new TrieNode("", null);
        for (String word : words) {
            root.add(word, word);
        }
    }

    public void add(String s) {
        root.add(s, s);
    }

    public TrieNode get(String prefix) {
        return root.get(prefix);
    }
}

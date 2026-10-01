import java.util.ArrayList;

public class TrieNode {
    public char prefix;
    public String terminal = null;
    public ArrayList<TrieNode> children;

    public TrieNode(String s, String word) {
        prefix = s.charAt(0);
        children = new ArrayList<TrieNode>();
        if (s.length() > 1) {
            children.add(new TrieNode(s.substring(1), word));
        } else {
            terminal = word;
        }
    }

    public void add(String s, String word) {
        if (s.length() == 0) {
            terminal = word;
            return;
        }
        char c = s.charAt(0);
        for (int i = 0; i < children.size(); i++) {
            if (children.get(i).prefix == c) {
                var node = children.get(i);
                node.add(s.substring(1), word);
                return;
            }
        }
        children.add(new TrieNode(s, word));
    }

    public ArrayList<TrieNode> get(String prefix) {
        if (prefix.isEmpty()) {
            return children;
        }
        char c = prefix.charAt(0);
        for (var child : children) {
            if (child.prefix == c) {
                return child.get(prefix.substring(1));
            }
        }
        return null;
    }
}

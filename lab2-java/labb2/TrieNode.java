import java.util.HashMap;

public class TrieNode {
    public char[] prefixes = null;
    public TrieNode[] children = null;
    public String terminal = null;

    public TrieNode(String s, String word) {
        if (s.length() > 0) {
            children = new TrieNode[]{new TrieNode(s.substring(1), word)};
            prefixes = new char[]{s.charAt(0)};
        } else {
            terminal = word;
        }
    }

    public void print() {
        print(0);
    }

    public void print(int indent) {
        if (terminal != null) {
            for (int i = 0; i < indent; i++) {
                System.out.print("  ");
            }

            System.out.println(terminal);
        }
        for (int i = 0; i < prefixes.length; i++) {
            for (int j = 0; j < indent; j++) {
                System.out.print("  ");
            }
            System.out.println(prefixes[i]);
            entry.getValue().print(indent + 1);
        }
    }

    public void add(String s, String word) {
        if (s.length() == 0) {
            terminal = word;
            return;
        }
        char c = s.charAt(0);
        for (var entry : children.entrySet()) {
            if (entry.getKey() == c) {
                entry.getValue().add(s.substring(1), word);
                return;
            }
        }
        children.put(s.charAt(0), new TrieNode(s.substring(1), word));
    }

    public TrieNode get(String prefix) {
        if (prefix.isEmpty()) {
            return this;
        }
        char c = prefix.charAt(0);
        for (var child : children.entrySet()) {
            if (child.getKey() == c) {
                return child.getValue();
            }
        }
        return null;
    }
}

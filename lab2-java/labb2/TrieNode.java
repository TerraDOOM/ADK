import java.util.HashMap;

public class TrieNode {
    public char[] prefix = null;
    public TrieNode[] children = null;
    public String terminal = null;
    int size = 0;

    public TrieNode(String s, String word) {
        if (s.length() > 0) {
            children = new TrieNode[]{new TrieNode(s.substring(1), word)};
            prefix = new char[] { s.charAt(0) };
            size += 1;
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
        for (int i = 0; i < length(); i++) {
            for (int j = 0; j < indent; j++) {
                System.out.print("  ");
            }
            System.out.println(prefix[i]);
            children[i].print(indent + 1);
        }
    }

    public void append(char c, TrieNode node) {
        if (prefix == null) {
            prefix = new char[]{c};
            children = new TrieNode[]{node};
            return;
        }

        int len = prefix.length;
        var newPrefixes = new char[len + 1];
        var newChildren = new TrieNode[len + 1];
        System.arraycopy(prefix, 0, newPrefixes, 0, len);
        System.arraycopy(children, 0, newChildren, 0, len);
        newPrefixes[len] = c;
        newChildren[len] = node;
        prefix = newPrefixes;
        children = newChildren;
    }

    public void add(String s, String word) {
        if (s.length() == 0) {
            terminal = word;
            return;
        }
        size += 1;
        if (prefix == null) {
            append(s.charAt(0), new TrieNode(s.substring(1), word));
            return;
        }
        char c = s.charAt(0);
        for (int i = 0; i < prefix.length; i++) {
            if (prefix[i] == c) {
                children[i].add(s.substring(1), word);
                return;
            }
        }
        append(s.charAt(0), new TrieNode(s.substring(1), word));
    }

    public TrieNode get(String s) {
        if (s.isEmpty()) {
            return this;
        }
        char c = s.charAt(0);
        for (int i = 0; i < prefix.length; i++) {
            if (prefix[i] == c) {
                return children[i];
            }
        }
        return null;
    }

    public TrieNode getChild(int index) {
        return children[index];
    }

    public char getPrefix(int index) {
        return prefix[index];
    }

    public int length() {
        return prefix == null ? 0 : prefix.length;
    }
}

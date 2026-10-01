/* Labb 2 i DD2350 Algoritmer, datastrukturer och komplexitet    */
/* Se labbinstruktionerna i kursrummet i Canvas                  */
/* Ursprunglig författare: Viggo Kann KTH viggo@kth.se           */
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

public class ClosestWords {
    ArrayList<String> closestWords = new ArrayList<String>();
    WordList wordList;
    Trie trie;

    int closestDistance;

    public Trie buildTrie(WordList list) {
        return new Trie(list.words);
    }

    static int partDist(String w1, String w2, int w1len, int w2len) {
        if (w1len == 0)
            return w2len;
        if (w2len == 0)
            return w1len;
        int res = partDist(w1, w2, w1len - 1, w2len - 1) +
                (w1.charAt(w1len - 1) == w2.charAt(w2len - 1) ? 0 : 1);
        int addLetter = partDist(w1, w2, w1len - 1, w2len) + 1;
        if (addLetter < res)
            res = addLetter;
        int deleteLetter = partDist(w1, w2, w1len, w2len - 1) + 1;
        if (deleteLetter < res)
            res = deleteLetter;
        return res;
    }

    public void search(TrieNode node, String word, char letter, int[] last) {
        int len = last.length;
        var state = new int[len];
        state[0] = last[0] + 1;
        int min = Integer.MAX_VALUE;
        for (int i = 1; i < len; i++) {
            int cost = word.charAt(i-1) == letter ? 0 : 1;
            state[i] = Math.min(last[i] + 1,
                                Math.min(state[i - 1] + 1,
                                         last[i - 1] + cost));
            min = Math.min(min, state[i]);
        }
        // System.err.printf("at distance %d on character %c\n", min, letter);

        if (state[len-1] <= closestDistance && node.terminal != null) {
            if (state[len-1] < closestDistance) {
                closestDistance = state[len - 1];
                closestWords.clear();
            }
            closestWords.add(node.terminal);
        }

        if (min <= closestDistance) {
            for (var next : node.children.entrySet()) {
                var nextNode = next.getValue();
                var nextLetter = next.getKey();
                search(nextNode, word, nextLetter, state);
            }
        }
    }

    int levDynamic(String w1, String w2) {
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

    int distance(String w1, String w2) {
        return levDynamic(w1, w2);
    }

    public ClosestWords(WordList _wordList) {
        wordList = _wordList;
        trie = new Trie(wordList.words);
    }

    void findMinWords(String s, int startCost) {
        closestWords.clear();
        closestDistance = startCost;
        int[] startingState = new int[s.length()+1];
        for (int i = 0; i < startingState.length; i++) {
            startingState[i] = i;
        }
        for (var entry : trie.root.children.entrySet()) {
            TrieNode node = entry.getValue();
            Character letter = entry.getKey();
            search(node, s, letter, startingState);
        }
    }

    void findMinWords(String s) {
        int worstCase = wordList.findClosestWordSize(s.length());
        for (int i = 1; i <= worstCase; i++) {
            findMinWords(s, i);
            if (!closestWords.isEmpty()) return;
        }
    }

    int getMinDistance() {
        return closestDistance;
    }

    ArrayList<String> getClosestWords() {
        var list = new ArrayList<String>();
        for (var word : closestWords) list.add(word);
        list.sort(String.CASE_INSENSITIVE_ORDER);
        return list;
    }
}

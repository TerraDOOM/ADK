/* Labb 2 i DD2350 Algoritmer, datastrukturer och komplexitet    */
/* Se labbinstruktionerna i kursrummet i Canvas                  */
/* Ursprunglig författare: Viggo Kann KTH viggo@kth.se           */
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;

public class ClosestWords {
    final static boolean useOptimism = true;

    ArrayList<String> closestWords = new ArrayList<String>();
    ArrayList<int[]> states = new ArrayList<int[]>();
    int optimisticLen = 0;
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

    public void search(TrieNode node, String word, char letter, int lastIdx) {
        int[] last = states.get(lastIdx);
        int len = last.length;
        if (lastIdx == states.size() - 1) {
            states.add(new int[last.length]);
        }
        var state = states.get(lastIdx + 1);
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
            searchNext(node, min, word, lastIdx + 1);
        }
    }

    void searchNext(TrieNode node, int min, String word, int stateIdx) {
        if (min < closestDistance) {
            for (int i = 0; i < node.length(); i++) {
                var nextNode = node.getChild(i);
                var nextLetter = node.getPrefix(i);
                search(nextNode, word, nextLetter, stateIdx);
            }
        } else if (min == closestDistance) {
            for (int i = 0; i < node.length(); i++) {
                var nextNode = node.getChild(i);
                var nextLetter = node.getPrefix(i);
                for (int j = 1; j <= word.length(); j++) {
                    if (nextLetter == word.charAt(j - 1)) {
                        search(nextNode, word, nextLetter, stateIdx);
                        break;
                    }
                }
            }
        }
    }

    int levDynamic(String w1, String w2, int max) {
        int m = w1.length();
        int n = w2.length();

        if (n > m) {
            return levDynamic(w2, w1, max);
        }

        int[] last = new int[n + 1];
        int[] cur = new int[n + 1];

        int min = 0;

        for (int i = 0; i <= n; i++) {
            last[i] = i;
            cur[i] = 0;
        }

        for (int i = 1; i <= m; i++) {
            cur[0] = i;
            min = cur[0];
            for (int j = 1; j <= n; j++) {
                int cost = w1.charAt(i - 1) == w2.charAt(j - 1) ? 0 : 1;
                cur[j] = Math.min(last[j] + 1,
                        Math.min(cur[j - 1] + 1,
                                last[j - 1] + cost));
                min = Math.min(cur[j], min);
            }
            if (min > max) {
                return min;
            }
            int[] tmp = last;
            last = cur;
            cur = tmp;
        }
        return last[n];
    }

    int distance(String w1, String w2, int max) {
        return levDynamic(w1, w2, max);
    }

    public ClosestWords(WordList _wordList) {
        wordList = _wordList;
        trie = new Trie(wordList.words);
    }

    TrieNode findOptimistic(String s) {
        var node = trie.root;
        for (optimisticLen = 0; optimisticLen < s.length(); optimisticLen++) {
            var c = s.charAt(optimisticLen);
            boolean cont = false;
            for (int j = 0; j < node.length(); j++) {
                if (c == node.getPrefix(j)) {
                    node = node.getChild(j);
                    cont = true;
                    break;
                }
            }
            if (!cont) {
                break;
            }
        }
        return node;
    }

    int trimPrefixIdx(String s, String word) {
        int minLength = Math.min(s.length(), word.length());
        int i = 0;
        int j = 0;
        while (s.charAt(i++) == word.charAt(j++)) {
            if (i >= minLength)
                break;
        }
        return i - 1;
    }

    void findMinWords(String s) {
        closestWords.clear();
        closestDistance = wordList.findClosestWordSize(s.length());
        for (int i = 0; i < wordList.words.size(); i++) {
            String word = wordList.words.get(i);
            int prefixLen = trimPrefixIdx(s, word);
            int dist;
            if (prefixLen == s.length()) {
                dist = word.length() - i;
            } else if (prefixLen == word.length()) {
                dist = s.length() - prefixLen;
            } else {
                dist = distance(s.substring(prefixLen), word.substring(prefixLen), closestDistance);
            }
            if (dist < closestDistance) {
                closestDistance = dist;
                closestWords.clear();
                closestWords.add(wordList.words.get(i));
            } else if (dist == closestDistance) {
                closestWords.add(wordList.words.get(i));
            }
        }
    }

    void findMinWords(String s, int startCost) {
        closestWords.clear();
        closestDistance = startCost;
        states.clear();
        int[] startingState = new int[s.length()+1];
        states.add(startingState);
        for (int i = 0; i < startingState.length; i++) {
            startingState[i] = i;
        }
        for (int i = 0; i < trie.root.length(); i++) {
            TrieNode node = trie.root.getChild(i);
            Character letter = trie.root.getPrefix(i);
            search(node, s, letter, 0);
        }
    }

    void findMinWordsOptimistic(String s, TrieNode root, int startCost) {
        closestWords.clear();
        closestDistance = startCost;
        states.clear();
        int[] startingState = new int[s.length()+1];
        states.add(startingState);
        for (int i = 0; i < startingState.length; i++) {
            startingState[i] = i;
        }
        for (int i = 0; i < root.length(); i++) {
            TrieNode node = root.getChild(i);
            Character letter = root.getPrefix(i);
            search(node, s, letter, 0);
        }
    }

    void _findMinWords(String s) {
        int worstCase = wordList.findClosestWordSize(s.length());
        var optimisticRoot = findOptimistic(s);
        var optimisticString = s.substring(optimisticLen);
        for (int i = 1; i <= worstCase; i++) {
            if (useOptimism) {
                boolean done = false;
                findMinWordsOptimistic(optimisticString, optimisticRoot, i);
                if (!closestWords.isEmpty())
                    done = true;
                closestWords.clear();
                findMinWords(s, i);
                if (done)
                    return;
            } else {
                findMinWords(s, i);
                if (!closestWords.isEmpty()) return;
            }
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

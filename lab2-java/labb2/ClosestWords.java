/* Labb 2 i DD2350 Algoritmer, datastrukturer och komplexitet    */
/* Se labbinstruktionerna i kursrummet i Canvas                  */
/* Ursprunglig författare: Viggo Kann KTH viggo@kth.se           */
import java.util.ArrayList;

public class ClosestWords {
    final static boolean useOptimism = true;

    ArrayList<String> closestWords = new ArrayList<String>();
    int optimisticLen = 0;
    WordList wordList;

    int closestDistance;

    LevMatrix mat = new LevMatrix();

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


    int distance(String word, String search, int max) {
        int dist = mat.levDynamic(word, search, max);
        return dist;
    }

    public ClosestWords(WordList _wordList) {
        wordList = _wordList;
    }

    void findMinWords(String s) {
        closestWords.clear();
        mat.last = "";
        closestDistance = 40; // wordList.findClosestWordSize(s.length());
        for (int i = 0; i < wordList.words.size(); i++) {
            String word = wordList.words.get(i);
            int dist = mat.levDynamic(word, s, closestDistance);
            // mat.print(s);
            if (dist < closestDistance) {
                // System.err.println("found closer word " + word + " at distance " + dist);
                // System.err.println("last word " + wordList.words.get(Math.max(i - 1, 0)) + " at distance " + dist);
                // mat.print(s);
                closestWords.clear();
                closestDistance = dist;
            }
            if (dist <= closestDistance) {
                closestWords.add(word);
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

/* Labb 2 i DD2350 Algoritmer, datastrukturer och komplexitet    */
/* Se labbinstruktionerna i kursrummet i Canvas                  */
/* Ursprunglig författare: Viggo Kann KTH viggo@kth.se           */
import java.util.ArrayList;
import java.util.List;

public class ClosestWords {
    ArrayList<String> closestWords = new ArrayList<String>();

    int closestDistance = -1;

    int partDist(String w1, String w2, int w1len, int w2len) {
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

    int levDynamic(String w1, String w2) {
        int m = w1.length();
        int n = w2.length();

        if (n > m) {
            return levDynamic(w2, w1);
        }

        int[] last = new int[n+1];
        int[] cur = new int[n+1];

        for (int i = 0; i <= n; i++) {
            last[i] = i;
            cur[i] = 0;
         }

         for (int i = 1; i <= m; i++) {
             cur[0] = i;
             for (int j = 1; j <= n; j++) {
                 int cost = w1.charAt(i-1) == w2.charAt(j-1) ? 0 : 1;
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

    public ClosestWords(String w, List<String> wordList) {
        for (String s : wordList) {
            int dist = distance(w, s);
            // System.out.println("d(" + w + "," + s + ")=" + dist);
            if (dist < closestDistance || closestDistance == -1) {
                closestDistance = dist;
                closestWords.clear();
                closestWords.add(s);
            }
            else if (dist == closestDistance)
                closestWords.add(s);
        }
    }

    int getMinDistance() {
        return closestDistance;
    }

    List<String> getClosestWords() {
        return closestWords;
    }
}

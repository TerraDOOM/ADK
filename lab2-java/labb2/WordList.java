import java.util.HashMap;
import java.util.ArrayList;

public class WordList {
    public ArrayList<Integer> offsets;
    public ArrayList<String> words;

    public WordList(ArrayList<String> _words) {
        var cmp = new StupidComparatorThatIHate();
        words = _words;
        words.sort(cmp);
        offsets = new ArrayList<Integer>();
        int currentSize = 0;
        for (int i = 0; i < words.size(); i++) {
            var word = words.get(i);
            if (word.length() > currentSize) {
                currentSize = word.length();
                offsets.add(i);
            }
        }
    }

    public int getWordsOfSize(int size) {
        for (int i = 0; i < offsets.size(); i++) {
            if (words.get(offsets.get(i)).length() > size) {
                return offsets.get(i) - offsets.get(i - 1);
            }
        }
        return 0;
    }
}

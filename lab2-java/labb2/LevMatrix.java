class LevMatrix {
    int[] mat = new int[41 * 41];
    String last = "";
    String lastSearch = null;
    int lastEnd = 40;
    int lastCost = 0;

    public void set(int i, int j, int v) {
        mat[i * 41 + j] = v;
    }

    public int get(int i, int j) {
        return mat[i * 41 + j];
    }

    public LevMatrix() {
        for (int j = 0; j < 41; j++) {
            set(0, j, j);
        }
        for (int i = 0; i < 41; i++) {
            set(i, 0, i);
        }
    }

    public int levDynamic(String word, String search, int max) {
        // System.err.println("word is " + word);
        // System.err.println("search is " + search);

        int m = word.length();
        int n = search.length();

        int prefixLen = 0;
        if (search == lastSearch) {
            for (prefixLen = 0; prefixLen < Math.min(word.length(), last.length())
                     && word.charAt(prefixLen) == last.charAt(prefixLen); prefixLen++)
                ;
        }
        lastSearch = search;
        int min = 0;

        for (int i = Math.clamp(prefixLen, 1, Math.min(m, lastEnd)); i <= m; i++) {
            min = get(i, 0);
            for (int j = 1; j <= n; j++) {
                int cost = word.charAt(i - 1) == search.charAt(j - 1) ? 0 : 1;

                int val = Math.min(get(i-1, j) + 1,
                                   Math.min(get(i, j-1) + 1,
                                            get(i-1, j-1) + cost));
                set(i, j, val);
                min = Math.min(min, val);
            }
            if (min > max) {
                last = word;
                lastEnd = i;
                return min;
            }
        }

        lastEnd = word.length();
        last = word;
        return get(m, n);
    }

    public void print(String search) {

        System.out.print("      ");
        for (char c : search.toCharArray()) {
            System.out.printf("%3c", c);
        }
        System.out.println();
        for (int i = 0; i <= Math.max(last.length(), search.length()); i++) {
            if (i > 0 && i <= last.length()) {
                System.out.print(last.charAt(i - 1) + "  ");
            } else {
                System.out.print("   ");
            }
            for (int j = 0; j <= Math.max(last.length(), search.length()); j++) {
                System.out.printf("%3d", get(i, j));
            }
            System.out.println();
        }
    }
}

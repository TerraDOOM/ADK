class LevMatrix {
    byte[] mat = new byte[41 * 41];
    String last = "";
    String lastSearch = null;
    int lastEnd = 40;
    int lastCost = 0;

    public LevMatrix() {
        for (byte j = 0; j < 41; j++) {
            mat[0 * 41 + j] = j;
        }
        for (byte i = 0; i < 41; i++) {
            mat[i * 41 + 0] = i;
        }
    }

    public int levDynamic(String word, String search, int max) {
        // System.err.println("word is " + word);
        // System.err.println("search is " + search);

        int m = word.length();
        int n = search.length();

        int prefixLen = 0;
        for (prefixLen = 0; prefixLen < Math.min(word.length(), last.length())
                 && word.charAt(prefixLen) == last.charAt(prefixLen); prefixLen++)
            ;
        int min = 0;

        for (int i = Math.clamp(prefixLen+1, 1, Math.min(m, lastEnd)); i <= m; i++) {
            min = i;
            for (int j = 1; j <= n; j++) {
                int cost = word.charAt(i - 1) == search.charAt(j - 1) ? 0 : 1;

                byte val = (byte)Math.min(mat[(i-1) * 41 + j] + 1,
                                          Math.min(mat[i * 41 + j - 1] + 1,
                                                   mat[(i - 1) * 41 + j - 1] + cost));
                mat[i * 41 + j] = val;
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
        return mat[m * 41 + n];
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
                System.out.printf("%3d", mat[i * 41 + j]);
            }
            System.out.println();
        }
    }
}

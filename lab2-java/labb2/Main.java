/* Labb 2 i DD2350 Algoritmer, datastrukturer och komplexitet    */
/* Se labbinstruktionerna i kursrummet i Canvas                  */
/* Ursprunglig författare: Viggo Kann KTH viggo@kth.se           */
import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.File;
import java.io.FileReader;
import java.io.InputStreamReader;
import java.time.Duration;
import java.time.Instant;
import java.time.temporal.ChronoUnit;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Random;

public class Main {

    public static final String DEFAULT_DIR = "./test";
    public static final boolean PRINT_TIMINGS = false;

    public static WordList readWordList(BufferedReader input) throws IOException {
        ArrayList<String> list = new ArrayList<String>();
        while (true) {
            String s = input.readLine();
            if (s.equals("#"))
                break;
            list.add(s);
        }

        WordList wordList = new WordList(list);
        return wordList;
    }


    public static void main(String args[]) throws IOException {
        if (parseArgs(args)) {
            System.exit(0);
        }

        long t1 = System.currentTimeMillis();
        BufferedReader stdin = new BufferedReader(new InputStreamReader(System.in, "UTF-8"));
        // Säkrast att specificera att UTF-8 ska användas, för vissa system har annan
        // standardinställning för teckenkodningen.
        WordList wordList = readWordList(stdin);
        String word;
        ClosestWords closestWords = new ClosestWords(wordList);
        if (PRINT_TIMINGS)
           System.out.println("built trie in " + (System.currentTimeMillis() - t1) + " ms");
       if (PRINT_TIMINGS && wordList.words.size() < 20)
           closestWords.trie.root.print();
       while ((word = stdin.readLine()) != null) {
           long innerTime = System.currentTimeMillis();
           closestWords.findMinWords(word);
           if (PRINT_TIMINGS)
               System.out.println(
                       "found solutions for " + word + " in " + (System.currentTimeMillis() - innerTime) + " ms");
           System.out.print(word + " (" + closestWords.getMinDistance() + ")");
           for (String w : closestWords.getClosestWords())
               System.out.print(" " + w);
           System.out.println();
       }
       long tottime = (System.currentTimeMillis() - t1);
        if (PRINT_TIMINGS) System.out.println("total CPU time: " + tottime + " ms");
    }


    private static boolean parseArgs(String[] args) {
        boolean inTestZone = false;
        boolean beenInTestZone = false;
        boolean testRandom = false;
        boolean parseFile = false;
        String filename = null;
        int finishedTestBatches = 0;
        int testsToRun = 100;
        if (args.length >= 1) {
            for (String current: args) {
                if (current.equals("-t")) {
                    inTestZone = true;
                    beenInTestZone = true;
                    continue;
                }
                if (current.equals("-r")) {
                    testRandom = true;
                    continue;
                }
                if (current.equals("-f")) {
                    parseFile = true;
                    continue;
                }
                if (parseFile) {
                    filename = current;
                    parseFile = false;
                    continue;
                }
                if (testRandom) {
                    testsToRun = Integer.parseInt(current);
                    continue;
                }
                if (inTestZone) {
                    if (current.startsWith("-")) {
                        inTestZone = false;
                        continue;
                    }
                    runTestsInCatalogue(current);
                    finishedTestBatches += 1;
                }
            }
        }
        if (testRandom) {
            try {
                BufferedReader file;
                if (filename != null) {
                    file = new BufferedReader(new InputStreamReader(new FileInputStream(filename), "UTF-8"));
                } else {
                    file = new BufferedReader(new InputStreamReader(System.in, "UTF-8"));
                }
                runRandomizedTests(testsToRun, file);
            } catch (IOException fuckthislanguage) {}
            return true;
        }
        if(beenInTestZone && finishedTestBatches == 0) {
            runTestsInCatalogue(DEFAULT_DIR);
            finishedTestBatches++;
        }
        return finishedTestBatches > 0;
    }

    public static void runRandomizedTests(int count, BufferedReader in) throws IOException {

        // Säkrast att specificera att UTF-8 ska användas, för vissa system har annan
        // standardinställning för teckenkodningen.
        WordList wordList = readWordList(in);
        ClosestWords closestWords = new ClosestWords(wordList);
        Random rng = new Random();
        char[] letters = { 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'å', 'ä', 'ö' };
        HashMap<Integer, ArrayList<Integer>> runtimes = new HashMap<Integer, ArrayList<Integer>>();
        System.out.println("Trie size: " + closestWords.trie.root.size);
        for (int i = 0; i < count; i++) {
            int selectedWord = rng.nextInt(wordList.words.size());
            String word = wordList.words.get(selectedWord);
            StringBuilder sb = new StringBuilder(word);
            for (int j = 0; j < 4; j++) {
                int pos = rng.nextInt(word.length());
                char newLetter = letters[rng.nextInt(letters.length)];
                sb.replace(pos, pos + 1, String.valueOf(newLetter));
            }
            String changed = sb.toString();
            Instant t1 = Instant.now();
            closestWords.findMinWords(changed);
            Duration elapsed = Duration.between(t1, Instant.now());
            int elapsedMicros = (int) elapsed.toNanos() / 1000;
            int foundAnswer = closestWords.closestDistance;
            int foundCount = closestWords.closestWords.size();
            closestWords._findMinWords(changed);
            int correctAnswer = closestWords.closestDistance;
            int correctCount = closestWords.closestWords.size();
            if (correctAnswer != foundAnswer) {
                System.err.println(
                        "incorrect answer for " + changed + ": got " + foundAnswer + " expected " + correctAnswer);

            }
            if (correctCount != foundCount) {
                System.err.println(
                        "incorrect answer for " + changed + ": got " + foundCount + " expected " + correctCount);
            }
            if (!runtimes.containsKey(closestWords.closestDistance)) {
                var list = new ArrayList<Integer>();
                list.add(elapsedMicros);
                runtimes.put(closestWords.closestDistance, list);
            } else {
                runtimes.get(closestWords.closestDistance).add(elapsedMicros);
            }
            System.out.println("Found words within dist " + closestWords.closestDistance + " in "
                               + elapsed.toNanos() / 1000 + "μs " + (foundAnswer == correctAnswer && foundCount == correctCount ? "(correct)": "(incorrect)"));
        }
        for (var kv : runtimes.entrySet()) {
            int total = 0;
            for (var micros : kv.getValue()) {
                total += micros;
            }
            double avg = (double) total / (double) kv.getValue().size();
            System.err.println("Average time for distance " + kv.getKey() + ": " + avg + "μs");
        }
    }


    public static void runTestsInCatalogue(String folder) {
        System.out.println("Processing folder: " + folder);
        File f = new File(folder);
        String[] filenames = f.list();
        Arrays.sort(filenames);
        final int N_FILES = filenames.length;
        for(int i=0; i<N_FILES; i++) {
            String current = filenames[i];
            if(current.endsWith(".indata")) { // Potential testcase
                int pointPos = current.indexOf(".");
                String testName = current.substring(0, pointPos);
                String outFileName = testName + ".utdata";
                int outFileIndex = Arrays.binarySearch(filenames, 0, N_FILES, outFileName);
                if(outFileIndex >= N_FILES || !filenames[outFileIndex].equals(outFileName)) {
                    System.out.println("Skipping testcase " + testName + " because I could not find matching answers file.");
                    continue;
                }
                runSingleTestCase(folder, testName, current, outFileName);
            }
        }
    }

    public static void runSingleTestCase(String path, String name, String in, String ans) {
        System.out.println("Processing testcase: " + name);
        long t1 = System.currentTimeMillis();
        BufferedReader inFile = null;
        BufferedReader ansFile = null;
        final String inFilePath = path + "/" + in;
        final String outFilePath = path + "/" + ans;
        try {
            inFile = new BufferedReader(new InputStreamReader(new FileInputStream(inFilePath), "UTF-8"));
            ansFile = new BufferedReader(new InputStreamReader(new FileInputStream(outFilePath), "UTF-8"));
        } catch(Exception e) {
            System.out.println("At least one of the files would not open.");
            e.printStackTrace();
            System.exit(4);
        }
        // Säkrast att specificera att UTF-8 ska användas, för vissa system har annan
        // standardinställning för teckenkodningen.
        WordList wordList = null;
        try {
            wordList = readWordList(inFile);
        } catch (Exception e) {
            System.out.println("Could not read the wordList of this testcase.");
            System.exit(1);
        }
        String word;
        try {
            ClosestWords closestWords = new ClosestWords(wordList);

            while ((word = inFile.readLine()) != null) {
                closestWords.findMinWordsBk(word);
                String answerLine;
                if ((answerLine = ansFile.readLine()) == null) {
                    System.err.println("The file " + ans + " ran out of answers but there were questions left in " + in + ".");
                    System.exit(2);
                }
                Iterator<String> it = (Arrays.asList(answerLine.split("\\s"))).iterator();
                String rightWord = it.next();
                if (!rightWord.equals(word)) {
                    System.out.println("Wrong word. Expected " + rightWord + " and found " + word);
                }
                String candidateDistance = "(" + closestWords.getMinDistance() + ")";
                String rightDistance = it.next();
                if (!candidateDistance.equals(rightDistance)) {
                    System.out.println(
                            "Wrong minimum distance. Expected " + rightDistance + " and found " + candidateDistance);
                }
                for (String w : closestWords.getClosestWords()) {
                    if(!it.hasNext()) {
                        System.out.println("Suggested solution has more words than the correct solution.");
                        break;
                    }
                    rightWord = it.next();
                    if (!w.equals(rightWord)) {
                        System.out.println("Wrong word. Expected " + rightWord + " but found " + w);
                    }
                }
            }
        } catch(Exception e) {
            System.out.println("File reading error while processing this testcase!");
            e.printStackTrace();
            System.exit(3);
        }
        long tottime = (System.currentTimeMillis() - t1);
        System.out.println("CPU time for this test: " + tottime + " ms");
    }
}

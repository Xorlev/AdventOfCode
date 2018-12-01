package aoc2016;

import aoc.Util;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * Highest frequency character in each position reveals the message
 *
 * An indexed list of maps collects frequency information with a final scan to find the most frequent
 *
 * @author Michael Rose (xorlev)
 */
public class Six {
    public static void main(String[] args) {
        List<String> messages = Util.input(2016, 6);

        List<Map<Character, Integer>> columnFrequencies = new ArrayList<>(8);
        for(int i = 0; i < 8; i++) {
            columnFrequencies.add(new HashMap<>());
        }

        for(String message : messages) {
            for(int c = 0; c < message.length(); c++) {
                Map<Character, Integer> frequencyMap = columnFrequencies.get(c);

                Character key = message.charAt(c);

                frequencyMap.put(key, frequencyMap.getOrDefault(key, 0) + 1);
            }
        }

        for(int i = 0; i < 8; i++) {
            System.out.print(mostFrequent(columnFrequencies.get(i)));
        }
        System.out.println();
    }

    static Character mostFrequent(Map<Character, Integer> f) {
        return f.entrySet().stream().min(Comparator.comparing(Map.Entry::getValue)).get().getKey();
    }
}

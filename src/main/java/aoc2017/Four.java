package aoc2017;

import aoc.Util;
import com.google.common.base.CharMatcher;
import com.google.common.base.Splitter;

import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Four {
    public static void main(String[] args) {
        List<String> input = Util.input(2017, 4);

        Util.assertThat(isValid("aa bb cc dd ee")).isEqualTo(true);
        Util.assertThat(isValid("aa bb cc dd aa")).isEqualTo(false);
        Util.assertThat(isValid("aa bb cc dd aaa")).isEqualTo(true);

        int valid = 0;
        for(String passphrase : input) {
            if (isValid(passphrase)) {
                valid++;
            }
        }

        Util.assertThat(isValid2("abcde fghij")).isEqualTo(true);
        Util.assertThat(isValid2("abcde xyz ecdab")).isEqualTo(false);
        Util.assertThat(isValid2("a ab abc abd abf abj")).isEqualTo(true);
        Util.assertThat(isValid2("iiii oiii ooii oooi oooo")).isEqualTo(true);
        Util.assertThat(isValid2("oiii ioii iioi iiio")).isEqualTo(false);

        int valid2 = 0;
        for(String passphrase : input) {
            if (isValid2(passphrase)) {
                valid2++;
            }
        }

        System.out.println("p1: " + valid);
        System.out.println("p2: " + valid2);
    }

    private static boolean isValid(String passphrase) {
        Set<String> words = new HashSet<>();

        for (String word : Splitter.on(CharMatcher.BREAKING_WHITESPACE).splitToList(passphrase)) {
            if (words.contains(word)) {
                return false;
            } else {
                words.add(word);
            }
        }

        return true;
    }

    private static boolean isValid2(String passphrase) {
        Set<String> words = new HashSet<>();

        for (String word : Splitter.on(CharMatcher.BREAKING_WHITESPACE).splitToList(passphrase)) {
            char[] letters = word.toCharArray();
            Arrays.sort(letters);
            String sortedWord = new String(letters);

            if (words.contains(sortedWord)) {
                return false;
            } else {
                words.add(sortedWord);
            }
        }

        return true;
    }
}

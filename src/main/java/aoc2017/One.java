package aoc2017;

import aoc.Util;
import com.google.common.collect.Iterables;

/**
 * The captcha requires you to review a sequence of digits (your puzzle input) and find the sum of all digits that match
 * the next digit in the list. The list is circular, so the digit after the last digit is the first digit in the list.
 * <p>
 * For example:
 * <p>
 * 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
 * 1111 produces 4 because each digit (all 1) matches the next.
 * 1234 produces 0 because no digit matches the next.
 * 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
 */
public class One {
    public static void main(String[] args) {
        String input = Iterables.getOnlyElement(Util.input(2017, 1));

        Util.assertThat(3).isEqualTo(solvePartOne("1122"));
        Util.assertThat(4).isEqualTo(solvePartOne("1111"));
        Util.assertThat(0).isEqualTo(solvePartOne("1234"));
        Util.assertThat(9).isEqualTo(solvePartOne("91212129"));

        System.out.println("Part 1 solution: " + solvePartOne(input));

        Util.assertThat(6).isEqualTo(solvePartTwo("1212"));
        Util.assertThat(0).isEqualTo(solvePartTwo("1221"));
        Util.assertThat(4).isEqualTo(solvePartTwo("123425"));
        Util.assertThat(12).isEqualTo(solvePartTwo("123123"));
        Util.assertThat(4).isEqualTo(solvePartTwo("12131415"));

        System.out.println("Part 2 solution: " + solvePartTwo(input));
    }

    /**
     * Returns the sum of all digits that match.
     */
    private static int solvePartOne(String captcha) {
        return solve(captcha, 1);
    }

    /**
     * Now, instead of considering the next digit, it wants you to consider the digit halfway around the circular list.
     * That is, if your list contains 10 items, only include a digit in your sum if the digit 10/2 = 5 steps forward
     * matches it. Fortunately, your list has an even number of elements.
     * For example:

     1212 produces 6: the list contains 4 items, and all four digits match the digit 2 items ahead.
     1221 produces 0, because every comparison is between a 1 and a 2.
     123425 produces 4, because both 2s match each other, but no other digit has a match.
     123123 produces 12.
     12131415 produces 4.
     */
    private static int solvePartTwo(String captcha) {
        assert captcha.length() % 2 == 0;

        return solve(captcha, captcha.length() / 2);
    }

    /**
     * Returns the sum of all digits that match.
     */
    private static int solve(String captcha, int skip) {
        int sum = 0;
        for (int i = 0; i < captcha.length(); i++) {
            int digit = Integer.valueOf(captcha.substring(i, i+1));

            int nextIndex = (i+skip) % captcha.length();
            int nextDigit = Integer.valueOf(captcha.substring(nextIndex, nextIndex+1));

            if (digit == nextDigit) {
                sum += digit;
            }
        }

        return sum;
    }
}

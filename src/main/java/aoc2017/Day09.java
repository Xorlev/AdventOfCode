package aoc2017;

import aoc.Util;
import com.google.common.collect.Iterables;
import lombok.Value;

public class Day09 {
    public static void main(String[] args) {
        String input = Iterables.getOnlyElement(Util.input(2017, 9));

        Util.assertThat(scoreGroups("{}").score).isEqualTo(1);
        Util.assertThat(scoreGroups("{{{}}}").score).isEqualTo(6);
        Util.assertThat(scoreGroups("{{},{}}").score).isEqualTo(5);
        Util.assertThat(scoreGroups("{{{},{},{{}}}}").score).isEqualTo(16);
        Util.assertThat(scoreGroups("{<a>,<a>,<a>,<a>}").score).isEqualTo(1);
        Util.assertThat(scoreGroups("{{<ab>},{<ab>},{<ab>},{<ab>}}").score).isEqualTo(9);
        Util.assertThat(scoreGroups("{{<!!>},{<!!>},{<!!>},{<!!>}}").score).isEqualTo(9);
        Util.assertThat(scoreGroups("{{<a!>},{<a!>},{<a!>},{<ab>}}").score).isEqualTo(3);


        Util.assertThat(scoreGroups("<>").garbage).isEqualTo(0);
        Util.assertThat(scoreGroups("<<<<>").garbage).isEqualTo(3);
        Util.assertThat(scoreGroups("<{!>}>").garbage).isEqualTo(2);
        Util.assertThat(scoreGroups("<!!>").garbage).isEqualTo(0);
        Util.assertThat(scoreGroups("<!!!>>").garbage).isEqualTo(0);
        Util.assertThat(scoreGroups("<{o\"i!a,<{i<a>,").garbage).isEqualTo(10);
        Util.assertThat(scoreGroups("<random characters>").garbage).isEqualTo(17);

        Result result = scoreGroups(input);
        System.out.println("Part 1: " + result.score);
        System.out.println("Part 2: " + result.garbage);
    }

    /**
     * Your goal is to find the total score for all groups in your input. Each group is assigned a score which is one
     * more than the score of the group that immediately contains it. (The outermost group gets a score of 1.)
     * @param stream
     * @return
     */
    private static Result scoreGroups(String stream) {
        int nestingLevel = 0;
        int score = 0;
        int garbage = 0;

        boolean inGarbage = false;
        for (int i = 0; i < stream.length(); i++) {
            char currentChar = stream.charAt(i);

            // skip < if we're in garbage
            if (currentChar == '<' && inGarbage) {
                garbage++;
                continue;
            }

            // ! skip next character
            if (currentChar == '!' && inGarbage) {
                i++;
                continue;
            }

            if (currentChar == '<') {
                inGarbage = true;
            } else if (currentChar == '>') {
                inGarbage = false;
            } else if (!inGarbage && currentChar == '{') {
                nestingLevel++;
            } else if (!inGarbage && currentChar == '}') {
                score += nestingLevel;
                nestingLevel--;
            } else if (inGarbage) {
                garbage++;
            }
        }
        return new Result(score, garbage);
    }

    @Value
    static class Result {
        int score;
        int garbage;
    }
}

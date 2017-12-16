package aoc2017;

import aoc.Util;

public class Day15 {
    public static void main(String[] args) {
        Util.assertThat(findLowerMatches(
                new Generator(16807, 65, -1),
                new Generator(48271, 8921, -1),
                40_000_000))
                .isEqualTo(588);

        System.out.println("Part 1: " + findLowerMatches(
                new Generator(16807, 591, -1),
                new Generator(48271, 393, -1),
                40_000_000));
        Util.assertThat(findLowerMatches(
                new Generator(16807, 65, 4),
                new Generator(48271, 8921, 8),
                5_000_000))
                .isEqualTo(309);

        System.out.println("Part 2: " + findLowerMatches(
                new Generator(16807, 591, 4),
                new Generator(48271, 393, 8),
                5_000_000));
    }

    private static int findLowerMatches(Generator a, Generator b, int pairs) {
        int matches = 0;
        for (int i = 0; i < pairs; i++) {
            while(a.nextValue() < 0) {}
            while(b.nextValue() < 0) {}

            if (a.lower() == b.lower()) {
                matches++;
            }
        }
        return matches;
    }

    static class Generator {
        private final int factor;
        private int value;
        private final int multiple;

        public Generator(int factor, int value, int multiple) {
            this.factor = factor;
            this.value = value;
            this.multiple = multiple;
        }

        public int nextValue() {
            long next = (long)value * factor;
            next %= Integer.MAX_VALUE;
            value = (int)next;

            if (multiple > 0) {
                if (value % multiple != 0) {
                    return -1;
                }
            }

            return value;
        }

        public short lower() {
            return (short)value;
        }
    }
}

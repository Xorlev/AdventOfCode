package aoc2017;

import aoc.Util;
import com.google.common.base.Splitter;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.Iterables;

import java.util.ArrayList;
import java.util.List;

public class Day10 {
    public static void main(String[] args) {
        String sequence = Iterables.getOnlyElement(Util.input(2017, 10));
        ImmutableList<Integer> p1 =
                Splitter.on(',').splitToList(sequence)
                .stream()
                .map(Integer::parseInt)
                .collect(ImmutableList.toImmutableList());

        int[] testOutput = knotHash(ImmutableList.of(3, 4, 1, 5), generateInput(5), 1);
        Util.assertThat(testOutput[0] * testOutput[1]).isEqualTo(12);

        int[] part1Output = knotHash(p1, generateInput(256),1 );
        int part1 = part1Output[0] * part1Output[1];

        System.out.println("Part 1: " + part1);
        System.out.println("Part 2: " + knotHash(sequence));
    }

    static String knotHash(String input) {
        return toHexHash(knotHash(strToLengths(input), generateInput(256), 64));
    }

    private static int[] knotHash(ImmutableList<Integer> knotLengths, int[] input, int rounds) {
        int pos = 0;
        int skipSize = 0;
        int listLen = input.length;
        for (int r = 0; r < rounds; r++) {
            for (int length : knotLengths) {
                if (length > 1) {
                    int target = pos % listLen;
                    int destination = (pos + length - 1) % listLen;

                    for (int i = 0; i < length / 2; i++) {
                        int temp = input[target];
                        input[target] = input[destination];
                        input[destination] = temp;

                        target = (target + 1) % listLen;
                        destination--;

                        if (destination < 0) {
                            destination = listLen - 1;
                        }
                    }

                }

                pos = (pos + length + skipSize) % listLen;
                skipSize++;
            }
        }

        return input;
    }

    private static String toHexHash(int[] sparse) {
        List<Integer> dense = new ArrayList<>();

        for (int i = 0; i < sparse.length; i += 16) {
            int block = 0;
            for (int j = i; j < i+16; j++) {
                block ^= sparse[j];
            }
            dense.add(block);
        }

        StringBuilder sb = new StringBuilder();
        for (int value : dense) {
            sb.append(String.format("%02X", value));
        }

        return sb.toString();
    }

    private static int[] generateInput(int listLen) {
        int[] list = new int[listLen];
        for (int i = 0; i < listLen; i++) {
            list[i] = i;
        }
        return list;
    }

    private static ImmutableList<Integer> strToLengths(String input) {
        List<Integer> list = new ArrayList<>();
        for (int i = 0; i < input.length(); i++) {
            list.add((int) input.charAt(i));
        }
        list.addAll(ImmutableList.of(17, 31, 73, 47, 23));
        return ImmutableList.copyOf(list);
    }
}

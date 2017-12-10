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

        List<Integer> testOutput = knotHash(ImmutableList.of(3, 4, 1, 5), generateInput(5), 1);
        Util.assertThat(testOutput.get(0) * testOutput.get(1)).isEqualTo(12);

        List<Integer> part1Output = knotHash(p1, generateInput(256),1 );
        int part1 = part1Output.get(0) * part1Output.get(1);

        System.out.println("Part 1: " + part1);

        ImmutableList<Integer> p2 = part2Lengths(sequence);
        String part2 = toHexHash(knotHash(p2, generateInput(256), 64));

        System.out.println("Part 2: " + part2);
    }

    private static List<Integer> knotHash(ImmutableList<Integer> knotLengths, List<Integer> input, int rounds) {
        int pos = 0;
        int skipSize = 0;
        int listLen = input.size();
        for (int r = 0; r < rounds; r++) {
            for (int length : knotLengths) {
                if (length > 1) {
                    int target = pos % listLen;
                    int destination = (pos + length - 1) % listLen;

                    for (int i = 0; i < length / 2; i++) {
                        int temp = input.get(target);
                        input.set(target, input.get(destination));
                        input.set(destination, temp);

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

    private static String toHexHash(List<Integer> sparse) {
        List<Integer> dense = new ArrayList<>();

        for (int i = 0; i < sparse.size(); i += 16) {
            int block = 0;
            for (int j = i; j < i+16; j++) {
                block ^= sparse.get(j);
            }
            dense.add(block);
        }

        StringBuilder sb = new StringBuilder();
        for (int value : dense) {
            sb.append(Integer.toHexString(value));
        }

        return sb.toString();
    }

    private static List<Integer> generateInput(int listLen) {
        List<Integer> list = new ArrayList<>(listLen);
        for (int i = 0; i < listLen; i++) {
            list.add(i);
        }
        return list;
    }

    private static ImmutableList<Integer> part2Lengths(String input) {
        List<Integer> list = new ArrayList<>();
        for (int i = 0; i < input.length(); i++) {
            list.add((int) input.charAt(i));
        }
        list.addAll(ImmutableList.of(17, 31, 73, 47, 23));
        return ImmutableList.copyOf(list);
    }
}

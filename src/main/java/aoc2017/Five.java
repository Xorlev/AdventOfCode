package aoc2017;

import aoc.Util;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.Lists;

import java.util.List;
import java.util.function.Function;

public class Five {
    public static void main(String[] args) {
        List<String> input = Util.input(2017, 5);
        ImmutableList<Integer> offsets = input.stream()
                .map(Integer::valueOf)
                .collect(ImmutableList.toImmutableList());

        Util.assertThat(findExit(Lists.newArrayList(0, 3, 0, 1, -3))).isEqualTo(5);
        Util.assertThat(findExitPartTwo(Lists.newArrayList(0, 3, 0, 1, -3))).isEqualTo(10);

        System.out.println("Part 1: " + findExit(Lists.newArrayList(offsets)));
        System.out.println("Part 2: " + findExitPartTwo(Lists.newArrayList(offsets)));
    }

    private static int findExit(List<Integer> offsets) {
        return findExit(offsets, unused -> 1);
    }

    private static int findExitPartTwo(List<Integer> offsets) {
        return findExit(offsets, offset -> offset >= 3 ? -1 : 1);
    }

    private static int findExit(List<Integer> offsets, Function<Integer, Integer> jumpFn) {
        int steps = 0;
        int pc = 0;
        while(true) {
            steps++;

            int offset = offsets.get(pc);
            offsets.set(pc, offset + jumpFn.apply(offset));
            pc += offset;

            if(pc >= offsets.size()) {
                break;
            }
        }

        return steps;
    }
}

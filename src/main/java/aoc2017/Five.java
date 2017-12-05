package aoc2017;

import aoc.Util;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.Lists;

import java.util.List;

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
        int steps = 0;
        int pc = 0;
        while(true) {
            steps++;

            int offset = offsets.get(pc);
            offsets.set(pc, offset + 1);
            pc += offset;

            if(pc >= offsets.size()) {
                break;
            }
        }

        return steps;
    }

    private static int findExitPartTwo(List<Integer> offsets) {
        int steps = 0;
        int pc = 0;
        while(true) {
            steps++;

            int offset = offsets.get(pc);

            int modifier = 1;
            if (offset >= 3) {
                modifier = -1;
            }

            offsets.set(pc, offset + modifier);
            pc += offset;

            if(pc >= offsets.size()) {
                break;
            }
        }

        return steps;
    }
}

package aoc2017;

import aoc.Util;

import java.util.ArrayList;
import java.util.List;

public class Day17 {
    public static void main(String[] args) {
        Util.assertThat(spinlockP1(2017, 3)).isEqualTo(638);

        System.out.println("Part 1: " + spinlockP1(2017, 386));
        System.out.println("Part 2: " + spinlockP2(50_000_000, 386));
    }

    private static int spinlockP1(int iterations, int steps) {
        List<Integer> list = new ArrayList<>();
        list.add(0);

        int pos = 0;
        for (int i = 0; i < iterations; i++) {
            for (int j = 0; j < steps; j++) {
                pos = (pos+1)%list.size();
            }
            pos++;
            list.add(pos, i+1);
        }

        return list.get((pos+1)%list.size());
    }

    private static int spinlockP2(int iterations, int steps) {
        int value = 0;
        int pos = 0;
        for (int i = 1; i <= iterations; i++) {
            pos = (pos+steps)%(i) + 1;

            if (pos == 1) {
                value = i;
            }
        }

        return value;
    }
}

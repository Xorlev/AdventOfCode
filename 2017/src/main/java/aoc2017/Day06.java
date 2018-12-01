package aoc2017;

import aoc.Util;
import com.google.common.base.CharMatcher;
import com.google.common.base.Splitter;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.Iterables;
import com.google.common.collect.Lists;
import lombok.Value;

import java.util.*;

public class Day06 {
    public static void main(String[] args) {
        ImmutableList<Integer> banks = Splitter.on(CharMatcher.breakingWhitespace())
                .splitToList(Iterables.getOnlyElement(Util.input(2017, 6)))
                .stream()
                .map(Integer::valueOf)
                .collect(ImmutableList.toImmutableList());

        Util.assertThat(solve(ImmutableList.of(0, 2, 7, 0))).isEqualTo(new Result(5, 4));

        Result result = solve(banks);
        System.out.println("Part 1: " + result.totalSteps);
        System.out.println("Part 2: " + result.loopLength);
    }

    private static Result solve(ImmutableList<Integer> banks) {
        Map<ImmutableList<Integer>, Integer> seenConfigurations = new HashMap<>();
        seenConfigurations.put(banks, 0);
        while(true) {
            banks = rebalance(banks);

            if(seenConfigurations.containsKey(banks)) {
                int previousIndex = seenConfigurations.get(banks);

                return new Result(seenConfigurations.size(), seenConfigurations.size() - previousIndex);
            } else {
                seenConfigurations.put(banks, seenConfigurations.size());
            }
        }
    }

    private static ImmutableList<Integer> rebalance(ImmutableList<Integer> banks) {
        int maxBank = -1;
        int maxBankValue = -1;
        for(int i = 0; i < banks.size(); i++) {
            if (banks.get(i) > maxBankValue) {
                maxBank = i;
                maxBankValue = banks.get(i);
            }
        }

        // Redistribute
        List<Integer> mutableBank = Lists.newArrayList(banks);
        mutableBank.set(maxBank, 0);
        for (int i = maxBank + 1; i <= maxBank + maxBankValue; i++) {
            int idx = i % mutableBank.size();
            mutableBank.set(idx, mutableBank.get(idx) + 1);
        }

        return ImmutableList.copyOf(mutableBank);
    }

    @Value
    static class Result {
        int totalSteps;
        int loopLength;
    }
}

package aoc2017;

import aoc.Util;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.ImmutableMap;
import lombok.Value;

import java.util.*;

public class Day13 {
    public static void main(String[] args) throws Exception {
        List<String> input = Util.input(2017, 13);

        List<String> testInput = ImmutableList.of(
                "0: 3",
                "1: 2",
                "4: 4",
                "6: 4");

        Util.assertThat(Firewall.parse(testInput).simulate()).isEqualTo(new Result(true, 24));
        System.out.println("Part 1: " + Firewall.parse(input).simulate());

        Firewall firewall = Firewall.parse(input);
        Util.timeIt(() -> {
            for (int i = 0; ; i++) {
                if (!firewall.simulate(i, true).isCaught()) {
                    System.out.println("Part 2: " + i);
                    break;
                }
            }

            return null;
        });
    }

    static class Firewall {
        private final ImmutableMap<Integer, Integer> depthToRange;
        private final int maxDepth;

        public Firewall(Map<Integer, Integer> depthToRange) {
            this.depthToRange = ImmutableMap.copyOf(depthToRange);
            this.maxDepth = Collections.max(depthToRange.keySet());
        }

        public Result simulate() {
            return simulate(0, false);
        }

        public Result simulate(int delay, boolean failFast) {
            boolean caught = false;
            int cumulativeSeverity = 0;
            for (int i = delay; i <= maxDepth+delay; i++) {
                int position = i - delay;

                if (position >= 0 && position <= maxDepth) {
                    Optional<Integer> layerSeverity = calculateSeverity(delay, position);
                    if (layerSeverity.isPresent()) {
                        caught |= true;
                        cumulativeSeverity += layerSeverity.get();

                        if (failFast) {
                            return new Result(true, cumulativeSeverity);
                        }
                    }
                } else {
                    break;
                }
            }

            return new Result(caught, cumulativeSeverity);
        }

        private Optional<Integer> calculateSeverity(int delay, int position) {
            if(!depthToRange.containsKey(position)) {
                return Optional.empty();
            }

            int range = depthToRange.get(position);
            if ((position+delay) % ((range - 1)*2) == 0) {
                return Optional.of(range*position);
            } else {
                return Optional.empty();
            }
        }

        static Firewall parse(List<String> layers) {
            Map<Integer, Integer> depthToRange = parseMap(layers);

            return new Firewall(depthToRange);
        }

        static Map<Integer, Integer> parseMap(List<String> layers) {
            Map<Integer, Integer> depthToRange = new HashMap<>();
            for (String layer : layers) {
                String[] parts = layer.split(": ");

                depthToRange.put(
                        Integer.parseInt(parts[0]),
                        Integer.parseInt(parts[1])
                );
            }
            return depthToRange;
        }
    }

    @Value
    static class Result {
        boolean caught;
        int severity;
    }
}

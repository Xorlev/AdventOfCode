package aoc2017;

import aoc.Util;
import com.google.common.base.Joiner;
import com.google.common.base.Splitter;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.Iterables;
import com.google.common.collect.Lists;
import lombok.Value;

import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

public class Day21 {
    private static ImmutableList<String> EXAMPLE_PATTERNS = ImmutableList.of(
            "../.# => ##./#../...",
            ".#./..#/### => #..#/..../..../#..#"
    );

    static Pattern enhance(List<EnhancementRule> rules, List<Pattern> pattern) {
        for (Pattern rotation : pattern) {
            for (EnhancementRule rule : rules) {
                Optional<Pattern> newPattern = rule.enhance(rotation);
                if (newPattern.isPresent()) {
                    return newPattern.get();
                }
            }
        }

        throw new IllegalArgumentException("No enhancement pattern.");
    }

    public static void main(String[] args) {
        ImmutableList<EnhancementRule> enhancementPatterns = Util.input(2017, 21)
                .stream()
                .map(p -> {
                    String[] parts = p.split(" => ");
                    return new EnhancementRule(
                            new Pattern(parts[0]),
                            new Pattern(parts[1])
                    );
                })
                .collect(ImmutableList.toImmutableList());

//        Util.assertThat(pattern.divide()).isEqualTo(
//                ImmutableList.of(
//                        new Pattern("#./.."),
//                        new Pattern("../#."),
//                        new Pattern(".#/.."),
//                        new Pattern("../.#")
//                )
//        );
//        Util.assertThat(new Pattern("#./..").rotations()).isEqualTo(
//                ImmutableList.of(
//                        new Pattern("#./.."),
//                        new Pattern("../.#"),
//                        new Pattern("../#."),
//                        new Pattern(".#/..")
//                )
//        );

        Pattern start = new Pattern(".#./..#/###");
        List<Pattern> grid = Lists.newArrayList(start);
        for(int i = 0; i < 5; i++) {
            grid = grid.stream()
                    .flatMap(p -> p.divide()
                            .stream()
                            .map(Pattern::rotations)
                            .map(p2 -> enhance(enhancementPatterns, p2)))
                    .collect(Collectors.toList());
        }

        System.out.println(grid.stream().mapToInt(Pattern::pixelsEnabled).sum());
        System.out.println(grid);

        // Size of pattern
        // Flip Rotate pattern (size 2, size 3) (Pattern -> List<Pattern>)
        // Slice into sub-patterns of size{2,3} (Pattern -> List<Pattern>)
        // Stitch patterns back together
    }

    @Value
    static class Pattern {
        String pattern;

        List<Pattern> rotations() {
            if (size() == 2) {
                return ImmutableList.of(
                    this,
                    new Pattern(
                            String.valueOf(
                                    pattern.charAt(1)) + pattern.charAt(3) +
                                    '/' +
                                    pattern.charAt(4) + pattern.charAt(0)
                    ),
                    new Pattern(
                            String.valueOf(
                                    pattern.charAt(3)) + pattern.charAt(4) +
                                    '/' +
                                    pattern.charAt(0) + pattern.charAt(1)
                    ),
                    new Pattern(
                            String.valueOf(
                                    pattern.charAt(4)) + pattern.charAt(0) +
                                    '/' +
                                    pattern.charAt(1) + pattern.charAt(3)
                    )
                );
            } else if (size() == 3) {
                List<Pattern> patterns = Lists.newArrayList(this);
                for (int i = 0; i < 7; i++) {
                    Pattern last = Iterables.getLast(patterns);
                    List<List<String>> grid = last.toGrid();
                    StringBuilder sb = new StringBuilder();

                    // # # .  (0,0) (1,0) (2,0)
                    // # . .  (0,1) (1,1) (2,1)
                    // . . .  (0,2) (1,2) (2,2)
                    sb.append(get(grid, 1,0)).append(get(grid, 2,0)).append(get(grid, 2,1));
                    sb.append('/');
                    sb.append(get(grid, 0,0)).append(get(grid, 1,1)).append(get(grid, 2,2));
                    sb.append('/');
                    sb.append(get(grid, 0,1)).append(get(grid, 0,2)).append(get(grid, 1,2));
                    patterns.add(new Pattern(sb.toString()));
                }

                return patterns;
            } else {
                throw new IllegalStateException("Size > 3: " + pattern);
            }
        }

        static String get(List<List<String>> grid, int x, int y) {
            return grid.get(y).get(x);
        }

        List<Pattern> divide() {
            if(size() % 2 == 0 || size() % 3 == 0) {
                int stride = (size() % 3 == 0) ? 3 : 2;
                int steps = size() / stride;

                // TODO stop needing a real grid. :)
                List<List<String>> grid = toGrid();

                // Produce values at [0, stride), [stride, 2*stride), ...

                // ##.|##.|##.   (sx*step+x, sy*step+y)
                // #..|#..|#..
                // ...|...|...
                // ---+---+---
                // ##.|##.|##.
                // #..|#..|#..
                // ...|...|...
                // ---+---+---
                // ##.|##.|##.
                // #..|#..|#..
                // ...|...|...

                List<Pattern> patterns = new ArrayList<>();
                for (int sx = 0; sx < steps; sx++) {
                    for (int sy = 0; sy < steps; sy++) {
                        List<String> slice = new ArrayList<>();
                        for (int y = 0; y < stride; y++) {
                            for (int x = 0; x < stride; x++) {
                                int gy = sy * steps + y;
                                int gx = sx * steps + x;
                                String charAt = grid.get(gy).get(gx);
                                slice.add(charAt);
                            }
                            slice.add("/");
                        }
                        patterns.add(new Pattern(Joiner.on("").join(slice.subList(0, slice.size()-1))));
                    }
                }

                return patterns;
            } else {
                throw new IllegalStateException("Pattern size must be divisible by 2 or 3.");
            }
        }

        private List<List<String>> toGrid() {
            return Splitter.on('/')
                    .splitToList(pattern)
                    .stream()
                    .map(row -> Splitter.fixedLength(1).splitToList(row))
                    .collect(Collectors.toList());
        }

        int pixelsEnabled() {
            return pattern
                    .chars()
                    .map(c -> (c == '#') ? 1 : 0)
                    .sum();
        }

        int size() {
            return pattern.indexOf('/');
        }
    }

    @Value
    static class EnhancementRule {
        Pattern input;
        Pattern output;

        Optional<Pattern> enhance(Pattern subject) {
            if (input.equals(subject)) {
                return Optional.of(output);
            }

            return Optional.empty();
        }
    }
}

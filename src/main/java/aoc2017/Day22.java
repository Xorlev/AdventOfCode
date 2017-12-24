package aoc2017;

import aoc.Point;
import aoc.Util;
import com.google.common.collect.ImmutableList;
import lombok.Value;

import java.util.*;

import static aoc2017.Day19.Direction.*;

public class Day22 {
    public static void main(String[] args) {
        ImmutableList<String> example = ImmutableList.of(
                "..#",
                "#..",
                "..."
        );

        Util.assertThat(solvePartOne(parseMap(example))).isEqualTo(5587);


        System.out.println("Part 1: " + solvePartOne(parseMap(Util.input(2017, 22))));
        Util.assertThat(solvePartTwo(parseMap(example))).isEqualTo(2511944);
        System.out.println("Part 2: " + solvePartTwo(parseMap(Util.input(2017, 22))));
    }

    private static int solvePartOne(Input input) {
        Map<Point, Status> infected = input.infected;
        Point point = new Point(input.width/2, input.width/2);
        ImmutableList<Day19.Direction> directions = ImmutableList.of(
                UP, RIGHT, DOWN, LEFT
        );
        int direction = 0;

        int infections = 0;
        for(int i = 0; i < 10000; i++) {
            if (statusAt(infected, point) == Status.INFECTED) {
                // Right, clean
                infected.remove(point);
                direction++;
                direction %= directions.size();
            } else {
                // Left, infect
                infected.put(point, Status.INFECTED);
                infections++;
                direction--;
                if (direction < 0) {
                    direction = directions.size() - 1;
                }
            }

            // Move
            point = directions.get(direction).apply(point);
        }
        return infections;
    }

    private static int solvePartTwo(Input input) {
        Map<Point, Status> infected = input.infected;
        Point point = new Point(input.width/2, input.width/2);
        ImmutableList<Day19.Direction> directions = ImmutableList.of(
                UP, RIGHT, DOWN, LEFT
        );
        int direction = 0;

        int infections = 0;
        for(int i = 0; i < 10_000_000; i++) {
            // Infected, weaken and turn right
            if (statusAt(infected, point) == Status.INFECTED) {
                infected.put(point, Status.FLAGGED);
                direction++;
                direction %= directions.size();
            // Weakened, infect and continue moving
            } else if (statusAt(infected, point) == Status.WEAKENED) {
                infected.put(point, Status.INFECTED);
                infections++;
            // Flagged, clean and reverse direction
            } else if (statusAt(infected, point) == Status.FLAGGED) {
                infected.put(point, Status.CLEAN);
                direction += 2;
                direction %= directions.size();
            // Clean, weaken and turn left
            } else if (statusAt(infected, point) == Status.CLEAN) {
                infected.put(point, Status.WEAKENED);
                direction--;
                if (direction < 0) {
                    direction = directions.size() - 1;
                }
            } else {
                throw new IllegalStateException("Eh?");
            }

            // Move
            point = directions.get(direction).apply(point);
        }
        return infections;
    }

    private static Status statusAt(Map<Point, Status> infected, Point point) {
        return infected.getOrDefault(point, Status.CLEAN);
    }

    static Input parseMap(List<String> input) {
        int width = input.get(0).length();

        Map<Point, Status> infected = new HashMap<>();
        for (int y = 0; y < width; y++) {
            for (int x = 0; x < width; x++) {
                char v = input.get(y).charAt(x);

                if (v == '#') {
                    infected.put(new Point(x, y), Status.INFECTED);
                }
            }
        }

        return new Input(width, infected);
    }

    @Value
    static class Input {
        int width;
        Map<Point, Status> infected;
    }

    enum Status {
        CLEAN,
        INFECTED,
        WEAKENED,
        FLAGGED
    }
}

package aoc2017;

import aoc.Point;
import aoc.Util;
import com.google.common.base.Joiner;
import com.google.common.base.Splitter;
import com.google.common.primitives.Chars;
import lombok.Value;

import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class Day19 {
    private static final String EXAMPLE_INPUT =
            "     |          \n" +
            "     |  +--+    \n" +
            "     A  |  C    \n" +
            " F---|----E|--+ \n" +
            "     |  |  |  D \n" +
            "     +B-+  +--+ \n";

    public static void main(String[] args) {
        Util.assertThat(followPath(parseGrid(EXAMPLE_INPUT))).isEqualTo(new Result("ABCDEF", 38));

        System.out.println(followPath(parseGrid(Util.input(2017, 19))));
    }

    static Result followPath(List<List<Character>> grid) {
        List<Character> path = new ArrayList<>();
        Point currentPoint = new Point(grid.get(0).indexOf('|'), 0);
        Direction direction = Direction.DOWN;
        int steps = 0;
        while(true) {
            steps++;
            Point nextPoint = direction.apply(currentPoint);
            char pointChar = atPoint(grid, nextPoint);

            if (pointChar == '+') {
                // find next direction
                for (Point point : nextPoint.neighbors4()) {
                    if (!point.equals(currentPoint) && inBounds(grid, point) && atPoint(grid, point) != ' ') {
                        direction = Direction.direction(nextPoint, point);
                        break;
                    }
                }
            } else if (pointChar >= 'A' && pointChar <= 'Z') {
                path.add(pointChar);
            } else if (pointChar == ' ') {
                break;
            }

            currentPoint = nextPoint;
        }

        return new Result(Joiner.on("").join(path), steps);
    }

    static boolean inBounds(List<List<Character>> grid, Point point) {
        return point.getX() >= 0
                && point.getY() >= 0
                && point.getX() < grid.get(point.getY()).size()
                && point.getY() < grid.size();
    }

    static char atPoint(List<List<Character>> grid, Point point) {
        return grid.get(point.getY()).get(point.getX());
    }

    static List<List<Character>> parseGrid(String input) {
        return parseGrid(Splitter.on("\n").splitToList(input));
    }

    static List<List<Character>> parseGrid(List<String> input) {
        return input
                .stream()
                .map(s -> Chars.asList(s.toCharArray()))
                .collect(Collectors.toList());
    }

    @Value
    static class Result {
        String path;
        int steps;
    }

    enum Direction {
        UP {
            @Override
            public Point apply(Point point) {
                return point.add(new Point(0, -1));
            }
        },
        DOWN {
            @Override
            public Point apply(Point point) {
                return point.add(new Point(0, 1));
            }
        },
        LEFT {
            @Override
            public Point apply(Point point) {
                return point.add(new Point(-1, 0));
            }
        },
        RIGHT {
            @Override
            public Point apply(Point point) {
                return point.add(new Point(1, 0));
            }
        };

        public abstract Point apply(Point point);

        public static Direction direction(Point source, Point destination) {
            if (destination.getX() > source.getX()) {
                return Direction.RIGHT;
            } else if (destination.getX() < source.getX()) {
                return Direction.LEFT;
            } else if (destination.getY() > source.getY()) {
                return Direction.DOWN;
            } else if (destination.getY() < source.getY()) {
                return Direction.UP;
            }

            throw new IllegalArgumentException("Source " + source + ", destination " + destination);
        }
    }
}

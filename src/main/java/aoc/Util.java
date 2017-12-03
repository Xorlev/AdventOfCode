package aoc;

import com.google.common.base.Stopwatch;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.Lists;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.ToString;
import lombok.Value;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.ArrayDeque;
import java.util.Collections;
import java.util.Deque;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.Queue;
import java.util.concurrent.Callable;
import java.util.concurrent.TimeUnit;
import java.util.function.Function;
import java.util.stream.Collectors;

/**
 * Utility functions
 *
 * @author Michael Rose (xorlev)
 */
public class Util {
    public static List<String> input(int year, int day) {
        BufferedReader reader = new BufferedReader(new InputStreamReader(
                Util.class.getResourceAsStream("/" + year + "/" + day + "/input.txt")));

        return reader.lines()
                .filter(l -> !l.trim().isEmpty())
                .collect(Collectors.toList());
    }

    private static final long DAY_MS = TimeUnit.DAYS.toMillis(1);
    private static final long HOUR_MS = TimeUnit.HOURS.toMillis(1);
    private static final long MINUTE_MS = TimeUnit.MINUTES.toMillis(1);
    private static final long SECOND_MS = TimeUnit.SECONDS.toMillis(1);

    public static String msToString(long ms) {
        long remainingMs = ms;

        long days = remainingMs / DAY_MS;
        if (days > 0) {
            remainingMs -= days * DAY_MS;
        }
        long hours = remainingMs / HOUR_MS;
        if (hours > 0) {
            remainingMs -= hours * HOUR_MS;
        }
        long minutes = remainingMs / MINUTE_MS;
        if (minutes > 0) {
            remainingMs -= minutes * MINUTE_MS;
        }
        long seconds = remainingMs / SECOND_MS;
        if (seconds > 0) {
            remainingMs -= seconds * SECOND_MS;
        }

        StringBuilder sb = new StringBuilder();
        if (days > 0)
            sb.append(days).append("d");
        if (hours > 0)
            sb.append(hours).append("h");
        if (minutes > 0)
            sb.append(minutes).append("m");
        if (seconds > 0)
            sb.append(seconds).append("s");
        if (remainingMs > 0)
            sb.append(remainingMs).append("ms");

        return sb.toString();
    }

    public static <T> T timeIt(Callable<T> callable) throws Exception {
        Stopwatch stopwatch = Stopwatch.createStarted();

        T result = callable.call();

        System.out.printf("%s - %s\n", callable.toString(), msToString(stopwatch.stop().elapsed(TimeUnit.MILLISECONDS)));

        return result;
    }

    @Value
    public static class Point {
        public static Point ZERO = new Point(0,0);

        int x, y;

        public boolean isNatural() {
            return x >= 0 && y >= 0;
        }

        public Point add(Point point) {
            return new Point(x + point.x, y + point.y);
        }

    }

    public static List<Point> neighbors4(Point p) {
        return Lists.newArrayList(
                new Point(p.x - 1, p.y),
                new Point(p.x + 1, p.y),
                new Point(p.x, p.y - 1),
                new Point(p.x, p.y + 1)
        );
    }

    public static List<Point> neighbors8(Point p) {
        return Lists.newArrayList(
                new Point(p.x - 1, p.y),
                new Point(p.x + 1, p.y),
                new Point(p.x, p.y - 1),
                new Point(p.x, p.y + 1),
                new Point(p.x - 1, p.y - 1),
                new Point(p.x + 1, p.y + 1),
                new Point(p.x + 1, p.y - 1),
                new Point(p.x - 1, p.y + 1)
        );
    }

    public static double manhattanDistance(Point o, Point t) {
        return Math.abs(o.x - t.x) + Math.abs(o.y - t.y);
    }

    public static double euclideanDistance(Point o, Point t) {
        return Math.hypot(o.x - t.x, o.y - t.y);
    }

    @ToString
    @Getter
    @EqualsAndHashCode
    public static class AStarResult<T> {
        private final boolean failed;
        private final ImmutableList<T> path;

        private AStarResult() {
            this.failed = true;
            this.path = ImmutableList.of();
        }

        public AStarResult(Iterable<T> path) {
            this.failed = false;
            this.path = ImmutableList.copyOf(path);
        }

        public static <T> AStarResult<T> failed() {
            return new AStarResult<>();
        }
    }

    @Value
    public static class ValueWithCost<T> implements Comparable<ValueWithCost<T>> {
        T value;
        double cost;

        @Override
        public int compareTo(ValueWithCost<T> o) {
            return Double.compare(cost, o.cost);
        }

    }

    public static <T> AStarResult<T> fromPath(T p, Map<T, T> parentGraph) {
        Deque<T> points = new ArrayDeque<>();
        points.push(p);

        while (p != null) {
            p = parentGraph.get(p);

            if (p != null) {
                points.push(p);
            }
        }

        // Points were reversed by using Deque as a stack
        return new AStarResult<>(points);
    }

    public static <T> AStarResult<T> astarSearch(T start, Function<T, Double> hFunc, Function<T, List<T>> moveFunc) {
        PriorityQueue<ValueWithCost<T>> frontier = new PriorityQueue<>(
                Collections.singleton(new ValueWithCost<>(start, hFunc.apply(start)))
        );
        Map<T, T> previous = new HashMap<>();
        Map<T, Double> pathCost = new HashMap<>();
        pathCost.put(start, 0.0);
        previous.put(start, null);

        while (!frontier.isEmpty()) {
            ValueWithCost<T> p = frontier.poll();

            if (Math.abs(0 - hFunc.apply(p.value)) < 1e-6) {
                return fromPath(p.value, previous);
            }

            double newCost = pathCost.get(p.value) + 1;
            for (T newPoint : moveFunc.apply(p.value)) {
                Double cost = pathCost.get(newPoint);
                if (cost == null || newCost < cost) {
                    frontier.add(new ValueWithCost<>(newPoint, newCost + hFunc.apply(newPoint)));
                    pathCost.put(newPoint, newCost);
                    previous.put(newPoint, p.value);
                }
            }
        }

        return AStarResult.failed();
    }

    public static Map<Point, Double> bfsWithin(Point start, double maxCost, Function<Point, List<Point>> moveFn) {
        Queue<Point> frontier = new ArrayDeque<>();
        Map<Point, Double> pathCost = new HashMap<>();
        frontier.add(start);
        pathCost.put(start, 0.0);

        // for each neighbor
        // if we haven't seen it, add it to the frontier
        // if the cost is > steps, ignore it
        while (!frontier.isEmpty()) {
            Point point = frontier.poll();
            Double pointCost = pathCost.get(point);

            if (pointCost < maxCost) {
                for (Point newPoint : moveFn.apply(point)) {
                    if (!pathCost.containsKey(newPoint)) {
                        frontier.add(newPoint);
                        pathCost.put(newPoint, pointCost + 1);
                    }
                }
            }
        }

        return pathCost;
    }

    @Value
    public static class AssertChain<T> {
        private T expected;

        public void isEqualTo(T actual) {
            if (!expected.equals(actual)) {
                throw new AssertionException(
                        String.format("Assertion failed: expected [%s], actual [%s]", expected, actual));
            }
        }
    }

    public static <T> AssertChain<T> assertThat(T expected) {
        return new AssertChain<>(expected);
    }

    public static class AssertionException extends RuntimeException {
        public AssertionException(String message) {
            super(message);
        }
    }
}

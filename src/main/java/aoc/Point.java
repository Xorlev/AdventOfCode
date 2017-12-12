package aoc;

import com.google.common.collect.Lists;
import lombok.Value;

import java.util.List;

@Value
public class Point {
    public static Point ZERO = new Point(0,0);

    int x, y;

    public boolean isNatural() {
        return x >= 0 && y >= 0;
    }

    public Point add(Point point) {
        return new Point(x + point.x, y + point.y);
    }

    public List<Point> neighbors4() {
        return Lists.newArrayList(
                new Point(x - 1, y),
                new Point(x + 1, y),
                new Point(x, y - 1),
                new Point(x, y + 1)
        );
    }

    public List<Point> neighbors8() {
        return Lists.newArrayList(
                new Point(x - 1, y),
                new Point(x + 1, y),
                new Point(x, y - 1),
                new Point(x, y + 1),
                new Point(x - 1, y - 1),
                new Point(x + 1, y + 1),
                new Point(x + 1, y - 1),
                new Point(x - 1, y + 1)
        );
    }

    public double manhattanDistance(Point t) {
        return Math.abs(this.x - t.x) + Math.abs(this.y - t.y);
    }

    public double euclideanDistance(Point t) {
        return Math.hypot(this.x - t.x, this.y - t.y);
    }
}

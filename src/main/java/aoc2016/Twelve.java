package aoc2016;

import aoc.Point;
import aoc.Util;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

/**
 * 2016-12-27
 *
 * @author Michael Rose (xorlev)
 */
public class Twelve {

    static boolean isOpen(Point point, int favoriteNumber) {
        int x = point.getX();
        int y = point.getY();
        return Integer.bitCount(x*x + 3*x + 2*x*y + y + y*y + favoriteNumber) % 2 == 0;
    }

    static void printBoard(int favoriteNumber) {
        for(int y = 0; y < 60; y++) {
            for(int x = 0; x < 30; x++) {
                Point p = new Point(x, y);

                System.out.print((isOpen(p, favoriteNumber) ? "." : "#"));
            }
            System.out.println();
        }

    }

    static List<Point> neighbors(Point point, int favoriteNumber) {
        return point.neighbors4()
                   .stream()
                   .filter(p -> p.isNatural() && isOpen(p, favoriteNumber))
                   .collect(Collectors.toList());
    }

    public static void main(String[] args) {
        int favoriteNumber = 1364;

        printBoard(favoriteNumber);

        Point start = new Point(1, 1);
        Point end = new Point(31, 39);
        Util.AStarResult result = Util.astarSearch(start, p -> p.manhattanDistance(end), p -> neighbors(p, favoriteNumber));
        Map<Point, Double> bfsWithin = Util.bfsWithin(start, 50, p -> neighbors(p, favoriteNumber));

        System.out.println(result);
        System.out.println(result.getPath().size() - 1);
        System.out.println(bfsWithin);
        System.out.println(bfsWithin.size());
    }
}

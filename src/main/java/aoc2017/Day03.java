package aoc2017;

import aoc.Point;
import aoc.Util;

import java.util.HashMap;
import java.util.Map;

/**
 * Spiral memory grid
 *
 * You come across an experimental new kind of memory stored on an infinite two-dimensional grid.

 Each square on the grid is allocated in a spiral pattern starting at a location marked 1 and then counting up while
 spiraling outward. For example, the first few squares are allocated like this:

 17  16  15  14  13
 18   5   4   3  12
 19   6   1   2  11
 20   7   8   9  10
 21  22  23---> ...

 While this is very space-efficient (no squares are skipped), requested data must be carried back to square 1 (the
 location of the only access port for this memory system) by programs that can only move up, down, left, or right. They
 always take the shortest path: the Manhattan Distance between the location of the data and square 1.

 For example:

 Data from square 1 is carried 0 steps, since it's at the access port.
 Data from square 12 is carried 3 steps, such as: down, left, left.
 Data from square 23 is carried only 2 steps: up twice.
 Data from square 1024 must be carried 31 steps.
 */
public class Day03 {
    public static void main(String[] args) {
        int input = 265149;

        Util.assertThat(solvePartOne(2)).isEqualTo(1);
        Util.assertThat(solvePartOne(1024)).isEqualTo(31);
        Util.assertThat(solvePartOne(23)).isEqualTo(2);
        Util.assertThat(solvePartOne(24)).isEqualTo(3);
        Util.assertThat(solvePartOne(12)).isEqualTo(3);


        System.out.println("Part 1: " + solvePartOne(input));
        System.out.println("Part 2: " + solvePartTwo(input));


    }

    static int solvePartOne(int input) {
        Point pos = getPos(input);

        return (int) pos.manhattanDistance(Point.ZERO);
    }

    /**
     * As a stress test on the system, the programs here clear the grid and then store the value 1 in square 1. Then, in
     * the same allocation order as shown above, they store the sum of the values in all adjacent squares, including
     * diagonals.

     So, the first few squares' values are chosen as follows:

     Square 1 starts with the value 1.
     Square 2 has only one adjacent filled square (with value 1), so it also stores 1.
     Square 3 has both of the above squares as neighbors and stores the sum of their values, 2.
     Square 4 has all three of the aforementioned squares as neighbors and stores the sum of their values, 4.
     Square 5 only has the first and fourth squares as neighbors, so it gets the value 5.
     */
    static int solvePartTwo(int input) {
        Map<Point, Integer> value = new HashMap<>();
        value.put(new Point(0, 0), 1);

        for (int i = 2; i < input; i++) {
            int sum = 0;
            for (Point point : getPos(i).neighbors8()) {
                sum += value.getOrDefault(point, 0);
            }

            value.put(getPos(i), sum);

            if (sum > input) {
                return sum;
            }
        }

        return -1;
    }

    private static Point getPos(int n) {
        if (n == 1) {
            return new Point(0,0 );
        }

        int sideLength = (int) Math.ceil(Math.sqrt(n));
        if (sideLength % 2 == 0) {
            // Side length increases by at least two each step.
            sideLength += 1;
        }


        int x = sideLength / 2;
        int y = sideLength / 2;

        // Number of steps from middle to edge.
        int radius = sideLength - 1;

        // 'Bottom-right' corner, (radius, radius)
        int bottomRight = sideLength*sideLength;
        int bottomLeft = bottomRight - radius;
        int topLeft = bottomRight - 2*radius;
        int topRight = bottomRight - 3*radius;


        if (n <= topRight) {
            y = topRight - n - sideLength/2;
        } else if (n <= topLeft) {
            x = topLeft - n - sideLength/2;
            y *= -1;
        } else if (n <= bottomLeft) {
            x *= -1;
            y = n - bottomLeft + sideLength/2;
        } else if (n <= bottomRight) {
            x = n - bottomRight + sideLength/2;
        }

        return new Point(x, y);
    }
}

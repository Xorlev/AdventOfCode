package aoc2017;

import aoc.HexPoint;
import aoc.Util;
import com.google.common.base.Splitter;
import com.google.common.collect.Iterables;

import java.util.ArrayList;
import java.util.List;

public class Day11 {
    public static void main(String[] args) {
        Util.assertThat(minSteps(parseSteps("ne,ne,ne"))).isEqualTo(3);
        Util.assertThat(minSteps(parseSteps("ne,ne,sw,sw"))).isEqualTo(0);
        Util.assertThat(minSteps(parseSteps("ne,ne,s,s"))).isEqualTo(2);
        Util.assertThat(minSteps(parseSteps("se,sw,se,sw,sw"))).isEqualTo(3);

        String inputSteps = Iterables.getOnlyElement(Util.input(2017, 11));

        List<HexPoint> parsedSteps = parseSteps(inputSteps);
        System.out.println("Part 1: " + minSteps(parsedSteps));
        System.out.println("Part 2: " + maxSteps(parsedSteps));
    }

    private static int minSteps(List<HexPoint> hexPoints) {
        HexPoint goal = Iterables.getLast(hexPoints);

        return (int) goal.manhattanDistance(HexPoint.ZERO);
    }

    private static int maxSteps(List<HexPoint> hexPoints) {
        return hexPoints
                .stream()
                .mapToInt(point -> (int )point.manhattanDistance(HexPoint.ZERO))
                .max()
                .getAsInt();
    }

    static List<HexPoint> parseSteps(String steps) {
        List<HexPoint> points = new ArrayList<>();
        HexPoint position = HexPoint.ZERO;
        points.add(position);

        for(String step : Splitter.on(',').trimResults().split(steps)) {
            position = position.add(parseToDirection(step));
            points.add(position);
        }

        return points;
    }

    private static HexPoint parseToDirection(String step) {
        switch (step) {
            case "n":
                return HexPoint.N;
            case "s":
                return HexPoint.S;
            case "ne":
                return HexPoint.NE;
            case "nw":
                return HexPoint.NW;
            case "se":
                return HexPoint.SE;
            case "sw":
                return HexPoint.SW;
            default:
                throw new IllegalArgumentException("Unsupported direction: " + step);
        }
    }
}

package aoc2017;

import aoc.Util;
import com.google.common.base.Joiner;
import com.google.common.base.Splitter;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.Iterables;
import com.google.common.collect.Lists;
import lombok.Value;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Day16 {
    public static void main(String[] args) {
        Util.assertThat(applyMoves(genPrograms(5), parseMoves("s1,x3/4,pe/b"), 1))
                .isEqualTo(ImmutableList.of('b', 'a', 'e', 'd', 'c'));

        String input = Util.inputLine(2017, 16);
        List<Character> programs = applyMoves(genPrograms(16), parseMoves(input), 1);
        System.out.println("Part 1: " + Joiner.on("").join(programs));

        // Part 2: Run it until the dance repeats, don't run it 1e9 times.
        Util.assertThat(applyMoves(genPrograms(5), parseMoves("s1,x3/4,pe/b"), 1_000_000_000))
                .isEqualTo(ImmutableList.of('a', 'b', 'c', 'd', 'e'));

        List<Character> programs2 = applyMoves(genPrograms(16), parseMoves(input), 1_000_000_000);
        System.out.println("Part 2: " + Joiner.on("").join(programs2));
    }

    static List<Character> applyMoves(List<Character> programs, List<Move> moves, int times) {
        ImmutableList<Character> initialState = ImmutableList.copyOf(programs);
        ImmutableList<Character> lastState = ImmutableList.copyOf(programs);

        int cycleLength = 0;
        for (int i = 0; i < times; i++) {
            moves.forEach(move -> move.apply(programs));
            lastState = ImmutableList.copyOf(programs);

            if (lastState.equals(initialState)) {
                // Found cycle in states.
                cycleLength = i+1;
                break;
            }
        }

        if (cycleLength > 0) {
            int remainingMoves = times % cycleLength;

            List<Character> state = Lists.newArrayList(initialState);
            for (int i = 0; i < remainingMoves; i++) {
                moves.forEach(move -> move.apply(state));
            }

            return state;
        }

        return programs;
    }

    static List<Character> genPrograms(int size) {
        List<Character> programs = new ArrayList<>();
        for (int i = 0; i < size; i++) {
            programs.add((char) ((int) 'a' + i));
        }

        return programs;
    }

    static ImmutableList<Move> parseMoves(String moves) {
        ImmutableList.Builder<Move> moveBuilder = ImmutableList.builder();
        for (String moveStr : Splitter.on(',').split(moves)) {
            Move move;
            switch (moveStr.charAt(0)) {
                case 's':
                    move = new Spin(Integer.parseInt(moveStr.substring(1)));
                    break;
                case 'x':
                    String[] parts = moveStr.substring(1).split("/");
                    move = new Exchange(Integer.parseInt(parts[0]), Integer.parseInt(parts[1]));
                    break;
                case 'p':
                    parts = moveStr.substring(1).split("/");
                    move = new Partner(parts[0].charAt(0), parts[1].charAt(0));
                    break;
                default:
                    throw new IllegalArgumentException("Unknown move type: " + moveStr);
            }
            moveBuilder.add(move);
        }

        return moveBuilder.build();
    }

    abstract static class Move {
        abstract void apply(List<Character> programs);
    }

    @Value
    static class Spin extends Move {
        int distance;

        @Override
        void apply(List<Character> programs) {
            Collections.rotate(programs, distance);
        }
    }

    @Value
    static class Exchange extends Move {
        int a;
        int b;

        @Override
        void apply(List<Character> programs) {
            Character temp = programs.get(a);
            programs.set(a, programs.get(b));
            programs.set(b, temp);
        }
    }

    @Value
    static class Partner extends Move {
        Character a;
        Character b;

        @Override
        void apply(List<Character> programs) {
            int indexOfA = programs.indexOf(a);
            int indexOfB = programs.indexOf(b);

            new Exchange(indexOfA, indexOfB).apply(programs);
        }
    }
}

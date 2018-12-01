package aoc2017;

import aoc.Util;
import com.google.common.collect.ImmutableList;
import lombok.Builder;
import lombok.Value;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.function.BiFunction;
import java.util.function.Function;

import static aoc2017.Day18.getAndCombine;
import static aoc2017.Day18.lvalue;
import static aoc2017.Day18.rvalue;

public class Day23 {
    public static void main(String[] args) {
        System.out.println("Part 1: " + solvePartOne(Util.input(2017, 23)));
        System.out.println("Part 2: " + solvePartTwo());
    }

    private static long solvePartOne(List<String> input) {
        ImmutableList<Day18.Operation> operations = input
                .stream()
                .map(Day18.Operation::parse)
                .collect(ImmutableList.toImmutableList());

        Map<String, Long> registers = new HashMap<>();
        long mulInvokations = 0;
        int pc = 0;
        while(true) {
            if (pc < 0 || pc >= operations.size()) {
                break;
            }

            Day18.Operation operation = operations.get(pc);
            switch(operation.getOpType()) {
                case SET:
                    getAndCombine(registers, operation, (l,r) -> r);
                    break;
                case SUB:
                    getAndCombine(registers, operation, (l,r) -> l-r);
                    break;
                case MUL:
                    mulInvokations++;
                    getAndCombine(registers, operation, (l,r) -> l*r);
                    break;
                case JNZ:
                    if (lvalue(registers, operation) != 0L) {
                        pc += rvalue(registers, operation);
                        continue;
                    }
                    break;
                default:
                    throw new IllegalArgumentException("Unsupported operation: " + operation);
            }
            pc++;
        }

        return mulInvokations;
    }

    private static long solvePartTwo() {
        int b = 99 * 100 + 100000;

        int composites = 0;
        for (int n = b - 17000; n <= b ; n += 17) {
            for (int i = 2; i < Math.sqrt(n); i++) {
                if (n % i == 0) {
                    composites++;
                    break;
                }
            }
        }

        return composites;
    }
}

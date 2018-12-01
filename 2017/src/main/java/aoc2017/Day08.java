package aoc2017;

import aoc.Util;
import com.google.common.base.Splitter;
import com.google.common.collect.ImmutableList;
import lombok.Builder;
import lombok.Value;

import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day08 {
    private static final Pattern INSTRUCTION_PATTERN =
            Pattern.compile("([a-z]+) ([^\\s]+) (-?[0-9]+) if ([a-z]+) ([><=!]+) (-?[0-9]+)");

    public static void main(String[] args) {
        List<String> exampleInput = Splitter.on("\n").splitToList("" +
                "b inc 5 if a > 1\n" +
                "a inc 1 if b < 5\n" +
                "c dec -10 if a >= 1\n" +
                "c inc -20 if c == 10");

        ImmutableList<Instruction> exampleInstructions = parse(exampleInput);

        Util.assertThat(Collections.max(interpreter(exampleInstructions).registers.values())).isEqualTo(1);
        Util.assertThat(interpreter(exampleInstructions).maxValue).isEqualTo(10);


        ImmutableList<Instruction> instructions = parse(Util.input(2017, 8));
        Result result = interpreter(instructions);
        System.out.println("Part 1: " + Collections.max(result.registers.values()));
        System.out.println("Part 2: " + result.maxValue);
    }

    private static Result interpreter(ImmutableList<Instruction> instructions) {
        Map<String, Integer> registers = new HashMap<>();
        int max = 0;
        for (Instruction instruction : instructions) {
            int conditionValue = registers.getOrDefault(instruction.comparisonRegister, 0);

            if (instruction.comparator.apply(conditionValue, instruction.conditionalValue)) {
                int value = 0;
                switch (instruction.operation) {
                    case INC:
                        value = registers.getOrDefault(instruction.register, 0) + instruction.operand;
                        break;
                    case DEC:
                        value = registers.getOrDefault(instruction.register, 0) - instruction.operand;
                        break;
                }

                registers.put(instruction.register, value);
                if (max < value) {
                    max = value;
                }
            }
        }
        return new Result(registers, max);
    }

    private static ImmutableList<Instruction> parse(List<String> exampleInput) {
        return exampleInput
                .stream()
                .map(Instruction::parse)
                .collect(ImmutableList.toImmutableList());
    }

    @Value
    @Builder
    static class Instruction {
        String register;
        Op operation;
        int operand;

        String comparisonRegister;
        ComparatorType comparator;
        int conditionalValue;


        static Instruction parse(String line) {
            Matcher matcher = INSTRUCTION_PATTERN.matcher(line);
            if (matcher.find()) {
                String register = matcher.group(1);
                Op operation = Op.parse(matcher.group(2));
                int operand = Integer.parseInt(matcher.group(3));
                String comparisonRegister = matcher.group(4);
                ComparatorType comparatorType = ComparatorType.parse(matcher.group(5));
                int conditionalValue = Integer.parseInt(matcher.group(6));

                return Instruction.builder()
                        .register(register)
                        .operation(operation)
                        .operand(operand)
                        .comparisonRegister(comparisonRegister)
                        .comparator(comparatorType)
                        .conditionalValue(conditionalValue)
                        .build();
            } else {
                throw new IllegalArgumentException("Bad line: " + line);
            }
        }
    }

    @Value
    static class Result {
        Map<String, Integer> registers;
        int maxValue;
    }

    enum ComparatorType {
        GT {
            @Override
            public boolean apply(int left, int right) {
                return left > right;
            }
        },
        LT {
            @Override
            public boolean apply(int left, int right) {
                return left < right;
            }
        },
        GTE {
            @Override
            public boolean apply(int left, int right) {
                return left >= right;
            }
        },
        LTE {
            @Override
            public boolean apply(int left, int right) {
                return left <= right;
            }
        },
        EQ {
            @Override
            public boolean apply(int left, int right) {
                return left == right;
            }
        },
        NEQ {
            @Override
            public boolean apply(int left, int right) {
                return left != right;
            }
        };

        public abstract boolean apply(int left, int right);

        static ComparatorType parse(String token) {
            switch (token) {
                case ">": return GT;
                case "<": return LT;
                case ">=": return GTE;
                case "<=": return LTE;
                case "==": return EQ;
                case "!=": return NEQ;
                default: throw new IllegalArgumentException("Cannot parse token: " + token);
            }
        }
    }

    enum Op {
        INC,
        DEC;

        static Op parse(String token) {
            switch (token) {
                case "inc": return Op.INC;
                case "dec": return Op.DEC;
                default: throw new IllegalArgumentException("Cannot parse op: " + token);
            }
        }
    }
}

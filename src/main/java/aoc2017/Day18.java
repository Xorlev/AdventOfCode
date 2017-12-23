package aoc2017;

import aoc.Util;
import com.google.common.collect.ImmutableList;
import lombok.Builder;
import lombok.Value;

import java.util.*;
import java.util.function.BiFunction;
import java.util.function.Function;

public class Day18 {
    private static final ImmutableList<String> EXAMPLE_INPUT =
            ImmutableList.of("set a 1",
            "add a 2",
            "mul a a",
            "mod a 5",
            "snd a",
            "set a 0",
            "rcv a",
            "jgz a -1",
            "set a 1",
            "jgz a -2");
    private static final ImmutableList<String> EXAMPLE_INPUT2 =
            ImmutableList.of("snd 1",
                    "snd 2",
                    "snd p",
                    "rcv a",
                    "rcv b",
                    "rcv c",
                    "rcv d");

    public static void main(String[] args) {
        Util.assertThat(solvePartOne(EXAMPLE_INPUT)).isEqualTo(4L);
        System.out.println("Part 1: " + solvePartOne(Util.input(2017, 18)));
        Util.assertThat(solvePartTwo(EXAMPLE_INPUT2)).isEqualTo(3);
        System.out.println("Part 2: " + solvePartTwo(Util.input(2017, 18)));
    }

    private static long solvePartOne(List<String> input) {
        ImmutableList<Operation> operations = input
                .stream()
                .map(Operation::parse)
                .collect(ImmutableList.toImmutableList());

        Map<String, Long> registers = new HashMap<>();
        long lastFreq = 0;
        for (int pc = 0; pc < operations.size();) {
            if (pc < 0) {
                break;
            }

            Operation operation = operations.get(pc);
            switch(operation.opType) {
                case SET:
                    getAndCombine(registers, operation, (l,r) -> r);
                    break;
                case ADD:
                    getAndCombine(registers, operation, (l,r) -> l+r);
                    break;
                case MUL:
                    getAndCombine(registers, operation, (l,r) -> l*r);
                    break;
                case MOD:
                    getAndCombine(registers, operation, (l,r) -> l%r);
                    break;
                case SND:
                    lastFreq = lvalue(registers, operation);
                    break;
                case RCV:
                    if (registers.getOrDefault(operation.left, 0L) != 0) {
                        return lastFreq;
                    }
                    break;
                case JGZ:
                    if (registers.getOrDefault(operation.left, 0L) > 0) {
                        pc += Long.parseLong(operation.right);
                        continue;
                    }
                    break;
            }
            pc++;
        }

        return -1;
    }

    private static int solvePartTwo(List<String> input) {
        ImmutableList<Operation> operations = input
                .stream()
                .map(Operation::parse)
                .collect(ImmutableList.toImmutableList());

        Deque<Long> program0Channel = new ArrayDeque<>();
        Deque<Long> program1Channel = new ArrayDeque<>();

        Task program0 = new Task(0, program1Channel, program0Channel, operations);
        Task program1 = new Task(1, program0Channel, program1Channel, operations);
        ImmutableList<Task> tasks = ImmutableList.of(program0, program1);


        while(true) {
            int ready = 0;
            for (Task task : tasks) {
                if(task.execute() == Task.Status.READY) {
                    ready++;
                }
            }

            // Detect deadlock.
            if (ready == 0) {
                return program1.sent;
            }
        }
    }

    private static void getAndCombine(
            Map<String, Long> registers, Operation operation, BiFunction<Long, Long, Long> combineFn) {
        long lvalue = lvalue(registers, operation);
        long rvalue = rvalue(registers, operation);

        registers.put(operation.left, combineFn.apply(lvalue, rvalue));
    }

    private static long lvalue(Map<String, Long> registers, Operation operation) {
        return value(registers, operation, Operation::getLeft);
    }

    private static long rvalue(Map<String, Long> registers, Operation operation) {
        return value(registers, operation, Operation::getRight);
    }

    private static long value(Map<String, Long> registers, Operation operation, Function<Operation, String> getter) {
        String v = getter.apply(operation);

        long value;
        if (valueIsRegister(v)) {
            value = registers.getOrDefault(v, 0L);
        } else {
            value = Long.parseLong(v);
        }
        return value;
    }

    private static boolean valueIsRegister(String value) {
        return value.charAt(0) >= 'a' && value.charAt(0) <= 'z';
    }

    static class Task {
        private final Deque<Long> inputChannel;
        private final Deque<Long> outputChannel;
        private final ImmutableList<Operation> operations;
        private final Map<String, Long> registers = new HashMap<>();

        private int pc = 0;
        private int sent = 0;
        private boolean finished = false;

        Task(int programId, Deque<Long> inputChannel, Deque<Long> outputChannel, ImmutableList<Operation> operations) {
            this.inputChannel = inputChannel;
            this.outputChannel = outputChannel;
            this.operations = operations;

            this.registers.put("p", (long)programId);
        }

        public Status execute() {
            Operation operation = operations.get(pc);
            switch(operation.opType) {
                case SET:
                    getAndCombine(registers, operation, (l,r) -> r);
                    break;
                case ADD:
                    getAndCombine(registers, operation, (l,r) -> l+r);
                    break;
                case MUL:
                    getAndCombine(registers, operation, (l,r) -> l*r);
                    break;
                case MOD:
                    getAndCombine(registers, operation, (l,r) -> l%r);
                    break;
                case SND:
                    sent++;
                    outputChannel.add(lvalue(registers, operation));
                    break;
                case RCV:
                    if (inputChannel.isEmpty()) {
                        return Status.BLOCKED;
                    }

                    registers.put(operation.left, inputChannel.poll());
                    break;
                case JGZ:
                    if (lvalue(registers, operation) > 0L) {
                        pc += rvalue(registers, operation);
                        return Status.READY;
                    }
                    break;
                default:
                    throw new IllegalArgumentException("Invalid instruction: " + operation);
            }
            pc++;

            return Status.READY;
        }

        enum Status {
            READY,
            BLOCKED,
        }
    }

    @Value
    @Builder
    static class Operation {
        OpType opType;
        String left;
        String right;


        public static Operation parse(String opString) {
            String[] parts = opString.split(" ");
            OpType opType;
            String register = parts[1];
            String value = null;
            switch (parts[0]) {
                case "set":
                    opType = OpType.SET;
                    value = parts[2];
                    break;
                case "add":
                    opType = OpType.ADD;
                    value = parts[2];
                    break;
                case "mul":
                    opType = OpType.MUL;
                    value = parts[2];
                    break;
                case "mod":
                    opType = OpType.MOD;
                    value = parts[2];
                    break;
                case "snd":
                    opType = OpType.SND;
                    break;
                case "rcv":
                    opType = OpType.RCV;
                    break;
                case "jgz":
                    opType = OpType.JGZ;
                    value = parts[2];
                    break;
                default:
                    throw new IllegalArgumentException("Could not parse op: " + opString);
            }

            return new Operation(opType, register, value);
        }

        enum OpType {
            SET,
            ADD,
            MUL,
            MOD,
            SND,
            RCV,
            JGZ;
        }
    }
}

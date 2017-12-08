package aoc2017;

import aoc.Util;
import com.google.common.base.Splitter;
import com.google.common.collect.*;
import lombok.Value;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day07 {
    public static void main(String[] args) {
        List<String> testInput = Splitter.on("\n").splitToList("pbga (66)\n" +
                            "xhth (57)\n" +
                            "ebii (61)\n" +
                            "havc (66)\n" +
                            "ktlj (57)\n" +
                            "fwft (72) -> ktlj, cntj, xhth\n" +
                            "qoyq (66)\n" +
                            "padx (45) -> pbga, havc, qoyq\n" +
                            "tknk (41) -> ugml, padx, fwft\n" +
                            "jptl (61)\n" +
                            "ugml (68) -> gyxo, ebii, jptl\n" +
                            "gyxo (61)\n" +
                            "cntj (57)");

        ImmutableList<Program> testPrograms = parsePrograms(testInput);

        Util.assertThat(solvePartOne(testPrograms))
                .isEqualTo(new Program("tknk", 41, ImmutableSet.of("ugml", "padx", "fwft")));

        Util.assertThat(solvePartTwo(testPrograms))
                .isEqualTo(60);

        ImmutableList<Program> programs = parsePrograms(Util.input(2017, 7));
        System.out.println("Part 1: " + solvePartOne(programs));
        System.out.println("Part 2: " + solvePartTwo(programs));
    }

    private static Program solvePartOne(ImmutableList<Program> programs) {
        return findRoot(programs);
    }

    /**
     * In the example above, this means that for ugml's disc to be balanced, gyxo, ebii, and jptl must all have the
     * same weight, and they do: 61.

     However, for tknk to be balanced, each of the programs standing on its disc and all programs above it must each
     match. This means that the following sums must all be the same:

     ugml + (gyxo + ebii + jptl) = 68 + (61 + 61 + 61) = 251
     padx + (pbga + havc + qoyq) = 45 + (66 + 66 + 66) = 243
     fwft + (ktlj + cntj + xhth) = 72 + (57 + 57 + 57) = 243
     As you can see, tknk's disc is unbalanced: ugml's stack is heavier than the other two. Even though the nodes above
     ugml are balanced, ugml itself is too heavy: it needs to be 8 units lighter for its stack to weigh 243 and keep the
     towers balanced. If this change were made, its weight would be 60.
     */
    private static int solvePartTwo(ImmutableList<Program> programs) {
        ImmutableMultimap<String, String> graph = buildGraph(programs);

        // Mapping of node id to Program.
        ImmutableMap<String, Program> programMap = Maps.uniqueIndex(programs, Program::getName);

        // Find the root node (no parents).
        Program root = findRoot(programs);

        // DFS through the graph, writing weights as we pop back up the stack.
        ImmutableMap<String, Integer> cumulativeWeightsByProgram = calculateCumulativeWeights(programMap, graph, root);

        // Create a mapping of weight -> [program] from the cumulative weights.
        Multimap<Integer, String> weightToProgram =
                Multimaps.invertFrom(Multimaps.forMap(cumulativeWeightsByProgram), HashMultimap.create());

        // Traverse down. At each level, follow the imbalanced subtree. Keep following until all subtrees are balanced.
        String node = root.name;
        int weightCorrection = -1;
        while(true) {
            // Count weight frequencies.
            Multiset<Integer> weightToFrequency = HashMultiset.create();
            for (String child : graph.get(node)) {
                int weight = cumulativeWeightsByProgram.get(child);
                weightToFrequency.add(weight);
            }

            if (weightToFrequency.entrySet().size() <= 1) {
                // Things are balanced below us. We're the problem. Or it's a balanced tree, in which case
                // weightCorrection will be 0.
                return programMap.get(node).weight + weightCorrection;
            } else {
                // Find the "common" weight and the weight of the imbalanced subtree.
                int imbalancedWeight = -1;
                int commonWeight = -1;
                for (Multiset.Entry<Integer> entry : weightToFrequency.entrySet()) {
                    if (entry.getCount() == 1) {
                        imbalancedWeight = entry.getElement();
                    } else {
                        commonWeight = entry.getElement();
                    }
                }

                // Calculate the difference needed to bring the subtree back into alignment.
                if (imbalancedWeight != commonWeight) {
                    weightCorrection = commonWeight - imbalancedWeight;
                }

                // Start traversing the dependents of the imbalanced subtree.
                node = Iterables.getOnlyElement(weightToProgram.get(imbalancedWeight));
            }
        }
    }

    /**
     * For each program (identified by {@link Program#name}), calculate the weight of all dependent programs.
     */
    private static ImmutableMap<String, Integer> calculateCumulativeWeights(
            ImmutableMap<String, Program> programs,
            ImmutableMultimap<String, String> graph,
            Program root) {
        // Calculate the weight of the tree at each level including dependents.
        Map<String, Integer> weightOn = new HashMap<>();
        calculateWeightOfDependents(programs, graph, weightOn, root.name);

        return ImmutableMap.copyOf(weightOn);
    }

    /**
     * Recursive depth-first traversal which calculates the weight of a program and all dependent programs.
     */
    private static void calculateWeightOfDependents(
            ImmutableMap<String, Program> programMap,
            ImmutableMultimap<String, String> graph,
            Map<String, Integer> weightOn,
            String root) {
        int weight = programMap.get(root).weight;
        for (String dependent : graph.get(root)) {
            calculateWeightOfDependents(programMap, graph, weightOn, dependent);
            weight += weightOn.get(dependent);
        }

        weightOn.put(root, weight);
    }

    private static ImmutableList<Program> parsePrograms(List<String> input) {
        return input
                .stream()
                .map(Day07::parseProgram)
                .collect(ImmutableList.toImmutableList());
    }

    /** The root of the "program tree" will not have a parent.  */
    private static Program findRoot(ImmutableList<Program> programs) {
        ImmutableMap<String, String> parentGraph = buildParentGraph(programs);

        return programs.stream().filter(program -> !parentGraph.containsKey(program.name)).findFirst().get();
    }

    /**
     * Child -> parent graph. This graph is actually a tree, so we don't need to worry about multiple incoming edges.
     */
    private static ImmutableMap<String, String> buildParentGraph(List<Program> programs) {
        Map<String, String> parentGraph = new HashMap<>();
        for (Program program : programs) {
            for (String child : program.getDependents()) {
                parentGraph.put(child, program.name);
            }
        }

        return ImmutableMap.copyOf(parentGraph);
    }

    /** Typical parent -> [child] graph. */
    private static ImmutableMultimap<String, String> buildGraph(List<Program> programs) {
        Multimap<String, String> graph = HashMultimap.create();
        for (Program program : programs) {
            for (String child : program.getDependents()) {
                graph.put(program.name, child);
            }
        }

        return ImmutableMultimap.copyOf(graph);
    }

    private static final Pattern PROGRAM_PATTERN = Pattern.compile("([a-z]+) \\((\\d+)\\).*");
    private static Program parseProgram(String line) {
        Matcher matcher = PROGRAM_PATTERN.matcher(line);
        if (matcher.find()) {
            ImmutableSet<String> dependents = ImmutableSet.of();

            // Parse edges.
            if (line.contains(" -> ")) {
                dependents = ImmutableSet.copyOf(line.split(" -> ")[1].split(", "));
            }

            return  new Program(
                    matcher.group(1),
                    Integer.valueOf(matcher.group(2)),
                    dependents
            );
        } else {
            throw new IllegalArgumentException("Line didn't match: " + line);
        }
    }

    @Value
    private static class Program {
        String name;
        int weight;

        ImmutableSet<String> dependents;
    }
}

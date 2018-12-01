package aoc2017;

import aoc.Util;
import com.google.common.base.Splitter;
import com.google.common.collect.HashMultimap;
import com.google.common.collect.ImmutableSet;
import com.google.common.collect.Multimap;
import lombok.Value;

import java.util.*;
import java.util.stream.Collectors;

public class Day12 {
    public static void main(String[] args) {
        List<String> input = Util.input(2017, 12);

        ConnectedComponents<Integer> result = ConnectedComponents.fromGraph(buildGraph(input));
        System.out.println("Part 1: " + result.itemsInComponent(result.groupId(0)).size());
        System.out.println("Part 2: " + result.getComponentIdToItem().keySet().size());
    }


    private static Multimap<Integer, Integer> buildGraph(List<String> input) {
        Multimap<Integer, Integer> graph = HashMultimap.create();

        for(String line : input) {
            String[] parts = line.split(" <-> ");

            int program = Integer.parseInt(parts[0]);
            List<Integer> directConnections = Splitter.on(',').trimResults()
                    .splitToList(parts[1])
                    .stream()
                    .map(Integer::parseInt)
                    .collect(Collectors.toList());

            graph.putAll(program, directConnections);
        }

        return graph;
    }

    @Value
    static class ConnectedComponents<T> {
        Map<T, Integer> itemToComponentId;
        Multimap<Integer, T> componentIdToItem;

        public Integer groupId(T item) {
            return itemToComponentId.get(item);
        }

        public ImmutableSet<T> itemsInComponent(Integer componentId) {
            return ImmutableSet.copyOf(componentIdToItem.get(componentId));
        }

        /**
         * Less optimal but fast to write. Would be better as Union-Find.
         */
        public static <T> ConnectedComponents<T> fromGraph(Multimap<T, T> graph) {
            Map<T, Integer> itemToGroupId = new HashMap<>();
            Multimap<Integer, T> groupIdToItem = HashMultimap.create();

            int i = 0;
            for (T item : graph.keySet()) {
                if (itemToGroupId.containsKey(item)) {
                    continue;
                }

                i++;
                Deque<T> stack = new ArrayDeque<>();
                stack.push(item);

                while (!stack.isEmpty()) {
                    T current = stack.pop();

                    // Add node to group.
                    itemToGroupId.put(current, i);
                    groupIdToItem.put(i, current);

                    // Add all connections to stack.
                    for (T connection : graph.get(current)) {
                        if (!itemToGroupId.containsKey(connection)) {
                            stack.add(connection);
                        }
                    }
                }
            }
            return new ConnectedComponents<>(itemToGroupId, groupIdToItem);
        }
    }
}

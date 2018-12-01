package aoc2017;

import aoc.Util;

import java.util.*;

public class Day14 {
    private static final int DISK_WIDTH = 128;

    public static void main(String[] args) {
        Util.assertThat(squaresUsed("flqrgnkx")).isEqualTo(8108);
        Util.assertThat(regions("flqrgnkx")).isEqualTo(1242);

        System.out.println("Part 1: " + squaresUsed("hfdlxzhv"));
        System.out.println("Part 2: " + regions("hfdlxzhv"));
    }

    private static int squaresUsed(String key) {
        BitSet disk = diskFromKey(key);

        return disk.cardinality();
    }

    private static int regions(String key) {
        BitSet disk = diskFromKey(key);

        // Run connected components
        Map<Integer, Integer> indexToComponent = new HashMap<>();
        Deque<Integer> indexes = new ArrayDeque<>();
        for (int id = 0; id < disk.size(); id++) {
            indexes.clear();
            indexes.add(id);

            while(!indexes.isEmpty()) {
                int currentIndex = indexes.pop();
                if (currentIndex < 0
                        || currentIndex >= disk.size()
                        || !disk.get(currentIndex)
                        || indexToComponent.containsKey(currentIndex)) {
                    continue;
                }

                indexToComponent.put(currentIndex, id);

                // Add adjacent values
                // 0   1   2   3   4 ...
                // 128 129 130 131 132
                indexes.push(currentIndex - DISK_WIDTH);
                indexes.push(currentIndex + DISK_WIDTH);

                // Left boundary
                if (currentIndex % DISK_WIDTH != 0) {
                    indexes.push(currentIndex - 1);
                }
                // Right boundary
                if ((currentIndex + 1) % DISK_WIDTH != 0) {
                    indexes.push(currentIndex + 1);
                }
            }
        }

        return (int) indexToComponent.values().stream().distinct().count();
    }

    private static BitSet diskFromKey(String key) {
        BitSet disk = new BitSet(DISK_WIDTH * DISK_WIDTH);

        for (int i = 0; i < DISK_WIDTH; i++) {
            String hash = Day10.knotHash(key + "-" + i);

            int start = i*DISK_WIDTH;

            for (int j = 0; j < hash.length(); j++) {
                int idx = start + j * 4;
                int value = Integer.parseInt(hash.substring(j, j+1), 16);

                for (int k = 0; k < 4; k++) {
                    boolean bit = (value & 1 << k) != 0;
                    int bitIndex = idx + 3 - k;
                    disk.set(bitIndex, bit);
                }
            }
        }
        return disk;
    }
}

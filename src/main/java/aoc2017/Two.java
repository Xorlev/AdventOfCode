package aoc2017;

import aoc.Util;
import com.google.common.base.CharMatcher;
import com.google.common.base.Splitter;
import com.google.common.collect.*;

import java.util.Collection;
import java.util.Collections;
import java.util.List;
import java.util.stream.Collectors;

/**
 * The spreadsheet consists of rows of apparently-random numbers. To make sure the recovery process is on the right
 * track, they need you to calculate the spreadsheet's checksum. For each row, determine the difference between the
 * largest value and the smallest value; the checksum is the sum of all of these differences.

 For example, given the following spreadsheet:

 5 1 9 5
 7 5 3
 2 4 6 8
 The first row's largest and smallest values are 9 and 1, and their difference is 8.
 The second row's largest and smallest values are 7 and 3, and their difference is 4.
 The third row's difference is 6.
 In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
 */
public class Two {
    public static void main(String[] args) {
        Table<Integer, Integer, Integer> demoSpreadsheet1 = readSpreadsheet(
                ImmutableList.of(
                        "5 1 9 5",
                        "7 5 3",
                        "2 4 6 8"
                )
        );
        Table<Integer, Integer, Integer> demoSpreadsheet2 = readSpreadsheet(
                ImmutableList.of(
                        "5 9 2 8",
                        "9 4 7 3",
                        "3 8 6 5"
                )
        );

        Util.assertThat(checksumPartOne(demoSpreadsheet1)).isEqualTo(18);

        Table<Integer, Integer, Integer> spreadsheet = readSpreadsheet(Util.input(2017, 2));
        System.out.println("Part 1: " + checksumPartOne(spreadsheet));

        Util.assertThat(checksumPartTwo(demoSpreadsheet2)).isEqualTo(9);

        System.out.println("Part 2: " + checksumPartTwo(spreadsheet));
    }

    private static int checksumPartOne(Table<Integer, Integer, Integer> spreadsheet) {
        int checksum = 0;
        for (int i = 0; i < spreadsheet.rowKeySet().size(); i++) {
            Collection<Integer> values = spreadsheet.row(i).values();

            int min = Integer.MAX_VALUE;
            int max = Integer.MIN_VALUE;
            for (int value : values) {
                if (value > max) {
                    max = value;
                }
                if (value < min) {
                    min = value;
                }
            }

            checksum += max - min;
        }

        return checksum;
    }

    private static int checksumPartTwo(Table<Integer, Integer, Integer> spreadsheet) {
        int checksum = 0;
        for (int i = 0; i < spreadsheet.rowKeySet().size(); i++) {
            List<Integer> values = Lists.newArrayList(spreadsheet.row(i).values());
            Collections.sort(values);

            int result = 0;
            for (int d = 0; d < values.size() / 2; d++) {
                for (int n = values.size() - 1; n >= values.size() / 2; n--) {
                    Integer numerator = values.get(n);
                    Integer denominator = values.get(d);
                    if (numerator % denominator == 0) {
                        result = numerator / denominator;
                    }
                }
            }

            checksum += result;
        }

        return checksum;
    }

    private static Table<Integer, Integer, Integer> readSpreadsheet(List<String> rows) {
        Table<Integer, Integer, Integer> spreadsheet = HashBasedTable.create();

        for (int i = 0; i < rows.size(); i++) {
            List<Integer> rowValues = Splitter.on(CharMatcher.BREAKING_WHITESPACE)
                    .splitToList(rows.get(i))
                    .stream()
                    .map(Integer::valueOf)
                    .collect(Collectors.toList());

            for (int j = 0; j < rowValues.size(); j++) {
                spreadsheet.put(i, j, rowValues.get(j));
            }
        }

        return ImmutableTable.copyOf(spreadsheet);
    }
}

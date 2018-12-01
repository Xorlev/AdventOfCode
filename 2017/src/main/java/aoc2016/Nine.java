package aoc2016;

import aoc.Util;
import com.google.common.base.Splitter;
import com.google.common.base.Strings;

import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * Decompress strings of the form X(8x2)(3x3)ABCY, where a (NxM) pair means replicate the next N characters M times.
 * Characters copied inside of N are skipped, e.x. in 'X(8x2)(3x3)ABCY', (3x3) is within the 8-character window and
 * is outputted directly. Decompression starts again after skipping the N chars.
 *
 * @author Michael Rose (xorlev)
 */
public class Nine {
    public static void main(String[] args) {
        String input = Util.input(2016, 9).get(0);

//        String input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";

        String output1 = v1(input);
        long output2 = v2(input);

//        System.out.println(output1.length());
//        System.out.println(output2);
        System.out.println(output2);
    }

    private static String v1(String input) {
        StringBuilder output = new StringBuilder();
        for(int i = 0; i < input.length(); i++) {
            char c = input.charAt(i);
            if(c != '(') {
                output.append(c);
            } else {
                int end = input.indexOf(')', i);
                String spec = input.substring(i+1, end);
                String[] specPieces = spec.split("x");
                int chars = Integer.valueOf(specPieces[0]);
                int repetitions = Integer.valueOf(specPieces[1]);

                int copyFrom = end + 1;
                int copyEnd = end + chars + 1;

                i += chars + spec.length() + 1;

                output.append(Strings.repeat(input.substring(copyFrom, copyEnd), repetitions));
            }
        }
        return output.toString();
    }

    private static long v2(String input) {
        long total = 0;

        System.out.println("in: " + input);
        for(int i = 0; i < input.length(); i++) {

            char c = input.charAt(i);
            if(c == '(') {
                int end = input.indexOf(')', i);
                String spec = input.substring(i+1, end);
                String[] specPieces = spec.split("x");
                int chars = Integer.valueOf(specPieces[0]);
                int repetitions = Integer.valueOf(specPieces[1]);

                System.out.println(spec);

                int copyFrom = end + 1;
                int copyEnd = end + chars + 1;
                String s = input.substring(copyFrom, copyEnd);
                long rlen = v2(s);
                total += repetitions * rlen;



                i += spec.length() + chars + 1;
            } else {
                total += 1;
            }
        }

        System.out.println("total ("+input+"): " + total);
        return total;
    }
}

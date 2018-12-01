package aoc2016;

import aoc.Util;
import com.google.common.collect.HashMultimap;
import com.google.common.collect.Iterables;
import com.google.common.collect.Lists;
import com.google.common.collect.SetMultimap;
import lombok.Value;

import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * 2016-12-30
 *
 * @author Michael Rose (xorlev)
 */
public class Ten {
    @Value
    public static class Bot {
        String name;
        String lowRecipient;
        String highRecipient;
    }

    public static void main(String[] args) {
        List<String> instructions = Util.input(2016, 10);

        // Map[Bot, Set[Chip]]
        Map<String, Bot> flow = new HashMap<>();
        SetMultimap<String, Integer> has = HashMultimap.create();

        // Initialize ownership
        // bot 200 gives low to bot 95 and high to bot 181
        Pattern givesPattern = Pattern.compile("(bot \\d+) gives low to (bot \\d+|output \\d+) and high to (bot \\d+|output \\d+)");
        for(String instruction : instructions) {
            Matcher matcher = givesPattern.matcher(instruction);
            if(matcher.find()) {
                flow.put(matcher.group(1), new Bot(matcher.group(1), matcher.group(2), matcher.group(3)));
            }
        }
        // Initialize ownership
        // value 3 goes to bot 67
        Pattern ownsPattern = Pattern.compile("value (\\d+) goes to (bot \\d+)");
        for(String instruction : instructions) {
            Matcher matcher = ownsPattern.matcher(instruction);
            if(matcher.find()) {
                moveChip(flow, has, "input", matcher.group(2), Integer.valueOf(matcher.group(1)));
            }
        }

        // Part 2: add chip values of output{0,1,2}
        Integer zero = Iterables.getOnlyElement(has.get("output 0"));
        Integer one = Iterables.getOnlyElement(has.get("output 1"));
        Integer two = Iterables.getOnlyElement(has.get("output 2"));

        System.out.println(zero*one*two);
    }

    static void moveChip(Map<String, Bot> flow, SetMultimap<String, Integer> has, String from, String to, int chip) {
        has.remove(from, chip);
        has.put(to, chip);

        if(has.get(to).containsAll(Lists.newArrayList(17, 61))) {
            System.out.println("Bot " + to + " compares (17, 61)");
        }

        if(has.get(to).size() == 2) {
            Bot spec = flow.get(to);
            Set<Integer> chips = has.get(to);
            moveChip(flow, has, to, spec.getLowRecipient(), Collections.min(chips));
            moveChip(flow, has, to, spec.getHighRecipient(), Collections.max(chips));
        }
    }
}

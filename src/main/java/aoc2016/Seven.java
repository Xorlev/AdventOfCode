package aoc2016;

import aoc.Util;
import com.google.common.base.CharMatcher;
import com.google.common.base.Splitter;
import com.google.common.collect.Lists;

import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * Attempts to determine if an "IPv7" IP supports "TLS". If an IP has an "ABBA" outside of square brackets and NOT in
 * square brackets, it supports TLS
 *
 * @author Michael Rose (xorlev)
 */
public class Seven {
    public static void main(String[] args) {
        List<String> ips = Util.input(2016, 7);
        int supportsTls = 0;
        int supportsSsl = 0;
        for(String ip : ips) {
            if(supportsTls(ip))
                supportsTls++;
            if(supportsSsl(ip))
                supportsSsl++;
        }

        System.out.println(supportsTls);
        System.out.println(supportsSsl);
    }

    static final Pattern IP_PATTERN = Pattern.compile("([a-z]+)\\[([a-z]+)\\]([a-z]+)");
    static boolean supportsTls(String ip) {
        Matcher m = IP_PATTERN.matcher(ip);

        if(m.find()) {
            List<String> parts = Splitter.on(Pattern.compile("\\[|\\]")).splitToList(ip);

            boolean outsideAbba = false;
            boolean insideAbba = false;
            for(int i = 0; i < parts.size(); i++) {
                String part = parts.get(i);
                if(i % 2 == 0) {
                    outsideAbba |= hasAbba(part);
                } else {
                    insideAbba |= hasAbba(part);
                }
            }

            return outsideAbba && !insideAbba;
        } else {
            throw new IllegalArgumentException("Invalid IP: " + ip);
        }
    }

    static final char[] alphabet = "abcdefghijklmnopqrstuvwxyz".toCharArray();
    static boolean supportsSsl(String ip) {
        Matcher m = IP_PATTERN.matcher(ip);

        if(m.find()) {
            List<String> parts = Splitter.on(Pattern.compile("\\[|\\]")).splitToList(ip);
            List<String> outside = new ArrayList<>(parts.size() / 2);
            List<String> inside= new ArrayList<>(parts.size() / 2);
            for(int i = 0; i < parts.size(); i++) {
                if(i % 2 == 0) {
                    outside.add(parts.get(i));
                } else {
                    inside.add(parts.get(i));
                }
            }

            for(char a : alphabet) {
                for(char b : alphabet) {
                    if(a != b) {
                        if(outside.stream().anyMatch(c -> c.contains(""+a+b+a))
                               && inside.stream().anyMatch(c -> c.contains(""+b+a+b)))
                            return true;
                    }
                }
            }
        } else {
            throw new IllegalArgumentException("Invalid IP: " + ip);
        }

        return false;
    }

    static boolean hasAbba(String str) {
        if(str.length() < 4)
            return false;

        for(int i = 0; i < str.length() - 3; i++) {
            if(str.charAt(i) == str.charAt(i+3)
               && str.charAt(i+1) == str.charAt(i+2)
               && str.charAt(i) != str.charAt(i+1))
                return true;
        }

        return false;
    }
}

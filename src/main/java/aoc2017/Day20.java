package aoc2017;

import aoc.Util;
import lombok.Builder;
import lombok.Data;
import lombok.Value;

import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day20 {
    public static void main(String[] args) {
        List<Particle> particles = new ArrayList<>();
        int pid = 0;
        for (String particle : Util.input(2017, 20)) {
            particles.add(parse(pid++, particle));
        }

        // Particle moving the _slowest_ that starts closest to <0,0,0> will eventually be closest.
        Comparator<Particle> comparator = Comparator
                .comparingDouble(p -> p.acceleration.magnitude());
        Optional<Particle> min = particles
                .stream()
                .min(comparator.thenComparingDouble(p -> p.position.magnitude()));

        System.out.println("Part 1: " + min);

        Map<Vector3D, Particle> positions = new HashMap<>(particles.size());
        Set<Particle> toRemove = new HashSet<>();
        for (int i = 0; i < 1_000; i++) {
            positions.clear();
            for (Particle p : particles) {
                if (positions.containsKey(p.position)) {
                    toRemove.add(p);
                    toRemove.add(positions.get(p.position));
                } else {
                    positions.put(p.position, p);
                }
            }

            for (Particle p : particles) {
                if (!toRemove.contains(p)) {
                    p.step();
                }
            }
        }

        System.out.println("Part 2: " + (particles.size() - toRemove.size()));
    }

    // p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
    private static final Pattern PATTERN = Pattern.compile(
            "p=<([^,]+),([^,]+),([^,]+)>, v=<([^,]+),([^,]+),([^,]+)>, a=<([^,]+),([^,]+),([^,]+)>"
    );
    static Particle parse(int id, String input) {
        Matcher matcher = PATTERN.matcher(input);

        if (matcher.find()) {
            return Particle
                    .builder()
                    .id(id)
                    .position(
                        new Vector3D(
                            Integer.parseInt(matcher.group(1)),
                            Integer.parseInt(matcher.group(2)),
                            Integer.parseInt(matcher.group(3))
                        )
                    )
                    .velocity(
                        new Vector3D(
                            Integer.parseInt(matcher.group(4)),
                            Integer.parseInt(matcher.group(5)),
                            Integer.parseInt(matcher.group(6))
                        )
                    )
                    .acceleration(
                        new Vector3D(
                            Integer.parseInt(matcher.group(7)),
                            Integer.parseInt(matcher.group(8)),
                            Integer.parseInt(matcher.group(9))
                        )
                    )
                    .build();
        } else {
            throw new IllegalArgumentException("Unable to parse particle: " + input);
        }
    }

    @Data
    @Builder
    static class Particle {
        int id;
        Vector3D position;
        Vector3D velocity;
        Vector3D acceleration;

        Particle step() {
            velocity = velocity.add(acceleration);
            position = position.add(velocity);

            return this;
        }
    }

    @Value
    static class Vector3D {
        int x,y,z;

        public Vector3D add(Vector3D other) {
            return new Vector3D(
                    x+other.x,
                    y+other.y,
                    z+other.z
            );
        }

        public double magnitude() {
            return Math.sqrt(Math.abs(x*x)+Math.abs(y*y)+Math.abs(z*z));
        }
    }
}

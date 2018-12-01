package aoc2016;

import aoc.Util;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.ImmutableSet;
import com.google.common.collect.Lists;
import com.google.common.collect.Sets;
import lombok.Value;

import java.util.ArrayList;
import java.util.Collection;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

/**
 * 2016-12-27
 *
 * @author Michael Rose (xorlev)
 */
public class Eleven {
    private static Set<Integer> LEGAL_FLOORS = ImmutableSet.of(0, 1, 2, 3);
    enum Item {
        TG, TM,
        PLG, PLM,
        PRG, PRM,
        SG, SM,
        RG, RM,
        EG, EM,
        DG, DM
    }

    @Value
    public static class State {
        int elevator;
        ImmutableList<ImmutableSet<Item>> floors;

        public State moveItemsToFloor(int newFloor, Set<Item> items) {
            ImmutableList.Builder<ImmutableSet<Item>> newFloors = ImmutableList.builder();
            for(int i = 0; i < floors.size(); i++) {
                Set<Item> floorItems = floors.get(i);

                if(i == elevator) {
                    floorItems = Sets.difference(floorItems, items);
                } else if (i == newFloor) {
                    floorItems = Sets.union(floorItems, items);
                }

                if(!legalState(floorItems))
                    return null;

                newFloors.add(ImmutableSet.copyOf(floorItems));
            }

            return new State(
                newFloor,
                newFloors.build()
            );
        }

        public ImmutableSet<Item> itemsOnFloor() {
            return floors.get(elevator);
        }
    }

    public static Collection<ImmutableSet<Item>> permutations(Set<Item> items) {
        Set<ImmutableSet<Item>> permutations = new HashSet<>();
        for(Item i1 : items) {
            // Add base case: one item
            permutations.add(ImmutableSet.of(i1));
            for(Item i2 : items) {
                permutations.add(ImmutableSet.of(i1, i2));
            }
        }

        return permutations;
    }

    public static List<State> moveFrom(State state) {
        // for floor above and below
        // can we move any combination of items to that floor?

        List<State> states = new ArrayList<>();
        for(Integer floor : Sets.intersection(LEGAL_FLOORS, ImmutableSet.of(state.elevator-1, state.elevator+1))) {
            // all combinations of size <= 2

            for(ImmutableSet<Item> itemCombo : permutations(state.itemsOnFloor())) {
                State newState = state.moveItemsToFloor(floor, itemCombo);

                if(newState != null)
                    states.add(newState);
            }
        }

        return states;
    }

    public static boolean legalState(Collection<Item> floor) {
        Set<Item> generators = new HashSet<>(7);

        for(Item i : floor)
            if(i.name().endsWith("G"))
                generators.add(i);

        // if there is any generator, chips must be paired with their generator
        if(!generators.isEmpty()) {
            for(Item i : floor) {
                if(i.name().endsWith("M")) {
                    // we must have its generator
                    if(!generators.contains(generatorFor(i))) {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    public static Item generatorFor(Item i) {
        return Item.valueOf(i.name().replace('M', 'G'));
    }

    public static double costToTop(State state) {
        double cost = 0;

        List<ImmutableSet<Item>> floors = Lists.reverse(state.floors);

        // Cost should be 0 at the top and multiplied by floor at the bottom
        for(int i = 0; i < floors.size(); i++) {
            cost += floors.get(i).size()*i*101;
        }

        return cost / 2; // up to two items per move
    }

    public static void main(String[] args) throws Exception {
        State easy = new State(0, ImmutableList.of(
            ImmutableSet.of(Item.RG),
            ImmutableSet.of(),
            ImmutableSet.of(Item.RM),
            ImmutableSet.of()
        ));

        /**
         * The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
         The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
         The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
         The fourth floor contains nothing relevant.
         */
        State part1 = new State(0, ImmutableList.of(
            ImmutableSet.of(Item.TG, Item.TM, Item.PLG, Item.SG),
            ImmutableSet.of(Item.PLM, Item.SM),
            ImmutableSet.of(Item.PRG, Item.PRM, Item.RG, Item.RM),
            ImmutableSet.of()
        ));
        State part2 = new State(0, ImmutableList.of(
            ImmutableSet.of(Item.TG, Item.TM, Item.PLG, Item.SG, Item.EG, Item.EM, Item.DG, Item.DM),
            ImmutableSet.of(Item.PLM, Item.SM),
            ImmutableSet.of(Item.PRG, Item.PRM, Item.RG, Item.RM),
            ImmutableSet.of()
        ));

        // legal floors:
        //  - no generators unless chip's RTG

        // legal moves:
        //  - at least one item
        //  - if two items, it must either be an item & corresponding RTG, or 2 microchips to a floor w/ both RTGs

        Util.AStarResult<State> result1 = Util.timeIt(() -> Util.astarSearch(part1, Eleven::costToTop, Eleven::moveFrom));
        System.out.println(result1);
        System.out.println(result1.getPath().size() - 1);
        System.out.println("---- PART 2 ----");
        Util.AStarResult<State> result2 = Util.timeIt(() -> Util.astarSearch(part2, Eleven::costToTop, Eleven::moveFrom));
        System.out.println(result2);
        System.out.println(result2.getPath().size() - 1);
    }
}

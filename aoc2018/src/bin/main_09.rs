use failure::*;
use util::aoc::*;
use util::aoc::linked_list::*;
use std::collections::HashMap;

type Marble = u32;
type Elf = u32;

fn main() -> Result<(), Box<std::error::Error>> {
    result("Part 1", || part1(459, 72103));
    result("Part 2", || part1(459, 72103*100));

    Ok(())
}

fn part1(players: u32, last_marble: Marble) -> u32 {
    let mut state = GameState::new(players, last_marble);
    for i in 0..last_marble {
        let player = i%players + 1;
        state.play(player);
    }

    *state.high_score().1
}

#[derive(Debug)]
struct GameState {
    current_marble: Marble,
    current_marble_idx: Idx,
    marbles: UsefulLinkedList<Marble>,
    scores: HashMap<Elf, u32>
}

impl GameState {
    fn new(players: u32, max_marbles: Marble) -> GameState {
        let mut linked_list = UsefulLinkedList::new();
        let idx = linked_list.push_back(0);

        GameState {
            current_marble: 1,
            current_marble_idx: idx,
            marbles: linked_list,
            scores: HashMap::new(),
        }
    }

    fn play(&mut self, elf: Elf) {
        // The Elves play this game by taking turns arranging the marbles in a circle according to
        // very particular rules. The marbles are numbered starting with 0 and increasing by 1 until
        // every marble has a number.
        //
        // First, the marble numbered 0 is placed in the circle. At this point, while it contains
        // only a single marble, it is still a circle: the marble is both clockwise from itself and
        // counter-clockwise from itself. This marble is designated the current marble.
        //
        // Then, each Elf takes a turn placing the lowest-numbered remaining marble into the circle
        // between the marbles that are 1 and 2 marbles clockwise of the current marble. (When the
        // circle is large enough, this means that there is one marble between the marble that was
        // just placed and the current marble.) The marble that was just placed then becomes the
        // current marble.
        //
        // However, if the marble that is about to be placed has a number which is a multiple of 23,
        // something entirely different happens. First, the current player keeps the marble they
        // would have placed, adding it to their score. In addition, the marble 7 marbles
        // counter-clockwise from the current marble is removed from the circle and also added to
        // the current player's score. The marble located immediately clockwise of the marble that
        // was removed becomes the new current marble.

        if self.current_marble % 23 == 0 {
            // Find marble 7 steps back.
            for _ in 0..7 {
                self.current_marble_idx = self.prev_index();
            }

            let next_marble_idx = self.next_index();

            // Remove that marble.
            let marble = self.marbles.remove(self.current_marble_idx);

            // Record the marble we're removing + the marble we almost placed.
            let score = self.scores.entry(elf).or_default();
            *score += marble + self.current_marble;

            self.current_marble_idx = next_marble_idx;
        } else {
            self.current_marble_idx =
                self.marbles.insert_after(self.next_index(), self.current_marble);
        }


        self.current_marble += 1;
    }

    fn high_score(&self) -> (&Elf, &u32) {
        self.scores.iter().max_by_key(|(_, &score)| score).unwrap()
    }

    // Finds the next index to insert after by looking at the "next" pointer.
    fn next_index(&self) -> Idx {
        match self.marbles[self.current_marble_idx].next {
            None => self.marbles.head().unwrap(),
            Some(next) => next,
        }
    }

    // Finds the next index to insert after by looking at the "next" pointer.
    fn prev_index(&self) -> Idx {
        match self.marbles[self.current_marble_idx].prev {
            None => self.marbles.tail().unwrap(),
            Some(prev) => prev,
        }
    }
}

mod test {
    #[test]
    fn test() {
        assert_eq!(32, super::part1(9, 25));
        assert_eq!(8317, super::part1(10, 1618));
        assert_eq!(146373, super::part1(13, 7999));
    }
}
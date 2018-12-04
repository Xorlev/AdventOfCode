use chrono::{NaiveDateTime, Timelike};
use failure::*;
use itertools::Itertools;
use lazy_static::*;
use regex::Regex;
use std::collections::hash_map::HashMap;
use util::aoc::*;

lazy_static! {
    static ref RE_SHIFT: Regex = Regex::new("\\[(.+)\\] Guard #(\\d+) begins shift").unwrap();
    static ref RE_WAKES: Regex = Regex::new("\\[(.+)\\] wakes up").unwrap();
    static ref RE_SLEEP: Regex = Regex::new("\\[(.+)\\] falls asleep").unwrap();
}

#[derive(Debug)]
enum EventType {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

#[derive(Debug)]
struct Event {
    timestamp: NaiveDateTime,
    event_type: EventType,
}

impl Event {
    fn minute(&self) -> u32 {
        self.timestamp.minute()
    }

    fn parse(event_str: &String) -> Result<Event, Error> {
        if let Some(captures) = RE_SHIFT.captures(event_str.as_str()) {
            let timestamp = Event::parse_time(&captures[1])?;
            let event = Event {
                timestamp,
                event_type: EventType::BeginShift(captures[2].parse::<u32>()?),
            };

            return Ok(event);
        }
        if let Some(captures) = RE_WAKES.captures(event_str.as_str()) {
            let timestamp = Event::parse_time(&captures[1])?;
            let event = Event {
                timestamp,
                event_type: EventType::WakeUp,
            };

            return Ok(event);
        }
        if let Some(captures) = RE_SLEEP.captures(event_str.as_str()) {
            let timestamp = Event::parse_time(&captures[1])?;
            let event = Event {
                timestamp,
                event_type: EventType::FallAsleep,
            };

            return Ok(event);
        }

        Err(format_err!("Couldn't parse event: {}", event_str))
    }

    fn parse_time(time_str: &str) -> Result<NaiveDateTime, chrono::ParseError> {
        NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M")
    }
}

#[derive(Debug, Default)]
struct SolutionState {
    guard_to_minute_frequency: HashMap<u32, HashMap<u32, u32>>,
}

impl SolutionState {
    fn compute_state(events: &Vec<Event>) -> Result<SolutionState, Error> {
        let mut state = SolutionState::default();

        let mut guard_on_shift = 0u32;
        let mut sleep_event: Option<&Event> = None;
        for event in events {
            match event.event_type {
                EventType::BeginShift(guard) => {
                    guard_on_shift = guard;
                }
                EventType::FallAsleep => {
                    sleep_event = Some(event);
                }
                EventType::WakeUp => {
                    let sleep_start = sleep_event
                        .ok_or(format_err!("Wakeup before sleep event."))
                        .map(Event::minute)?;
                    let sleep_end = event.minute();
                    for minute in sleep_start..sleep_end {
                        let frequency = state
                            .guard_to_minute_frequency
                            .entry(guard_on_shift)
                            .or_default()
                            .entry(minute)
                            .or_default();

                        *frequency += 1;
                    }

                    sleep_event = None;
                }
            }
        }

        Ok(state)
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(4)?;
    let events: Vec<Event> = lines
        .iter()
        .map(Event::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let events = events
        .into_iter()
        .sorted_by_key(|e| e.timestamp.timestamp());
    let state = time("Compute state", || SolutionState::compute_state(&events))?;

    result("Part 1", || part1(&state));
    result("Part 2", || part2(&state));

    Ok(())
}

// Strategy 1: Find the guard that has the most minutes asleep.
// What minute does that guard spend asleep the most?
// What is the ID of the guard you chose multiplied by the minute you chose?
// (In the above example, the answer would be 10 * 24 = 240.)
fn part1(state: &SolutionState) -> u32 {
    // Find the sleepiest guard.
    let sleepiest_guard = state
        .guard_to_minute_frequency
        .iter()
        // Take guard -> (minute -> frequency) and make it (guard, minutes).
        .map(|histogram| {
            (histogram.0, histogram.1.iter().map(|bucket| bucket.1).sum::<u32>())
        })
        .max_by_key(|v| v.1)
        .unwrap()
        .0;

    // Find the most frequent minute the guard was asleep.
    let guard_minute_histogram = state.guard_to_minute_frequency.get(sleepiest_guard);

    // Find the largest bucket (minute) this guard was asleep.
    let most_frequent_minute = match guard_minute_histogram {
        Some(histogram) => *histogram.iter().max_by_key(|v| *v.1).unwrap_or((&0, &0)).0,
        None => 0,
    };

    sleepiest_guard * most_frequent_minute
}

// Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?
fn part2(state: &SolutionState) -> u32 {
    // Find the minute a guard is most frequently asleep.
    // We do this by flattening all of the buckets into (guard, minute, times_asleep) and doing a
    // final O(n) pass over them to select the largest bucket.
    let sleepiest_minute_guard = state
        .guard_to_minute_frequency
        .iter()
        .flat_map(|(guard, minute_freq)| {
            minute_freq
                .iter()
                .map(move |(minute, frequency)| (guard, minute, frequency))
        })
        .max_by_key(|(_, _, frequency)| *frequency)
        .unwrap_or((&0, &0, &0));

    sleepiest_minute_guard.0 * sleepiest_minute_guard.1
}

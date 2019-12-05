use failure::*;
use itertools::Itertools;
use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use util::aoc::*;

// position=<-6, 10> velocity=< 2, -2>
lazy_static! {
    static ref RE: Regex = Regex::new(
        "position=<([\\s\\-0-9]+), ([\\s\\-0-9]+)> \
         velocity=<([\\s\\-0-9]+), ([\\s\\-0-9]+)>"
    )
    .unwrap();
}

#[derive(Copy, Clone, Debug)]
struct Light {
    position: Point,
    velocity: Point,
}

impl Light {
    fn step(&mut self) {
        self.position = self.position + self.velocity;
    }
}

struct Sky {
    lights: Vec<Light>,
    points: HashMap<Point, Light>,
}

impl Sky {
    fn new(lights: Vec<Light>) -> Sky {
        Sky {
            points: lights.iter().map(|l| (l.position, *l)).collect(),
            lights,
        }
    }

    fn step(&mut self) {
        self.lights.iter_mut().for_each(Light::step);
        // Re-index lights by point.
        self.points.clear();

        let mut points = &mut self.points;
        self.lights.iter().for_each(|l| {
            points.insert(l.position, *l);
        });
    }

    // Returns top-left and bottom-right
    fn bounds(&self) -> (Point, Point) {
        let x_minmax = self
            .lights
            .iter()
            .map(|l| l.position.x)
            .minmax()
            .into_option()
            .unwrap();
        let y_minmax = self
            .lights
            .iter()
            .map(|l| l.position.y)
            .minmax()
            .into_option()
            .unwrap();

        (
            Point::new(x_minmax.0, y_minmax.0),
            Point::new(x_minmax.1, y_minmax.1),
        )
    }

    fn probably_has_text(&self) -> bool {
        let bounds = self.bounds();

        // If the bounds are say >100 points apart, lets guess that it doesn't contain our message.
        if bounds.1.x - bounds.0.x > 100 || bounds.1.y - bounds.0.y > 10 {
            return false;
        }

        // Start at the left and scan down.
        for x in bounds.0.x..=bounds.1.x {
            let mut consecutive_points = 0;
            for y in bounds.0.y..=bounds.1.y {
                if self.points.contains_key(&Point::new(x, y)) {
                    consecutive_points += 1;

                    // Look for a vertical run of 10 characters. This is probably a letter.
                    if consecutive_points == 10 {
                        return true;
                    }
                } else {
                    consecutive_points = 0;
                }
            }
        }

        false
    }

    fn print(&self) {
        let bounds = self.bounds();

        for y in bounds.0.y..=bounds.1.y {
            for x in bounds.0.x..=bounds.1.x {
                if self.points.contains_key(&Point::new(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }

        println!("{}", "=".repeat((bounds.1.x - bounds.0.x) as usize))
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(10)?;
    let lights = lines
        .iter()
        .map(parse)
        .collect::<Result<Vec<Light>, Error>>()?;
    result("Part 1", || part1(lights.clone()));

    Ok(())
}

fn part1(lights: Vec<Light>) {
    let mut sky = Sky::new(lights);
    for i in 0..100000 {
        sky.step();

        if sky.probably_has_text() {
            sky.print();
            println!("Seconds {}", i + 1);
            break;
        }
    }
}

fn parse(line: &String) -> Result<Light, Error> {
    if let Some(captures) = RE.captures(line.as_str()) {
        Ok(Light {
            position: Point::new(captures[1].trim().parse()?, captures[2].trim().parse()?),
            velocity: Point::new(captures[3].trim().parse()?, captures[4].trim().parse()?),
        })
    } else {
        Err(format_err!("Didn't match input: {}", line))
    }
}

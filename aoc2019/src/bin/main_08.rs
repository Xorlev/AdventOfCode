use aoc2019::intcode::*;
use failure::_core::fmt::Formatter;
use failure::{bail, format_err, Error};
use itertools::Itertools;
use permutohedron::Heap;
use std::borrow::Borrow;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use util::aoc::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digits: Vec<i32> = input::read(8)?
        .first()
        .map(|f| {
            f.split("")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .unwrap_or(Vec::new());

    result("Part 1", || part1(digits.clone()));
    result("Part 2", || part2(digits.clone()));

    Ok(())
}

fn part1(digits: Vec<i32>) -> usize {
    println!("d: {:?}", digits);
    let layers = build_layers(digits);

    let layer_most_zeros = layers.into_iter().min_by_key(|l| l.count(0)).unwrap();

    layer_most_zeros.count(1) * layer_most_zeros.count(2)
}

fn part2(digits: Vec<i32>) -> i32 {
    let layers = build_layers(digits);

    println!("{}", Layer::compose(&layers));

    0
}

fn build_layers(digits: Vec<i32>) -> Vec<Layer> {
    let width = 6;
    let height = 25;
    let pixels = width * height;
    let mut layers: Vec<Layer> = Vec::new();
    for l in 0..(digits.len() / pixels) {
        let layer = Layer {
            height: width,
            width: height,
            pixels: digits[l * pixels..l * pixels + pixels].to_owned(),
        };

        layers.push(layer);
    }
    layers
}

#[derive(Debug)]
struct Layer {
    height: usize,
    width: usize,
    pixels: Vec<i32>,
}

impl Layer {
    fn count(&self, pixel_value: i32) -> usize {
        self.pixels.iter().filter(|p| **p == pixel_value).count()
    }

    fn compose(layers: &[Layer]) -> Layer {
        let mut composed = Vec::with_capacity(layers[0].pixels.len());
        for i in 0..layers[0].pixels.len() {
            let mut final_pixel = 2;
            for layer in layers {
                if layer.pixels[i] < 2 {
                    final_pixel = layer.pixels[i];
                    break;
                }
            }
            composed.push(final_pixel)
        }

        Layer {
            height: layers[0].height,
            width: layers[0].width,
            pixels: composed,
        }
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for h in 0..self.height {
            for w in 0..self.width {
                let pixel = self.pixels[h * self.width + w];
                let c = match pixel {
                    0 => " ",
                    1 => "1",
                    2 => " ",
                    _ => "",
                };
                f.write_str(c);
            }
            f.write_str("\n");
        }

        Ok(())
    }
}

#[macro_use] extern crate itertools;

use itertools::Itertools;

fn main() {
    use std::io::{self, Read};
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let cases: Vec<Vec<i32>> = buffer.split("\n")
                    .map(|s| s.trim())
                    .map(|l| l.split(" ").filter(|&x| x != "").map(|c| {println!("{:?}", c); c.parse::<i32>().unwrap()}).collect::<Vec<i32>>())
                    .filter(|l| l.len() > 0)
                    .collect();

    let valid_triangles1 = valid_triangles(&cases).len();
    println!("{:?}", valid_triangles1);

    // Next step: columnize
    let mut cases2: Vec<Vec<i32>> = Vec::new();
    for chunk in &cases.into_iter().chunks(3) {
        let mut piece1: Vec<i32> = Vec::new();
        let mut piece2: Vec<i32> = Vec::new();
        let mut piece3: Vec<i32> = Vec::new();

        for line in chunk {
            piece1.push(line[0]);
            piece2.push(line[1]);
            piece3.push(line[2]);
        }

        cases2.push(piece1);
        cases2.push(piece2);
        cases2.push(piece3);
    }

    let valid_triangles2 = valid_triangles(&cases2).len();
    println!("{:?}", valid_triangles2);
}

fn valid_triangles(cases: &Vec<Vec<i32>>) -> Vec<&Vec<i32>> {
    cases.iter()
         .filter(|x| x[0] + x[1] > x[2] && x[0] + x[2] > x[1] && x[2] + x[1] > x[0])
         .collect()
}

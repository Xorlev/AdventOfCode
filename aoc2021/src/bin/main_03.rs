use failure::bail;
use std::cmp::Ordering;
use std::hash::Hash;
use util::aoc::frequency::FrequencyMap;
use util::aoc::*;

fn main() -> AocResult<()> {
    let diagnostic_bits: Vec<String> = input::read(3)?;
    let columns = columnize(diagnostic_bits);

    result("Part 1", || part1(&columns));
    result("Part 2", || part2(&columns));

    Ok(())
}

fn columnize(input: Vec<String>) -> ColumnValues<i32> {
    let mut columns = ColumnValues::new(input[0].len());

    input
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .for_each(|bits| columns.add_row(bits).unwrap());

    columns
}

fn part1(columns: &ColumnValues<i32>) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in (0..columns.column_len()).rev() {
        let column_index = columns.column_len() - i - 1;
        let frequency = columns.column_frequency(column_index);
        let min = frequency.min().unwrap();
        let max = frequency.max().unwrap();

        gamma += max * 2i32.pow(i as u32);
        epsilon += min * 2i32.pow(i as u32);
    }

    gamma * epsilon
}

fn part2(columns: &ColumnValues<i32>) -> i32 {
    let mut oxygen_rows = columns.rows();
    let mut co2_rows = columns.rows();

    for i in 0..columns.column_len() {
        let (most_common, _) = find_common_values(&oxygen_rows, i, 1);
        oxygen_rows = filter_rows(oxygen_rows, i, most_common);
        if oxygen_rows.len() == 1 {
            break;
        }
    }

    for i in 0..columns.column_len() {
        let (_, least_common) = find_common_values(&co2_rows, i, 0);
        co2_rows = filter_rows(co2_rows, i, least_common);
        if co2_rows.len() == 1 {
            break;
        }
    }

    vec_to_digit(&oxygen_rows[0]) * vec_to_digit(&co2_rows[0])
}

fn find_common_values(values: &[Vec<&i32>], index: usize, default_if_equal: i32) -> (i32, i32) {
    let mut columns = ColumnValues::new(values[0].len());
    columns.add_rows(values).unwrap();

    let zero_freq = columns.column_frequency(index).count(&&0);
    let one_freq = columns.column_frequency(index).count(&&1);

    match zero_freq.cmp(&one_freq) {
        Ordering::Less => (1, 0),
        Ordering::Equal => (default_if_equal, default_if_equal),
        Ordering::Greater => (0, 1),
    }
}

fn filter_rows(rows: Vec<Vec<&i32>>, i: usize, most_common: i32) -> Vec<Vec<&i32>> {
    let mut filtered_rows = Vec::new();
    for row in rows {
        if *row[i] == most_common {
            filtered_rows.push(row);
        }
    }
    filtered_rows
}

fn vec_to_digit(row: &[&i32]) -> i32 {
    let mut value = 0;
    for i in 0..row.len() {
        let exp = row.len() - i - 1;
        value += row[i] * 2i32.pow(exp as u32);
    }
    value
}

pub struct ColumnValues<T: Clone + Eq + Hash> {
    columns_to_rows: Vec<Vec<T>>,
    frequency_per_column: Vec<FrequencyMap<T>>,
}

impl<T: Clone + Eq + Hash> ColumnValues<T> {
    pub fn new(columns: usize) -> ColumnValues<T> {
        let mut columns_to_rows = Vec::with_capacity(columns);
        let mut frequency_per_column = Vec::with_capacity(columns);
        for _ in 0..columns {
            columns_to_rows.push(Vec::new());
            frequency_per_column.push(FrequencyMap::new());
        }

        ColumnValues {
            columns_to_rows,
            frequency_per_column,
        }
    }

    pub fn add_rows(&mut self, rows: &[Vec<T>]) -> AocResult<()> {
        for row in rows {
            self.add_row(row.clone())?;
        }

        Ok(())
    }

    pub fn add_row(&mut self, row: Vec<T>) -> AocResult<()> {
        if self.columns_to_rows.len() != row.len() {
            bail!(
                "Row has {} values, but requires {}",
                row.len(),
                self.columns_to_rows.len()
            );
        }

        for (index, value) in row.into_iter().enumerate() {
            self.columns_to_rows[index].push(value.clone());
            self.frequency_per_column[index].add(value);
        }

        Ok(())
    }

    pub fn column(&self, index: usize) -> &[T] {
        &self.columns_to_rows[index]
    }

    pub fn rows(&self) -> Vec<Vec<&T>> {
        let mut transposed: Vec<Vec<&T>> = vec![Vec::new(); self.columns_to_rows[0].len()];

        for record in &self.columns_to_rows {
            for (index, element) in record.iter().enumerate() {
                transposed[index].push(element);
            }
        }

        transposed
    }

    pub fn column_frequency(&self, index: usize) -> &FrequencyMap<T> {
        &self.frequency_per_column[index]
    }

    pub fn column_len(&self) -> usize {
        self.columns_to_rows.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const SAMPLE_INPUT: &'static str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

    #[test]
    fn column_values() {
        let sample = SAMPLE_INPUT.split("\n").map(|s| s.to_string());
        let columns: ColumnValues<i32> = columnize(sample.collect());

        assert_eq!(vec![0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0], columns.column(0));
        assert_eq!(vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1], columns.column(1));
    }

    #[test]
    fn rows() {
        let sample = SAMPLE_INPUT.split("\n").map(|s| s.to_string());
        let columns: ColumnValues<i32> = columnize(sample.collect());

        let rows = columns.rows();
        assert_eq!(
            vec![0, 0, 1, 0, 0],
            rows[0].iter().map(|f| **f).collect_vec()
        );
    }

    #[test]
    fn part1_sample() {
        let sample = SAMPLE_INPUT.split("\n").map(|s| s.to_string());
        let columns: ColumnValues<i32> = columnize(sample.collect());

        assert_eq!(198, part1(&columns));
    }

    #[test]
    fn part2_sample() {
        let sample = SAMPLE_INPUT.split("\n").map(|s| s.to_string());
        let columns: ColumnValues<i32> = columnize(sample.collect());

        assert_eq!(230, part2(&columns));
    }
}

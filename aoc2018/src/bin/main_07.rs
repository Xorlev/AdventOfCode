use failure::*;
use lazy_static::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use util::aoc::*;

lazy_static! {
    static ref RE_STEP: Regex =
        Regex::new("Step (.+) must be finished before step (.+) can begin.").unwrap();
}

type Step = char;

fn main() -> Result<(), Box<std::error::Error>> {
    let lines: Vec<String> = input::read(7)?;

    let mut dependency_graph: HashMap<Step, BTreeSet<Step>> = HashMap::new();
    let mut all_nodes: BTreeSet<Step> = BTreeSet::new();
    for edge in lines.iter().map(parse) {
        let edge = edge?;

        all_nodes.insert(edge.0);
        all_nodes.insert(edge.1);

        // Creates a "step depends on" graph.
        dependency_graph.entry(edge.1).or_default().insert(edge.0);
    }

    result("Part 1", || part1(&all_nodes, &dependency_graph));
    result("Part 2", || part2(&all_nodes, &dependency_graph));

    Ok(())
}

fn parse(line: &String) -> Result<(Step, Step), Error> {
    if let Some(captures) = RE_STEP.captures(line.as_str()) {
        Ok((
            captures[1]
                .chars()
                .next()
                .ok_or(format_err!("Missing capture."))?,
            captures[2]
                .chars()
                .next()
                .ok_or(format_err!("Missing capture."))?,
        ))
    } else {
        Err(format_err!("Failed to parse line: {}", line))
    }
}

fn part1(all_nodes: &BTreeSet<Step>, dependency_graph: &HashMap<Step, BTreeSet<Step>>) -> String {
    let mut stack: Vec<Step> = Vec::new();
    let mut complete: HashSet<Step> = HashSet::new();

    loop {
        if all_nodes.len() == complete.len() {
            break;
        }

        // Check all steps to see which ones can run.
        for step in all_nodes {
            // skip it if we've seen it.
            if complete.contains(step) {
                continue;
            }

            // A step can run if all of its parents are seen (or it has no parents).
            if !dependency_graph.contains_key(step)
                || dependency_graph
                    .get(step)
                    .unwrap()
                    .iter()
                    .all(|s| complete.contains(s))
            {
                stack.push(*step);
                complete.insert(*step);
                break;
            }
        }
    }

    stack.iter().collect()
}

fn part2(all_nodes: &BTreeSet<Step>, dependency_graph: &HashMap<Step, BTreeSet<Step>>) -> u32 {
    let mut stack: Vec<Step> = Vec::new();
    let mut complete: HashSet<Step> = HashSet::new();
    let mut worker_pool = WorkerPool::new(5);

    loop {
        if all_nodes.len() == complete.len() {
            break;
        }

        // Check all steps to see which ones can run.
        for step in all_nodes {
            // Skip it if we've seen it.
            if complete.contains(&step) {
                continue;
            }

            // A step can run if all of its parents are seen (or it has no parents).
            if !dependency_graph.contains_key(step)
                || dependency_graph
                    .get(step)
                    .unwrap()
                    .iter()
                    .all(|s| complete.contains(s))
            {
                if !worker_pool.running(step) {
                    worker_pool.start(*step);
                }
            }
        }

        // For each completed job, record the completion.
        worker_pool.step().iter().for_each(|&s| {
            stack.push(s);
            complete.insert(s);
        });
    }

    worker_pool.time()
}

#[derive(Debug)]
struct Task {
    step: Step,
}

impl Task {
    fn new(step: Step) -> Task {
        Task { step }
    }

    fn execution_time(&self) -> u32 {
        let code: u32 = u32::from(self.step);

        // 'A' = 65, but + 60s per task and calibrating 'A' = 1
        code - 4
    }
}

#[derive(Default)]
struct WorkerPool {
    available: Vec<Worker>,
    busy: HashSet<Worker>,
    in_progress: HashSet<Step>,
    time: u32,
}

impl WorkerPool {
    fn new(workers: u32) -> WorkerPool {
        WorkerPool {
            available: (0..=workers).map(|_| Worker::default()).collect(),
            ..Default::default()
        }
    }

    fn start(&mut self, task: Step) -> bool {
        if let Some(mut worker) = self.available.pop() {
            self.in_progress.insert(task);
            worker.run(self.time, Task::new(task));
            self.busy.insert(worker);
            true
        } else {
            false
        }
    }

    fn running(&self, task: &Step) -> bool {
        self.in_progress.contains(task)
    }

    // Returns tasks completed on this step.
    fn step(&mut self) -> Vec<Step> {
        self.time += 1;

        // There's probably a better way to deal with this.
        let mut pending_tasks = HashSet::new();
        std::mem::swap(&mut self.busy, &mut pending_tasks);

        let mut tasks = Vec::new();
        for mut c in pending_tasks.into_iter() {
            if c.complete(self.time) {
                self.in_progress.remove(&c.task.unwrap());
                tasks.push(c.task.unwrap());

                c.reset();
                self.available.push(c);
            } else {
                self.busy.insert(c);
            }
        }

        tasks
    }

    fn time(&self) -> u32 {
        self.time
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum CompletionTime {
    Idle,
    CompleteAt(u32),
}

impl Default for CompletionTime {
    fn default() -> Self {
        CompletionTime::Idle
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Default)]
struct Worker {
    task: Option<Step>,
    completion_time: CompletionTime,
}

impl Worker {
    fn run(&mut self, time: u32, task: Task) {
        self.task = Some(task.step);
        self.completion_time = CompletionTime::CompleteAt(time + task.execution_time());
    }

    fn complete(&self, time: u32) -> bool {
        match self.completion_time {
            CompletionTime::Idle => false,
            CompletionTime::CompleteAt(completion_time) => completion_time == time,
        }
    }

    fn reset(&mut self) {
        self.task = None;
        self.completion_time = CompletionTime::Idle;
    }
}

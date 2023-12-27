use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Target {
    Accept,
    Reject,
    Workflow(String)
}

impl FromStr for Target {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Target::Accept),
            "R" => Ok(Target::Reject),
            s => Ok(Self::Workflow(s.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
struct Condition {
    variable: char,
    operation: char,
    amount: usize
}

impl Condition {
    fn matches(self: &Self, engine: &Engine) -> bool {
        match self {
            Condition { variable: 'x', operation: '>', amount } => engine.x > *amount,
            Condition { variable: 'x', operation: '<', amount } => engine.x < *amount,
            Condition { variable: 'm', operation: '>', amount } => engine.m > *amount,
            Condition { variable: 'm', operation: '<', amount } => engine.m < *amount,
            Condition { variable: 'a', operation: '>', amount } => engine.a > *amount,
            Condition { variable: 'a', operation: '<', amount } => engine.a < *amount,
            Condition { variable: 's', operation: '>', amount } => engine.s > *amount,
            Condition { variable: 's', operation: '<', amount } => engine.s < *amount,
            _ => panic!("Failure to apply condition {self:?} to engine {engine:?}")
        }
    }
}

impl FromStr for Condition {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let variable = chars.next().unwrap();
        let operation = chars.next().unwrap();
        let amount = chars.collect::<String>().parse().unwrap();

        Ok(Condition { variable, operation, amount })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRuleError;

#[derive(Debug, Clone)]
struct Rule {
    condition: Option<Condition>,
    target: Target
}

impl Rule {
    fn matches(self: &Self, engine: &Engine) -> bool {
        if let Some(condition) = &self.condition {
            return condition.matches(engine);
        };
        true
    }
}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(':') {
            return Ok(Rule{ condition: None, target: s.parse()? })
        }

        let (condition, target) = s.split_once(':').unwrap();

        Ok(Rule{ condition: Some(condition.parse()?), target: target.parse()? })
    }
}

#[derive(Debug)]
struct Engine {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

impl FromStr for Engine {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inside = s.strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',');
        let x = (&inside.next().unwrap()[2..]).parse().unwrap();
        let m = (&inside.next().unwrap()[2..]).parse().unwrap();
        let a = (&inside.next().unwrap()[2..]).parse().unwrap();
        let s = (&inside.next().unwrap()[2..]).parse().unwrap();
        Ok(Self { x, m, a, s })
    }
}

#[derive(Debug, Clone)]
struct EngineRange {
    x: Vec<(usize, usize)>,
    m: Vec<(usize, usize)>,
    a: Vec<(usize, usize)>,
    s: Vec<(usize, usize)>,
}

fn main() {
    let mut stdin_iter = std::io::stdin().lines();

    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();

    loop {
        let line = stdin_iter.next().unwrap().unwrap();

        if line.is_empty() {
            // We finished parsing workflows
            break
        }

        let (name, rules): (&str, &str) = line.split_once('{').unwrap();

        let rules: Vec<Rule> = rules.trim_end_matches('}').split(',').map(|rule| (*rule).parse().unwrap()).collect();

        workflows.insert(name.to_owned(), rules);
    }

    let mut engines: Vec<Engine> = Vec::new();

    for line in stdin_iter {
        let line = line.unwrap();

        let engine: Engine = line.parse().unwrap();
        engines.push(engine);
    }

    let mut part_1_result = 0;

    for engine in engines {
        // println!("Looking at engine {engine:?}");
        let mut workflow = "in".to_string();

        'outer: loop {
            for rule in workflows.get(&workflow).unwrap() {
                // println!("Looking a rule {rule:?}");
                if rule.matches(&engine) {
                    match &rule.target {
                        Target::Accept => { part_1_result += engine.x + engine.m + engine.a + engine.s; break 'outer; },
                        Target::Reject => { break 'outer; },
                        Target::Workflow(name) => { workflow = name.clone(); break; }
                    }
                }
            }
        }
    }

    println!("Part 1: {part_1_result}");

    // --- Part 2

    let engine = EngineRange { x: vec![(1, 4000)], m: vec![(1, 4000)], a: vec![(1, 4000)], s: vec![(1, 4000)] };

    let part_2_result = count_accepted_engines(engine, workflows.get(&"in".to_string()).unwrap(), &workflows);

    println!("Part 2: {part_2_result}");
}

fn count_possibilities(engine: &EngineRange) -> u64 {
    let possibilities = engine.x.iter().map(|(min, max)| (max - min + 1) as u64).sum::<u64>() *
    engine.m.iter().map(|(min, max)| (max - min + 1) as u64).sum::<u64>() *
    engine.a.iter().map(|(min, max)| (max - min + 1) as u64).sum::<u64>() *
    engine.s.iter().map(|(min, max)| (max - min + 1) as u64).sum::<u64>();

    // println!("Possibilities for {engine:?}: {possibilities}");

    possibilities
}

fn compute_gt_ranges(ranges: &Vec<(usize, usize)>, pivot: usize) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut upper: Vec<(usize, usize)> = Vec::new();
    let mut lower: Vec<(usize, usize)> = Vec::new();

    for range in ranges {
        match range {
            (min, max) if min > &pivot => { upper.push((*min, *max)); },
            (min, max) if max <= &pivot => { lower.push((*min, *max)); },
            (min, max) => { lower.push((*min, pivot)); upper.push((pivot + 1, *max)); },
        }
    }

    (lower, upper)
}

fn compute_lt_ranges(ranges: &Vec<(usize, usize)>, pivot: usize) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut upper: Vec<(usize, usize)> = Vec::new();
    let mut lower: Vec<(usize, usize)> = Vec::new();

    for range in ranges {
        match range {
            (min, max) if min >= &pivot => { upper.push((*min, *max)); },
            (min, max) if max < &pivot => { lower.push((*min, *max)); },
            (min, max) => { lower.push((*min, pivot - 1)); upper.push((pivot, *max)); },
        }
    }

    (lower, upper)
}

fn count_accepted_engines(engine: EngineRange, rules: &Vec<Rule>, workflows: &HashMap<String, Vec<Rule>>) -> u64 {
    // println!("Counting {engine:?}, {rules:?}");
    let rule = &rules[0];
    if let Some(condition) = &rule.condition {
        let mut matched_engine = engine.clone();
        let mut remaining_engine = engine.clone();

        let mut result = 0;

        match condition {
            Condition { variable: 'x', operation: '>', amount } => {
                let (lower_ranges, upper_ranges) = compute_gt_ranges(&engine.x, *amount);
                matched_engine.x = upper_ranges;
                remaining_engine.x = lower_ranges;
            },
            Condition { variable: 'x', operation: '<', amount } => {
                let (lower_ranges, upper_ranges) = compute_lt_ranges(&engine.x, *amount);
                matched_engine.x = lower_ranges;
                remaining_engine.x = upper_ranges;
            },
            Condition { variable: 'm', operation: '>', amount } => {
                let (lower_ranges, upper_ranges) = compute_gt_ranges(&engine.m, *amount);
                matched_engine.m = upper_ranges;
                remaining_engine.m = lower_ranges;
            },
            Condition { variable: 'm', operation: '<', amount } => {
                let (lower_ranges, upper_ranges) = compute_lt_ranges(&engine.m, *amount);
                matched_engine.m = lower_ranges;
                remaining_engine.m = upper_ranges;
            },
            Condition { variable: 'a', operation: '>', amount } => {
                let (lower_ranges, upper_ranges) = compute_gt_ranges(&engine.a, *amount);
                matched_engine.a = upper_ranges;
                remaining_engine.a = lower_ranges;
            },
            Condition { variable: 'a', operation: '<', amount } => {
                let (lower_ranges, upper_ranges) = compute_lt_ranges(&engine.a, *amount);
                matched_engine.a = lower_ranges;
                remaining_engine.a = upper_ranges;
            },
            Condition { variable: 's', operation: '>', amount } => {
                let (lower_ranges, upper_ranges) = compute_gt_ranges(&engine.s, *amount);
                matched_engine.s = upper_ranges;
                remaining_engine.s = lower_ranges;
            },
            Condition { variable: 's', operation: '<', amount } => {
                let (lower_ranges, upper_ranges) = compute_lt_ranges(&engine.s, *amount);
                matched_engine.s = lower_ranges;
                remaining_engine.s = upper_ranges;
            },
            _ => panic!("Failure to apply condition {condition:?} to engine {engine:?}")
        }

        result += match &rule.target {
            Target::Accept => count_possibilities(&matched_engine),
            Target::Reject => 0,
            Target::Workflow(name) => count_accepted_engines(matched_engine, &workflows.get(name).unwrap(), workflows)
        };
        result += count_accepted_engines(remaining_engine, &rules[1..].to_vec(), workflows);

        return result;
    } else {
        match &rule.target {
            Target::Accept => count_possibilities(&engine),
            Target::Reject => 0,
            Target::Workflow(name) => count_accepted_engines(engine, &workflows.get(name).unwrap(), workflows)
        }
    }
}

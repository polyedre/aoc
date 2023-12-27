use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
        println!("Looking at engine {engine:?}");
        let mut workflow = "in".to_string();

        'outer: loop {
            for rule in workflows.get(&workflow).unwrap() {
                println!("Looking a rule {rule:?}");
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
}

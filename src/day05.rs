use std::collections::{HashMap, HashSet};

type Rules = HashMap<u32, Vec<u32>>;

fn add_page_rule(rules: &mut Rules, page: u32, dependency: u32) {
    rules.entry(page).or_default().push(dependency);
}

fn valid_update(update: &[u32], rules: &Rules) -> bool {
    let mut printed: HashSet<u32> = HashSet::new();

    for page in update {
        let Some(rule) = rules.get(page) else {
            printed.insert(*page);
            continue;
        };

        for dependency in rule {
            if update.contains(dependency) && !printed.contains(dependency) {
                return false;
            }
        }

        printed.insert(*page);
    }

    true
}

pub fn part_1(input: &str) -> String {
    let mut rules: Rules = HashMap::new();

    let lines = input.lines();

    let mut parse_rules = true;
    let mut sum = 0;

    for line in lines {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }

        if parse_rules {
            let (dependency, page) = line
                .split_once('|')
                .map(|(dependency, page)| (dependency.parse().unwrap(), page.parse().unwrap()))
                .unwrap();

            add_page_rule(&mut rules, page, dependency);
            continue;
        }

        let update: Vec<u32> = line.split(',').map(|item| item.parse().unwrap()).collect();

        if valid_update(&update, &rules) {
            sum += update[update.len() / 2];
        }
    }

    sum.to_string()
}

fn print_with_rules(page: &u32, update: &[u32], rules: &Rules, print: &mut Vec<u32>) {
    let Some(rule) = rules.get(page) else {
        if !print.contains(page) {
            print.push(*page);
        }
        return;
    };

    for dependency in rule {
        if update.contains(dependency) && !print.contains(dependency) {
            print_with_rules(dependency, update, rules, print);
        }
    }

    if !print.contains(page) {
        print.push(*page);
    }
}

fn re_order(update: &[u32], rules: &Rules) -> Vec<u32> {
    let mut print: Vec<u32> = vec![];

    for page in update {
        if !print.contains(page) {
            print_with_rules(page, update, rules, &mut print);
        }
    }

    assert!(print.len() == update.len());
    print
}

pub fn part_2(input: &str) -> String {
    let mut rules: Rules = HashMap::new();

    let lines = input.lines();

    let mut parse_rules = true;
    let mut sum = 0;

    for line in lines {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }

        if parse_rules {
            let (dependency, page) = line
                .split_once('|')
                .map(|(dependency, page)| (dependency.parse().unwrap(), page.parse().unwrap()))
                .unwrap();

            add_page_rule(&mut rules, page, dependency);
            continue;
        }

        let update: Vec<u32> = line.split(',').map(|item| item.parse().unwrap()).collect();

        if !valid_update(&update, &rules) {
            let ordered = re_order(&update, &rules);
            sum += ordered[ordered.len() / 2];
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod test {
    use crate::{
        day05::{part_1, part_2},
        input, output,
    };

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(input::example::DAY_05), output::example::DAY_05_1);
    }
    #[test]
    fn part_1_real() {
        assert_eq!(part_1(input::DAY_05), output::DAY_05_1)
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_2(input::example::DAY_05), output::example::DAY_05_2);
    }
    #[test]
    fn part_2_real() {
        assert_eq!(part_2(input::DAY_05), output::DAY_05_2)
    }
}

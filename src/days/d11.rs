use std::fs;

pub fn d11() -> (String, String) {
    let read = fs::read_to_string("inputs/d11").unwrap();
    let lines = read.split("\n").collect::<Vec<&str>>();

    let mut monkey_texts = lines.split(|x| x.len() == 0).collect::<Vec<&[&str]>>();
    monkey_texts.remove(monkey_texts.len()-1);
    
    let mut monkeys = vec![];

    // Parse input
    for monkey_text in monkey_texts.iter() {
        let mut split0 = monkey_text[1].trim().split(' ').collect::<Vec<&str>>();
        split0.drain(0..2);

        let items = split0.iter().map(|x|(x.replace(',', "")).parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let mut split1 = monkey_text[2].trim().split(' ').collect::<Vec<&str>>();
        split1.drain(0..4);

        let operation: Operation = match split1[0] {
            "*" => match split1[1] {
                "old" => Operation::MultiplySelf(),
                a => Operation::Multiply(a.parse().unwrap())
            }
            "+" => Operation::Add(split1[1].parse().unwrap()),
            _ => unimplemented!()
        };

        let test: usize = monkey_text[3].trim().split(' ').last().unwrap().parse().unwrap();

        let res1: usize = monkey_text[4].trim().split(' ').last().unwrap().parse().unwrap();
        let res2: usize = monkey_text[5].trim().split(' ').last().unwrap().parse().unwrap();

        let results = (res1, res2);

        monkeys.push(Monkey {
            items: items.clone(),
            operation,
            test,
            results,
            inspect_count: 0
        });
    }

    let mut monkeys2 = monkeys.clone();

    for _ in 0..20 {
        round1(&mut monkeys);
    }
    
    for _ in 0..10000 {
        round2(&mut monkeys2);
    }

    monkeys.sort_by(|a,b| a.inspect_count.cmp(&b.inspect_count));
    monkeys2.sort_by(|a,b| a.inspect_count.cmp(&b.inspect_count));

    let part1 = monkeys[monkeys.len()-1].inspect_count * monkeys[monkeys.len()-2].inspect_count;
    let part2 = monkeys2[monkeys2.len()-1].inspect_count * monkeys2[monkeys2.len()-2].inspect_count;

    (part1.to_string(), part2.to_string())
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: usize,
    results: (usize, usize),
    inspect_count: usize
}

#[derive(Debug, Clone)]
enum Operation {
    Add(i64),
    Multiply(i64),
    MultiplySelf()
}

fn round1(monkeys: &mut Vec<Monkey>) {
    // Run one round
    for m in 0..monkeys.len() {
        //let mut monkey = &mut monkeys[m];

        for i in 0..monkeys[m].items.len() {
            let mut value = monkeys[m].items[i];
            // Inspect item
            value = match monkeys[m].operation {
                Operation::Add(a) => value + a,
                Operation::Multiply(a) => value * a,
                Operation::MultiplySelf() => value * value
            };
            monkeys[m].inspect_count += 1;

            // Bored with item
            value = (value as f32 / 3.0).floor() as i64;
            //value = value % 9699690; // Keep worry low

            // Test item
            let target = match (value % monkeys[m].test as i64) == 0 {
                true => monkeys[m].results.0,
                false => monkeys[m].results.1
            };
            monkeys[target].items.push(value);
        }
        monkeys[m].items.clear();
    }
}

fn round2(monkeys: &mut Vec<Monkey>) {
    // Run one round
    for m in 0..monkeys.len() {
        //let mut monkey = &mut monkeys[m];

        for i in 0..monkeys[m].items.len() {
            let mut value = monkeys[m].items[i];
            // Inspect item
            value = match monkeys[m].operation {
                Operation::Add(a) => value + a,
                Operation::Multiply(a) => value * a,
                Operation::MultiplySelf() => value * value
            };
            monkeys[m].inspect_count += 1;

            // Bored with item
            //value = (value as f32 / 3.0).floor() as i64;
            value = value % 9699690; // Keep worry low

            // Test item
            let target = match (value % monkeys[m].test as i64) == 0 {
                true => monkeys[m].results.0,
                false => monkeys[m].results.1
            };
            monkeys[target].items.push(value);
        }
        monkeys[m].items.clear();
    }
}

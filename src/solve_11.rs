use log::debug;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

type Score = i64;

struct Monkey {
    items: VecDeque<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> bool>,
    throw: Box<dyn Fn(bool) -> usize>,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("throw_if_true", &self.throw.deref()(true))
            .field("throw_if_false", &self.throw.deref()(false))
            .finish()
    }
}

pub fn solve_day_11_challenge_1() -> anyhow::Result<Score> {
    let monkey_0 = Monkey {
        items: VecDeque::from([64, 89, 65, 95]),
        operation: Box::new(|x| x * 7),
        test: Box::new(|x| (x % 3) == 0),
        throw: Box::new(|b| if b { 4 } else { 1 }),
    };
    let monkey_1 = Monkey {
        items: VecDeque::from([76, 66, 74, 87, 70, 56, 51, 66]),
        operation: Box::new(|x| x + 5),
        test: Box::new(|x| (x % 13) == 0),
        throw: Box::new(|b| if b { 7 } else { 3 }),
    };
    let monkey_2 = Monkey {
        items: VecDeque::from([91, 60, 63]),
        operation: Box::new(|x| x * x),
        test: Box::new(|x| (x % 2) == 0),
        throw: Box::new(|b| if b { 6 } else { 5 }),
    };
    let monkey_3 = Monkey {
        items: VecDeque::from([92, 61, 79, 97, 79]),
        operation: Box::new(|x| x + 6),
        test: Box::new(|x| (x % 11) == 0),
        throw: Box::new(|b| if b { 2 } else { 6 }),
    };
    let monkey_4 = Monkey {
        items: VecDeque::from([93, 54]),
        operation: Box::new(|x| x * 11),
        test: Box::new(|x| (x % 5) == 0),
        throw: Box::new(|b| if b { 1 } else { 7 }),
    };
    let monkey_5 = Monkey {
        items: VecDeque::from([60, 79, 92, 69, 88, 82, 70]),
        operation: Box::new(|x| x + 8),
        test: Box::new(|x| (x % 17) == 0),
        throw: Box::new(|b| if b { 4 } else { 0 }),
    };
    let monkey_6 = Monkey {
        items: VecDeque::from([64, 57, 73, 89, 55, 53]),
        operation: Box::new(|x| x + 1),
        test: Box::new(|x| (x % 19) == 0),
        throw: Box::new(|b| if b { 0 } else { 5 }),
    };
    let monkey_7 = Monkey {
        items: VecDeque::from([62]),
        operation: Box::new(|x| x + 4),
        test: Box::new(|x| (x % 7) == 0),
        throw: Box::new(|b| if b { 3 } else { 2 }),
    };
    let mut monkeys: Vec<RefCell<Monkey>> = vec![
        RefCell::new(monkey_0),
        RefCell::new(monkey_1),
        RefCell::new(monkey_2),
        RefCell::new(monkey_3),
        RefCell::new(monkey_4),
        RefCell::new(monkey_5),
        RefCell::new(monkey_6),
        RefCell::new(monkey_7),
    ];
    let mut inspections = [0; 8];
    for round in 0..20 {
        for monkey_id in 0..8 {
            let mut new_worries: Vec<i64> = vec![];
            for item in monkeys[monkey_id].get_mut().items.clone() {
                inspections[monkey_id] += 1;
                let new_worry: i64 = monkeys[monkey_id].get_mut().operation.deref()(item);
                let new_worry = new_worry / 3;
                new_worries.push(new_worry);
            }
            monkeys[monkey_id].get_mut().items.clear();
            for worry in new_worries {
                let t: bool = monkeys[monkey_id].get_mut().test.deref()(worry);
                let monkey_throw: usize = monkeys[monkey_id].get_mut().throw.deref()(t);
                monkeys[monkey_throw].get_mut().items.push_back(worry);
            }
        }
        debug!("round: {}", round);
        for monkey in monkeys.iter_mut() {
            debug!("monkey: {:?}", monkey.get_mut().items);
        }
    }
    inspections.sort();
    debug!("{:?}", inspections);
    Ok(inspections[6] * inspections[7])
}

pub fn solve_day_11_challenge_2() -> anyhow::Result<Score> {
    let monkey_0 = Monkey {
        items: VecDeque::from([64, 89, 65, 95]),
        operation: Box::new(|x| x * 7),
        test: Box::new(|x| (x % 3) == 0),
        throw: Box::new(|b| if b { 4 } else { 1 }),
    };
    let monkey_1 = Monkey {
        items: VecDeque::from([76, 66, 74, 87, 70, 56, 51, 66]),
        operation: Box::new(|x| x + 5),
        test: Box::new(|x| (x % 13) == 0),
        throw: Box::new(|b| if b { 7 } else { 3 }),
    };
    let monkey_2 = Monkey {
        items: VecDeque::from([91, 60, 63]),
        operation: Box::new(|x| x * x),
        test: Box::new(|x| (x % 2) == 0),
        throw: Box::new(|b| if b { 6 } else { 5 }),
    };
    let monkey_3 = Monkey {
        items: VecDeque::from([92, 61, 79, 97, 79]),
        operation: Box::new(|x| x + 6),
        test: Box::new(|x| (x % 11) == 0),
        throw: Box::new(|b| if b { 2 } else { 6 }),
    };
    let monkey_4 = Monkey {
        items: VecDeque::from([93, 54]),
        operation: Box::new(|x| x * 11),
        test: Box::new(|x| (x % 5) == 0),
        throw: Box::new(|b| if b { 1 } else { 7 }),
    };
    let monkey_5 = Monkey {
        items: VecDeque::from([60, 79, 92, 69, 88, 82, 70]),
        operation: Box::new(|x| x + 8),
        test: Box::new(|x| (x % 17) == 0),
        throw: Box::new(|b| if b { 4 } else { 0 }),
    };
    let monkey_6 = Monkey {
        items: VecDeque::from([64, 57, 73, 89, 55, 53]),
        operation: Box::new(|x| x + 1),
        test: Box::new(|x| (x % 19) == 0),
        throw: Box::new(|b| if b { 0 } else { 5 }),
    };
    let monkey_7 = Monkey {
        items: VecDeque::from([62]),
        operation: Box::new(|x| x + 4),
        test: Box::new(|x| (x % 7) == 0),
        throw: Box::new(|b| if b { 3 } else { 2 }),
    };
    let mut monkeys: Vec<RefCell<Monkey>> = vec![
        RefCell::new(monkey_0),
        RefCell::new(monkey_1),
        RefCell::new(monkey_2),
        RefCell::new(monkey_3),
        RefCell::new(monkey_4),
        RefCell::new(monkey_5),
        RefCell::new(monkey_6),
        RefCell::new(monkey_7),
    ];
    let mut inspections = [0; 8];
    for round in 0..10_000 {
        for monkey_id in 0..8 {
            let mut new_worries: Vec<i64> = vec![];
            for item in monkeys[monkey_id].get_mut().items.clone() {
                inspections[monkey_id] += 1;
                let new_worry: i64 = monkeys[monkey_id].get_mut().operation.deref()(item);
                let new_worry = new_worry % (2 * 3 * 5 * 7 * 9 * 11 * 13 * 15 * 17 * 19);
                new_worries.push(new_worry);
            }
            monkeys[monkey_id].get_mut().items.clear();
            for worry in new_worries {
                let t: bool = monkeys[monkey_id].get_mut().test.deref()(worry);
                let monkey_throw: usize = monkeys[monkey_id].get_mut().throw.deref()(t);
                monkeys[monkey_throw].get_mut().items.push_back(worry);
            }
        }
        debug!("round: {}", round);
        for monkey in monkeys.iter_mut() {
            debug!("monkey: {:?}", monkey.get_mut().items);
        }
    }
    inspections.sort();
    debug!("{:?}", inspections);
    Ok(inspections[6] * inspections[7])
}

fn main() {
    let mut monkeys = vec![
        Monkey {
            // 0
            inventory: Inventory::new("66, 79".to_string()),
            operation: Box::new(|x| x * 11),
            test: Test {
                value: 7,
                true_to: 6,
                false_to: 7,
            },
            inspections: 0,
        },
        Monkey {
            // 1
            inventory: Inventory::new("84, 94, 94, 81, 98, 75".to_string()),
            operation: Box::new(|x| x * 17),
            test: Test {
                value: 13,
                true_to: 5,
                false_to: 2,
            },
            inspections: 0,
        },
        Monkey {
            // 2
            inventory: Inventory::new("85, 79, 59, 64, 79, 95, 67".to_string()),
            operation: Box::new(|x| x + 8),
            test: Test {
                value: 5,
                true_to: 4,
                false_to: 5,
            },
            inspections: 0,
        },
        Monkey {
            // 3
            inventory: Inventory::new("70".to_string()),
            operation: Box::new(|x| x + 3),
            test: Test {
                value: 19,
                true_to: 6,
                false_to: 0,
            },
            inspections: 0,
        },
        Monkey {
            // 4
            inventory: Inventory::new("57, 69, 78, 78".to_string()),
            operation: Box::new(|x| x + 4),
            test: Test {
                value: 2,
                true_to: 0,
                false_to: 3,
            },
            inspections: 0,
        },
        Monkey {
            // 5
            inventory: Inventory::new("65, 92, 60, 74, 72".to_string()),
            operation: Box::new(|x| x + 7),
            test: Test {
                value: 11,
                true_to: 3,
                false_to: 4,
            },
            inspections: 0,
        },
        Monkey {
            // 6
            inventory: Inventory::new("77, 91, 91".to_string()),
            operation: Box::new(|x| x * x),
            test: Test {
                value: 17,
                true_to: 1,
                false_to: 7,
            },
            inspections: 0,
        },
        Monkey {
            // 7
            inventory: Inventory::new("76, 58, 57, 55, 67, 77, 54, 99".to_string()),
            operation: Box::new(|x| x + 6),
            test: Test {
                value: 3,
                true_to: 2,
                false_to: 1,
            },
            inspections: 0,
        },
    ];

    let total_divisor: u128 = monkeys.iter().map(|x| x.test.value).product();

    for _ in 0..10_000 {
        round(&mut monkeys, total_divisor);
    }

    monkey_business(monkeys)
}

fn round(monkeys: &mut Vec<Monkey>, total_divisor: u128) {
    for i in 0..monkeys.len() {
        while monkeys[i].inventory.items.len() > 0 {
            let (mut item, to) = monkeys[i].inspect_and_test();
            item.worry_level = item.worry_level % total_divisor;
            throw(item, &mut monkeys[to])
        }
    }
}

fn throw(item: Item, to: &mut Monkey) {
    to.inventory.items.push(item)
}

fn monkey_business(monkeys: Vec<Monkey>) {
    let mut inspections: Vec<u128> = monkeys.iter().map(|x| x.inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));

    let monkey_business_value: u128 = inspections[..2].iter().product();

    println!("Monkey Business: {}", monkey_business_value);

    println!("Values: {inspections:?}")
}

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new(itemstr: String) -> Inventory {
        let items: Vec<Item> = itemstr
            .split(", ")
            .map(|x| Item {
                worry_level: x.parse().expect("not an int"),
            })
            .collect();
        Inventory { items: items }
    }
}

struct Item {
    worry_level: u128,
}

struct Test {
    value: u128,
    true_to: usize,
    false_to: usize,
}

struct Monkey {
    inventory: Inventory,
    operation: Box<dyn Fn(u128) -> u128>,
    test: Test,
    inspections: u128,
}

impl Monkey {
    fn inspect(&self, item: &mut Item) {
        // item.worry_level = item.worry_level % self.test.value;
        item.worry_level = (self.operation)(item.worry_level);
    }

    fn test(&self, item: &mut Item) -> bool {
        item.worry_level % self.test.value == 0
    }

    fn inspect_and_test(&mut self) -> (Item, usize) {
        let mut item = self.inventory.items.remove(0);
        let to: usize;

        self.inspect(&mut item);
        if self.test(&mut item) {
            to = self.test.true_to;
        } else {
            to = self.test.false_to;
        }
        self.inspections += 1;

        (item, to)
    }
}

use std::{collections::HashSet, fs};

fn main() {
    let moves = fs::read_to_string("input.txt".to_string()).expect("unable to read");

    let mut rope = Rope::build(10);

    for mv in moves.lines() {
        let mut parts = mv.split(" ");
        let dir = parts.next().unwrap();
        let steps: i32 = parts.next().unwrap().parse().unwrap();

        rope.update(dir, steps);
    }

    println!("Number of tail positions: {}", rope.tail_positions.len())
}
#[derive(Clone, Default, Debug)]

struct Knot {
    x: i32,
    y: i32,
}
#[derive(Default)]
struct Rope {
    knots: Vec<Knot>,
    tail_positions: HashSet<(i32, i32)>,
}

impl Rope {
    fn build(size: i32) -> Rope {
        let mut knots = Vec::new();
        for _ in 0..size {
            knots.push(Knot::default())
        }
        Rope {
            knots: knots,
            ..Default::default()
        }
    }

    fn update(&mut self, dir: &str, steps: i32) {
        for _ in 0..steps {
            self.update_head(dir);
            self.update_rope();
        }
    }

    fn update_head(&mut self, dir: &str) {
        match dir {
            "L" => self.knots[0].x -= 1,
            "R" => self.knots[0].x += 1,
            "U" => self.knots[0].y += 1,
            "D" => self.knots[0].y -= 1,
            _ => panic!("Unkown direction {}", &dir),
        }
    }

    fn update_rope(&mut self) {
        for i in 1..self.knots.len() {
            let prev = self.knots[i - 1].clone();
            let cur = &mut self.knots[i];

            let dx = prev.x - cur.x;
            let dy = prev.y - cur.y;

            let change = match (dx, dy) {
                (2, 2) => (1, 1),
                (2, -2) => (1, -1),
                (-2, 2) => (-1, 1),
                (-2, -2) => (-1, -1),
                (2, dy) => (1, dy),
                (-2, dy) => (-1, dy),
                (dx, 2) => (dx, 1),
                (dx, -2) => (dx, -1),
                _ => (0, 0),
            };

            cur.x += change.0;
            cur.y += change.1;
        }

        let tail = self.knots.last().clone().expect("Tail not found");

        self.tail_positions.insert((tail.x, tail.y));
    }
}

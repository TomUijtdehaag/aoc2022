use std::fs;

fn main() {
    let ops = fs::read_to_string("input.txt".to_string()).expect("unable to read");

    let mut program = Program::default();
    program.states.push(State { cycle: 1, x: 1 });

    for opl in ops.lines() {
        let ov = opl.split(" ").collect::<Vec<&str>>();

        match ov[0] {
            "noop" => program.noop(),
            "addx" => program.addx(ov[1].parse().unwrap()),
            _ => (),
        }
    }

    let signal_strength = program.signal_strength();

    println!("Signal strength: {}", signal_strength);

    program.draw_screen();
}

#[derive(Debug, Default, Clone)]
struct State {
    cycle: u32,
    x: i32,
}

#[derive(Debug, Default)]
struct Program {
    states: Vec<State>,
}

impl Program {
    fn noop(&mut self) {
        let mut new_state = self.states.last().cloned().expect("List empty");
        new_state.cycle += 1;

        self.states.push(new_state)
    }

    fn addx(&mut self, v: i32) {
        self.noop();

        let mut new_state = self.states.last().cloned().expect("List empty");
        new_state.cycle += 1;
        new_state.x += v;

        self.states.push(new_state);
    }

    fn signal_strength(&self) -> i32 {
        let mut total_strength = 0;
        for c in (20..self.states.len()).step_by(40) {
            let signal_strength = self.states[c - 1].cycle as i32 * self.states[c - 1].x;
            println!("{:?}: {}", &self.states[c - 1], signal_strength);
            total_strength += signal_strength;
        }

        total_strength
    }

    fn draw_screen(&self) {
        for row in self.states[..self.states.len() - 1].chunks(40) {
            let mut screen_row = Vec::new();
            for (col, state) in row.iter().enumerate() {
                let window = (col as i32)..(col as i32 + 3);
                if window.contains(&(state.x + 1)) {
                    screen_row.push("#")
                } else {
                    screen_row.push(".")
                }
            }
            let line: String = screen_row.into_iter().collect();
            println!("{line}")
        }
    }
}

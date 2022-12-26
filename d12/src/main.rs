use std::fs;

fn main() {
    let mut map = parse_input("input.txt".to_string());

    map.make_option_map();

    // Pt. 1
    let original_start = map.start.clone();
    let path = bfs(&map, original_start);
    println!("Shortest path: {}", path.len());

    // Pt. 2
    let mut paths = Vec::new();
    for i in 0..map.grid.len() {
        for j in 0..map.grid.first().unwrap().len() {
            if map.grid[i][j] == 0 {
                let p = bfs(&map, (i, j));
                println!("Len of path from ({}, {}): {}", i, j, p.len());
                paths.push(p)
            }
        }
    }
    println!(
        "Shortest hiking trail path: {}",
        paths
            .iter()
            .map(|x| x.len())
            .filter(|&x| x > 0)
            .min()
            .unwrap()
    );
}

fn parse_input(path: String) -> Map {
    let inputstr = fs::read_to_string(path).expect("Not found");

    let mut grid = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in inputstr.lines().enumerate() {
        let mut gridline = Vec::new();

        for (j, c) in line.chars().enumerate() {
            let nc = match c {
                'S' => {
                    start = (i, j);
                    'a'
                }
                'E' => {
                    end = (i, j);
                    'z'
                }
                _ => c,
            };

            gridline.push(nc as u32 - 'a' as u32)
        }

        grid.push(gridline);
    }

    Map {
        grid: grid,
        option_map: Vec::new(),
        start: start,
        end: end,
    }
}

struct Map {
    grid: Vec<Vec<u32>>,
    option_map: Vec<Vec<Vec<(usize, usize)>>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn get_options(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut options = Vec::new();
        let current = self.grid[i][j];

        let mut n: (usize, usize);

        // left
        if j > 0 {
            n = (i, j - 1);
            if self.grid[n.0][n.1] <= current + 1 {
                options.push(n)
            }
        }
        // right
        if j < (self.grid.first().unwrap().len() - 1) {
            n = (i, j + 1);
            if self.grid[n.0][n.1] <= current + 1 {
                options.push(n)
            }
        }
        // up
        if i > 0 {
            n = (i - 1, j);
            if self.grid[n.0][n.1] <= current + 1 {
                options.push(n)
            }
        }
        // down
        if i < (self.grid.len() - 1) {
            n = (i + 1, j);
            if self.grid[n.0][n.1] <= current + 1 {
                options.push(n)
            }
        }

        options
    }

    fn make_option_map(&mut self) {
        let mut option_map = Vec::new();
        for (i, row) in self.grid.iter().enumerate() {
            let mut row_options = Vec::new();
            for (j, _) in row.iter().enumerate() {
                row_options.push(self.get_options(i, j))
            }
            option_map.push(row_options);
        }

        self.option_map = option_map;
    }
}

fn bfs(map: &Map, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut visited = vec![start];
    let mut paths = vec![vec![start]];

    while paths.len() > 0 {
        let cp = paths.remove(0);
        let cn = cp.last().unwrap();
        let options = &map.option_map[cn.0][cn.1];

        if options.contains(&map.end) {
            return cp;
        }

        for option in options.iter() {
            if !visited.contains(option) {
                let mut np = cp.clone();
                np.push(option.clone());
                paths.push(np);
                visited.push(option.clone());
            }
        }
    }
    Vec::new()
}

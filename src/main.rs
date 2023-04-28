use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Grid {
    Ice,
    Arrow(Arrow),
    On,
    Hand,
    Punch,
    Empty,
}

#[derive(Debug, Clone, Copy)]
struct Arrow {
    direction: (i8, i8),
}

#[derive(Debug)]
struct Spinner {
    r_size: usize,
    a_size: usize,
    grids: Vec<Vec<Grid>>,
}

impl Spinner {
    // r: Radial size, a: Angular size
    fn new(r: usize, a: usize) -> Spinner {
        let mut grids = Vec::with_capacity(r);
        for _i in 0..r {
            let mut circle = Vec::with_capacity(a);
            for _j in 0..a {
                circle.push(Grid::Empty);
            }
            grids.push(circle);
        }
        Spinner {
            grids,
            r_size: r,
            a_size: a,
        }
    }

    fn set_grids(&mut self, grids: Vec<(usize, usize, Grid)>) {
        for g in grids {
            self.set_grid(g.0, g.1, g.2);
        }
    }

    fn set_grid(&mut self, r: usize, a: usize, new_grid: Grid) {
        self.grids[r][a] = new_grid;
    }

    // mutates the spinner, in anglar direction
    // r: the radial index to change
    fn mutate_angular(&mut self, r: usize, n: usize) {
        // make a copy
        let mut new_circle = Vec::with_capacity(self.a_size);
        for i in 0..self.a_size {
            new_circle.push(self.grids[r][(i + n) % self.a_size]);
        }

        for i in 0..self.a_size {
            self.grids[r][i] = new_circle[i];
        }
    }
    // mutates the spinner, in raidal direction move the axes "a" towards the center by "n" units.
    // a: the angular index to change
    // a should only take half of the a_size.
    fn mutate_radial(&mut self, a: usize, n: usize) {
        let mut old_axes = Vec::with_capacity(self.r_size * 2);
        for i in 0..self.r_size {
            old_axes.push(self.grids[i][a]);
        }
        for i in (0..self.r_size).rev() {
            old_axes.push(self.grids[i][self.a_size / 2 + a]);
        }
        let mut new_axes = Vec::with_capacity(self.r_size * 2);

        for i in 0..self.r_size * 2 {
            new_axes.push(old_axes[(i + n) % (self.r_size * 2)])
        }
        // some grids will move to the other side and get arrow changed.
        // if the grid moves back and moves to the other half, it should have direction changed.
        for i in 0..self.r_size * 2 {
            if (((i + n) / self.r_size) % 2) == ((i / self.r_size) % 2) {
                continue;
            }

            let grid = new_axes[i];

            match grid {
                Grid::Arrow(arrow) => {
                    let new_arrow = Arrow {
                        direction: (-arrow.direction.0, -arrow.direction.1),
                    };
                    new_axes[i] = Grid::Arrow(new_arrow);
                }
                _ => (),
            }
        }

        for i in 0..self.r_size {
            self.grids[i][a] = new_axes[i];
            self.grids[self.r_size - i - 1][self.a_size / 2 + a] = new_axes[i + self.r_size];
        }
    }
}

#[derive(Debug)]
struct Score {
    punch: bool,
    enable_hand: bool,
    hand: bool,
    ice: bool,
}

fn exercise(spinner: &Spinner) -> Score {
    let mut score = Score {
        punch: false,
        enable_hand: false,
        hand: false,
        ice: false,
    };

    let mut loc: (i8, i8) = ((spinner.r_size - 1).try_into().unwrap(), 0);
    let mut dir: (i8, i8) = (0, 0);
    let mut is_stopped = false;
    let mut is_hand_enabled = false;
    let mut visited: HashSet<(i8, i8)> = HashSet::new();
    while !is_stopped {
        if visited.contains(&loc) || loc.0 < 0 || loc.0 >= spinner.r_size.try_into().unwrap() {
            break;
        }
        visited.insert(loc);
        match spinner.grids[loc.0 as usize][loc.1 as usize] {
            Grid::Arrow(arrow) => dir = arrow.direction,
            Grid::Ice => {
                is_stopped = true;
                score.ice = true;
            }
            Grid::Punch => {
                is_stopped = true;
                score.punch = true;
            }
            Grid::On => {
                is_hand_enabled = true;
                score.enable_hand = true;
            }
            Grid::Hand => {
                if is_hand_enabled {
                    score.hand = true;
                    is_stopped = true;
                }
            }
            _ => (),
        }
        loc = (
            (loc.0 + dir.0),
            (loc.1 + dir.1 + (spinner.a_size as i8)) % (spinner.a_size as i8),
        );
    }
    score
}

fn dfs(spinner: &mut Spinner, level: i8, mut path: Vec<(i8, i8, i8, i8)>) -> bool {
    if level > 2 {
        return false;
    }
    for a in 0..spinner.a_size / 2 {
        for r in 1..spinner.r_size * 2 {
            if r == 3 {
                println!("r==3");
            }
            spinner.mutate_radial(a, r);
            let step: (i8, i8, i8, i8) = (0, 0, a.try_into().unwrap(), r.try_into().unwrap());
            path.push(step.clone());
            // println!("exploring step {:?}\n\n", step);
            // println!("spinner after is: {:?}\n\n", spinner);

            let score = exercise(&spinner);
            if score.hand {
                println!("solution: {:?}", path.clone());
                return true;
            }
            if dfs(spinner, level + 1, path.clone()) {
                println!("spinner: {:?}", spinner);
                return true;
            }
            path.pop();
            spinner.mutate_radial(a, spinner.r_size * 2 - r);
        }
    }
    for a in 1..spinner.a_size {
        for r in 0..spinner.r_size {
            println!("before exploration spinner is: {:?}\n\n", spinner);
            spinner.mutate_angular(r, a);
            let step: (i8, i8, i8, i8) = (a.try_into().unwrap(), r.try_into().unwrap(), 0, 0);
            path.push(step.clone());
            let score = exercise(&spinner);
            if score.hand {
                println!("solution: {:?}", path.clone());
                return true;
            }
            if dfs(spinner, level + 1, path.clone()) {
                println!("spinner: {:?}", spinner);
                return true;
            }
            path.pop();
            spinner.mutate_angular(r, spinner.a_size - a);
        }
    }
    return false;
}

fn main() {
    let mut spinner = Spinner::new(4, 12);
    let ice_grid = Grid::Ice;
    let punch_grid = Grid::Punch;
    spinner.set_grids(vec![
        (0, 0, ice_grid),
        (2, 0, ice_grid),
        (3, 0, ice_grid),
        (3, 2, ice_grid),
        (3, 3, ice_grid),
        (3, 4, ice_grid),
        (3, 5, ice_grid),
        (1, 7, ice_grid),
        (1, 8, ice_grid),
        (2, 8, ice_grid),
        (1, 9, ice_grid),
        (2, 9, ice_grid),
        (0, 10, ice_grid),
        (2, 10, ice_grid),
        (3, 10, ice_grid),
        (0, 11, ice_grid),
        (1, 11, ice_grid),
        (2, 11, ice_grid),
        (0, 2, punch_grid),
        (1, 5, punch_grid),
        // (0, 8, Grid::Heart),
        (1, 2, Grid::On),
        (3, 11, Grid::Hand),
        (0, 1, Grid::Arrow(Arrow { direction: (0, -1) })),
        (1, 1, Grid::Arrow(Arrow { direction: (0, -1) })),
        (2, 1, Grid::Arrow(Arrow { direction: (0, 1) })),
        (3, 1, Grid::Arrow(Arrow { direction: (-1, 0) })),
        (0, 3, Grid::Arrow(Arrow { direction: (0, 1) })),
        (2, 4, Grid::Arrow(Arrow { direction: (-1, 0) })),
        (0, 5, Grid::Arrow(Arrow { direction: (1, 0) })),
        (1, 6, Grid::Arrow(Arrow { direction: (-1, 0) })),
        (3, 6, Grid::Arrow(Arrow { direction: (-1, 0) })),
        (2, 7, Grid::Arrow(Arrow { direction: (0, -1) })),
        (3, 8, Grid::Arrow(Arrow { direction: (-1, 0) })),
        (1, 10, Grid::Arrow(Arrow { direction: (0, 1) })),
    ]);
    let verify = false;
    if verify {
        println!("initial spinner:\n{:#?}", spinner);
        spinner.mutate_angular(2, 1);
        println!("\n after first mutation\n{:#?}", spinner);
        spinner.mutate_angular(3, 6);
        println!("\n after second mutation\n{:#?}", spinner);

        spinner.mutate_angular(1, 9);
        println!("\n after last mutation\n{:#?}", spinner);

        println!("{:?}", exercise(&spinner));
        return;
    }
    // Solution is in the format of
    // ( anticlockwise_angular_rotation_number,
    //   angular_rotation_row,
    //   radial_move_axis,
    //   radial_move_number
    // )
    // For example, the result of this search is [(1, 2, 0, 0), (6, 3, 0, 0), (9, 1, 0, 0)]
    // This indicates rotate circle 2 (the third circle from inside) counter clockwise by 1
    // rotate the fourth circle from inside counterclockwise by 6
    // rotate the second circle from inside counterclockwise by 9
    // See the attached screenshot.
    let path = Vec::with_capacity(3);
    if !dfs(&mut spinner, 0, path) {
        println!("no solution found!");
    }
}

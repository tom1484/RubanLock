use itertools::Itertools;

struct Ruban {
    grids: [[[u8; 4]; 4]; 4],
}

enum Direction {
    X,
    Y,
    Z,
}

impl Ruban {
    fn new(shape: [[[u8; 4]; 2]; 2]) -> Ruban {
        let mut grids = [[[0; 4]; 4]; 4];
        for x in 1..3 {
            for y in 1..3 {
                for z in 0..4 {
                    grids[x][y][z] = shape[x - 1][y - 1][z];
                }
            }
        }
        Ruban { grids: grids }
    }

    fn get_single_grid(&self, x: usize, y: usize, z: usize, rotation: u8) -> u8 {
        match rotation {
            // 90 degree rotation
            0 => self.grids[x][y][z],
            // 180 degree rotation
            1 => self.grids[y][3 - x][z],
            // 270 degree rotation
            2 => self.grids[3 - x][3 - y][z],
            // 360 degree rotation
            3 => self.grids[3 - y][x][z],
            _ => {
                panic!("Invalid rotation");
            }
        }
    }

    fn get_grids(
        &self,
        direction: &Direction,
        polarity: u8,
        rotation: u8,
        shift: u8,
    ) -> [[[u8; 4]; 4]; 4] {
        let mut grids = [[[0; 4]; 4]; 4];
        let mut xp: usize;
        let mut yp: usize;
        let mut zp: usize;

        // two Rubans in the same direction
        let s_iter = match shift {
            0 => 0..2,
            1 => 2..4,
            _ => {
                panic!("Invalid shift");
            }
        };

        match direction {
            Direction::X => {
                for x in 0..4 {
                    for y in 1..3 {
                        for (z, sz) in (1..3).zip(s_iter.clone()) {
                            if polarity == 0 {
                                xp = z;
                                yp = y;
                                zp = 3 - x;
                            } else {
                                xp = 3 - z;
                                yp = y;
                                zp = x;
                            }
                            grids[x][y][sz] += self.get_single_grid(xp, yp, zp, rotation);
                        }
                    }
                }
            }
            Direction::Y => {
                for (x, sx) in (1..3).zip(s_iter.clone()) {
                    for y in 0..4 {
                        for z in 1..3 {
                            if polarity == 0 {
                                xp = x;
                                yp = 3 - z;
                                zp = y;
                            } else {
                                xp = x;
                                yp = z;
                                zp = 3 - y;
                            }
                            grids[sx][y][z] += self.get_single_grid(xp, yp, zp, rotation);
                        }
                    }
                }
            }
            Direction::Z => {
                for x in 1..3 {
                    for (y, sy) in (1..3).zip(s_iter.clone()) {
                        for z in 0..4 {
                            if polarity == 0 {
                                xp = x;
                                yp = y;
                                zp = z;
                            } else {
                                xp = 3 - x;
                                yp = y;
                                zp = 3 - z;
                            }
                            grids[x][sy][z] += self.get_single_grid(xp, yp, zp, rotation);
                        }
                    }
                }
            }
        }

        grids
    }

    fn add_to_grids(
        &self,
        grids: &mut [[[u8; 4]; 4]; 4],
        direction: &Direction,
        polarity: u8,
        rotation: u8,
        shift: u8,
    ) {
        let mut xp: usize;
        let mut yp: usize;
        let mut zp: usize;

        let s_iter = match shift {
            0 => 0..2,
            1 => 2..4,
            _ => {
                panic!("Invalid shift");
            }
        };

        match direction {
            Direction::X => {
                for x in 0..4 {
                    for y in 1..3 {
                        for (z, sz) in (1..3).zip(s_iter.clone()) {
                            if polarity == 0 {
                                xp = z;
                                yp = y;
                                zp = 3 - x;
                            } else {
                                xp = 3 - z;
                                yp = y;
                                zp = x;
                            }
                            grids[x][y][sz] += self.get_single_grid(xp, yp, zp, rotation);
                        }
                    }
                }
            }
            Direction::Y => {
                for (x, sx) in (1..3).zip(s_iter.clone()) {
                    for y in 0..4 {
                        for z in 1..3 {
                            if polarity == 0 {
                                xp = x;
                                yp = 3 - z;
                                zp = y;
                            } else {
                                xp = x;
                                yp = z;
                                zp = 3 - y;
                            }
                            grids[sx][y][z] += self.get_single_grid(xp, yp, zp, rotation);
                        }
                    }
                }
            }
            Direction::Z => {
                for x in 1..3 {
                    for (y, sy) in (1..3).zip(s_iter.clone()) {
                        for z in 0..4 {
                            if polarity == 0 {
                                xp = x;
                                yp = y;
                                zp = z;
                            } else {
                                xp = 3 - x;
                                yp = y;
                                zp = 3 - z;
                            }
                            grids[x][sy][z] += self.get_single_grid(xp, yp, zp, rotation);
                        }
                    }
                }
            }
        }
    }
}

fn print_grids(grids: &[[[u8; 4]; 4]; 4], indent: usize) {
    let indent_str = " ".repeat(indent);
    for x in 0..4 {
        for y in 0..4 {
            print!("{}", indent_str);
            for z in 0..4 {
                print!("{} ", grids[x][y][z]);
            }
            println!();
        }
        println!();
    }
}
fn count_grids(grids: &[[[u8; 4]; 4]; 4]) -> u8 {
    let mut count = 0;
    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                count += grids[x][y][z] & 1;
            }
        }
    }
    count
}

fn main() {
    let rubans = [
        Ruban::new([[[0, 1, 0, 0], [1, 1, 1, 1]], [[0, 0, 0, 0], [1, 0, 0, 1]]]),
        Ruban::new([[[1, 1, 1, 1], [1, 1, 0, 0]], [[1, 0, 0, 1], [1, 0, 0, 0]]]),
        Ruban::new([[[1, 1, 1, 1], [0, 1, 0, 0]], [[1, 0, 0, 1], [0, 0, 0, 0]]]),
        Ruban::new([[[1, 1, 1, 1], [1, 1, 1, 1]], [[0, 0, 1, 1], [0, 0, 1, 1]]]),
        Ruban::new([[[1, 1, 1, 1], [0, 1, 1, 1]], [[1, 0, 0, 1], [0, 0, 0, 1]]]),
        Ruban::new([[[1, 1, 1, 1], [1, 0, 1, 1]], [[1, 0, 0, 1], [1, 0, 0, 1]]]),
    ];

    let mut grids = [[[0; 4]; 4]; 4];
    let order_iter = (0..6).permutations(6);

    let directions = [
        Direction::X,
        Direction::X,
        Direction::Y,
        Direction::Y,
        Direction::Z,
        Direction::Z,
    ];
    let shifts = [0, 1, 0, 1, 0, 1];

    // permute order
    'order_for: for (perm, order) in order_iter.enumerate() {
        println!("Permutation: {}...", perm);

        let permuted_rubans = order.iter().map(|&i| &rubans[i]).collect_vec();

        // permute polarity
        for j in 0..(1_u64 << 6) {
            let polarities = (0..6).map(|i| ((j >> i) & 1) as u8).collect_vec();

            // permute rotation
            for k in 0..(1_u64 << (2 * 6)) {
                let rotations = (0..6).map(|i| ((k >> (2 * i)) & 3) as u8).collect_vec();

                for b in 0..6 {
                    permuted_rubans[b].add_to_grids(
                        &mut grids,
                        &directions[b],
                        polarities[b],
                        rotations[b],
                        shifts[b],
                    );
                }

                let count = count_grids(&grids);
                if count == 56 {
                    println!("Found a solution!");
                    for b in 0..6 {
                        println!("Ruban {}:", order[b]);
                        let current = permuted_rubans[b].get_grids(
                            &directions[b],
                            polarities[b],
                            rotations[b],
                            shifts[b],
                        );
                        print_grids(&current, 2);
                    }
                    break 'order_for;
                }

                grids = [[[0; 4]; 4]; 4];
            }
        }
    }
}

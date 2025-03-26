struct Solution;

impl Solution {
    pub fn check_valid_cuts(n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let grid_len: usize = n as usize;
        let mut grid = vec![vec![0; grid_len]; grid_len];

        for rows in rectangles {
            if let &[x_start, y_start, x_end, y_end] = rows.as_slice() {
                let (x_start, y_start, x_end, y_end) = (
                    x_start as usize,
                    y_start as usize,
                    x_end as usize,
                    y_end as usize,
                );

                if grid
                    .iter_mut()
                    .take(x_end)
                    .skip(x_start)
                    .any(|row| row.iter().take(y_end).skip(y_start).any(|&cell| cell == 1))
                {
                    return false;
                }

                grid.iter_mut().take(x_end).skip(x_start).for_each(|row| {
                    row.iter_mut()
                        .take(y_end)
                        .skip(y_start)
                        .for_each(|cell| *cell = 1)
                });
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}

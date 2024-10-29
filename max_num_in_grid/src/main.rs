struct Solution;

impl Solution {
    const MOVES: [i32; 3] = [-1, 0, 1];

    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_moves = 0;
        let mut memo = vec![vec![-1; n]; m];

        for start_row in 0..m {
            max_moves = max_moves.max(Self::dfs(start_row, 0, &grid, &mut memo));
        }

        max_moves
    }

    fn dfs(row: usize, col: usize, grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[row][col] != -1 {
            return memo[row][col];
        }

        let m = grid.len();
        let n = grid[0].len();
        let mut max_moves = 0;

        for &move_row in &Self::MOVES {
            let new_row = row as i32 + move_row;
            let new_col = col + 1;

            if new_row >= 0 && new_row < m as i32 && new_col < n {
                let new_row = new_row as usize;

                if grid[new_row][new_col] > grid[row][col] {
                    max_moves = max_moves.max(1 + Self::dfs(new_row, new_col, grid, memo));
                }
            }
        }

        memo[row][col] = max_moves;
        max_moves
    }
}

fn main() {
    let grid = vec![
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20],


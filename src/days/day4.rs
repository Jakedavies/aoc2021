

#[derive(Debug, Clone, Copy)]
struct BingoCell {
    filled: bool,
    val: u32,
}

#[derive(Debug)]
struct Bingo {
    won: bool,
    board: Vec<Vec<BingoCell>>
}

impl Bingo {
    fn fill_number(&mut self, num: u32) {
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if self.board[y][x].val == num {
                    self.board[y][x].filled = true;
                }
            }
        }
    }

    fn check_won(&self) -> bool {
        let mut x_candidates = vec![true, true, true, true, true];
        let mut y_candidates = vec![true, true, true, true, true];
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                let cell = self.board[y][x];
                if cell.filled == false {
                    x_candidates[x] = false;
                    y_candidates[y] = false;
                }
            }
        }

        // if any candidates left, we won
        for candidate in [x_candidates, y_candidates].concat() {
            if candidate == true {
                return true;
            }
        } 
        false
    }

    fn from_lines(input: &[&str]) -> Self {
        let mut rows = vec![];
        for line in input {
            let slices = vec![
                &line[0..2],
                &line[3..5],
                &line[6..8],
                &line[9..11],
                &line[12..14],
            ];
            let row = slices.iter().map(|num| BingoCell { val: u32::from_str_radix(num.trim(), 10).unwrap(), filled: false }).collect();
            rows.push(row);
        }

        Bingo {
            won: false,
            board: rows,
        }
    }

    fn print(&self) {
        for row in &self.board {
            for x in row {
                print!("{} ", if x.filled { 1 } else { 0 })
            }
            println!();
        }
    }

    fn get_filled_sum(&self) -> u32 {
        let mut filled_sum: u32= 0;
        for row in &self.board {
            for x in row {
                if !x.filled {
                    filled_sum += x.val;
                }
            }
        }
        return filled_sum;
    }
}



pub fn part1() {
    let values: Vec<&str> = include_str!("./../../inputs/input4")
        .lines()
        .collect();

    let draw: Vec<u32> = values[0].split(",").map(|s| u32::from_str_radix(s, 10).unwrap()).collect();

    let chunks: Vec<&[&str]> = values[2..values.len()]
        .chunks(6)
        .map(|f| &f[0..5])
        .collect();

    let mut boards: Vec<Bingo> = vec![];

    for chunk in chunks {
        let board = Bingo::from_lines(chunk);
        boards.push(board);
    }

    println!("Board count {}", boards.len());
    
    let board_count = boards.len();
    'outer: for num in draw {
        for board_index in 0..board_count{
            let b = boards.get_mut(board_index).unwrap();
            b.fill_number(num);
            if b.check_won() {
                b.print();
                println!("Score: {}", b.get_filled_sum() * num);
                break 'outer;
            }
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn test_bingo_init() {
        
    }
}

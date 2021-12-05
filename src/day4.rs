use std::str::FromStr;
use array2d::Array2D;

struct BingoGame {
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn from_raw_data(data: Vec<String>) -> BingoGame {
        BingoGame {
            boards: data.split(|s| s.is_empty())
                .map(|b| BingoBoard::from_rows(b))
                .collect::<Vec<BingoBoard>>()
        }
    }

    fn draw(&mut self, v: &u32) -> Option<u32> {
        let mut winner_score: i32 = -1;

        for b in &mut self.boards {
            match b.draw(v) {
                Some(score) => {
                    winner_score = score as i32;
                }
                None => {
                    // do nothing, mark remaining boards
                }
            }
        }

        if winner_score >= 0 {
            return Some(winner_score as u32);
        }

        return None;
    }
}

struct BingoBoard {
    values: Array2D<u32>,
    marks: Array2D<u32>,
    won: bool,
}

impl BingoBoard {
    fn from_rows(rows: &[String]) -> BingoBoard {
        let board_values = Array2D::from_rows(
            &rows.iter()
                .map(|s| {
                    s.split_whitespace()
                        .map(|v| {
                            u32::from_str(v).unwrap()
                        })
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        );

        let nr = board_values.num_rows();
        let nc = board_values.num_columns();

        BingoBoard {
            values: board_values,
            marks: Array2D::filled_with(0, nr, nc),
            won: false,
        }
    }

    fn draw(&mut self, v: &u32) -> Option<u32> {
        if self.won {
            return None;
        }

        for i in 0..self.values.row_len() {
            for j in 0..self.values.column_len() {
                if self.values[(i, j)] == *v {
                    self.marks[(i, j)] = 1
                }
            }
        }

        for row in self.marks.as_rows() {
            if row.iter().all(|v| *v == 1) {
                self.won = true;
                return Some(self.score(v));
            }
        }

        for col in self.marks.as_columns() {
            if col.iter().all(|v| *v == 1) {
                self.won = true;
                return Some(self.score(v));
            }
        }

        return None;
    }

    fn score(&self, v: &u32) -> u32 {
        let mut unmarked_sum: u32 = 0;

        for i in 0..self.values.row_len() {
            for j in 0..self.values.column_len() {
                if self.marks[(i, j)] == 0 {
                    unmarked_sum += self.values[(i, j)];
                }
            }
        }

        return unmarked_sum * v;
    }
}

pub fn task1(input: Vec<String>) {
    let queue = input[0].clone()
        .split(",")
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<u32>>();

    let mut game = BingoGame::from_raw_data(input[2..].to_vec());
    for (step, n) in queue.iter().enumerate() {
        match game.draw(n) {
            Some(score) => {
                println!("Game finished at step={}, winning board score={}", step, score);
                break;
            }
            None => {
                // let the game continue
            }
        }
    }
}

pub fn task2(input: Vec<String>) {
    let queue = input[0].clone()
        .split(",")
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<u32>>();

    let mut game = BingoGame::from_raw_data(input[2..].to_vec());
    let mut scores = Vec::<u32>::new();

    for n in queue {
        match game.draw(&n) {
            Some(score) => {
                scores.push(score);
            }
            None => {}
        }
    }

    println!("last finished board score was {}", scores.last().unwrap());
}

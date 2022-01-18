use std::fs;

fn main() {
    let (nums, mut boards) = deserialize(&fs::read_to_string("./input.txt").unwrap());

    let winner_board: Option<usize> = nums.iter().find_map(|&num| {
        for board in boards.iter_mut() {
            board.mark_num(num);
            if board.has_win() {
                return Some(board.get_score() * num);
            }
        }
        return None;
    });

    if let Some(score) = winner_board {
        print!("{}", score);
    }
}

fn deserialize(serialized: &str) -> (Vec<usize>, Vec<Board>) {
    let (n, boards) = serialized.split_once("\n\n").unwrap();

    let numb_seq: Vec<usize> = n.split(",").map(|it| it.parse().unwrap()).collect();

    let b: Vec<Board> = boards
        .split("\n\n")
        .map(|b| {
            let nums: Vec<usize> = b.split_whitespace().map(|n| n.parse().unwrap()).collect();
            return Board::from_num_array(nums);
        })
        .collect();

    (numb_seq, b)
}

#[derive(Debug)]
struct Cell {
    value: usize,
    marked: bool,
}

impl Cell {
    fn new(value: usize) -> Cell {
        return Cell {
            value,
            marked: false,
        };
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

struct Board {
    board_size: usize,
    grid: Vec<Cell>,
}

impl Board {
    fn from_num_array(nums: Vec<usize>) -> Board {
        return Board {
            board_size: 5,
            grid: nums.iter().map(|&f| Cell::new(f)).collect(),
        };
    }

    /// Marks a number in the board and returns the index of the number
    fn mark_num(&mut self, num: usize) -> Option<usize> {
        let found = self.get_cell_idx_by_value(num)?;
        self.grid[found].mark();

        Some(found)
    }

    fn get_cell_idx_by_value(&self, value: usize) -> Option<usize> {
        self.grid.iter().enumerate().find_map(|(idx, c)| {
            if c.value == value {
                return Some(idx);
            }
            None
        })
    }

    fn get_rows(&self) -> Vec<Vec<&Cell>> {
        let outer = (0..self.board_size)
            .map(|row_idx| {
                (0..self.board_size)
                    .map(|idx| (row_idx * self.board_size) + idx)
                    .map(|idx| &self.grid[idx])
                    .collect()
            })
            .collect::<Vec<Vec<&Cell>>>();

        outer
    }

    fn get_columns(&self) -> Vec<Vec<&Cell>> {
        let outer = (0..self.board_size)
            .map(|row_idx| {
                (0..self.board_size)
                    .map(|idx| (idx * self.board_size) + row_idx)
                    .map(|idx| &self.grid[idx])
                    .collect()
            })
            .collect::<Vec<Vec<&Cell>>>();

        outer
    }

    fn has_win(&self) -> bool {
        let row_winner = self
            .get_rows()
            .iter()
            .any(|row| row.iter().all(|c| c.marked));
        if row_winner {
            return true;
        }

        let col_winner = self
            .get_columns()
            .iter()
            .any(|col| col.iter().all(|c| c.marked));
        return col_winner;
    }

    /// Score is the sum of all unmarked numbers
    fn get_score(&self) -> usize {
        self.grid
            .iter()
            .filter(|c| !c.marked)
            .map(|c| c.value)
            .sum()
    }
}

#[test]
fn test_horizontal() {
    let mut b = Board::from_num_array((0..25).collect());
    b.mark_num(0);
    b.mark_num(1);
    b.mark_num(2);
    b.mark_num(3);
    b.mark_num(4);
    assert_eq!(b.has_win(), true);
}

#[test]
fn test_vertical() {
    let mut b = Board::from_num_array((0..25).collect());
    b.mark_num(0);
    b.mark_num(5);
    b.mark_num(10);
    b.mark_num(15);
    b.mark_num(20);
    assert_eq!(b.has_win(), true);
}

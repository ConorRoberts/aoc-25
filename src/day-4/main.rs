use aoc_25::get_input_file;
use std::fs;

type CellBoard = Vec<Vec<Cell>>;

#[derive(PartialEq, Debug)]
enum Cell {
    Empty,
    Paper,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '@' => Self::Paper,
            _ => Self::Empty,
        }
    }
}

fn count_paper_around(x: usize, y: usize, cells: &CellBoard) -> usize {
    let mut count = 0;

    let size_x = cells[0].len();
    let size_y = cells.len();

    let delta_x: Vec<i16> = vec![-1, 0, 1];

    for dx in delta_x {
        let delta_y: Vec<i16> = vec![-1, 0, 1];
        for dy in delta_y {
            let px = x as i16 + dx;
            let py = y as i16 + dy;

            if (dx == 0 && dy == 0)
                || px < 0
                || px >= (size_x as i16)
                || py < 0
                || py >= (size_y as i16)
            {
                continue;
            }

            if cells[py as usize][px as usize] == Cell::Paper {
                count += 1;
            }
        }
    }

    count
}

type CellPosition = (usize, usize);

fn get_accessible_cells(cells: &CellBoard) -> (usize, Vec<CellPosition>) {
    let mut result: Vec<CellPosition> = Vec::new();

    for (y, row) in cells.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != Cell::Paper {
                continue;
            }

            let paper_count = count_paper_around(x, y, &cells);

            if paper_count < 4 {
                result.push((x, y))
            }
        }
    }

    (result.len(), result)
}

// Goal: find paper cells with <4 paper cells around them

fn main() {
    let file_name = get_input_file();

    let mut cells: CellBoard = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| Cell::from(c)).collect())
        .collect();

    let mut count = 0;

    loop {
        let (accessible_count, accessible_cells) = get_accessible_cells(&cells);

        if accessible_count == 0 {
            break;
        }

        count += accessible_count;

        for (x, y) in accessible_cells {
            cells[y][x] = Cell::Empty;
        }
    }

    println!("Total {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_board() -> CellBoard {
        vec![
            vec![
                Cell::Empty,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
            ],
            vec![
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
            ],
            vec![
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
            ],
            vec![
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
            ],
            vec![
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
            ],
            vec![
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
            ],
            vec![
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
            ],
            vec![
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
            ],
            vec![
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
            ],
            vec![
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Paper,
                Cell::Paper,
                Cell::Empty,
                Cell::Paper,
                Cell::Empty,
            ],
        ]
    }

    #[test]
    fn test_count_paper_around() {
        assert_eq!(count_paper_around(2, 0, &make_test_board()), 3);
        assert_eq!(count_paper_around(3, 0, &make_test_board()), 3);
    }
}

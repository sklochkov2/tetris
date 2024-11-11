use rand::seq::SliceRandom;
use std::{thread, time};

fn empty_board() -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    for _ in 0..16 {
        res.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
    res
}

fn all_the_pieces() -> Vec<Vec<Vec<i32>>> {
    let mut stick: Vec<Vec<i32>> = Vec::new();
    let mut square: Vec<Vec<i32>> = Vec::new();
    let mut triangle: Vec<Vec<i32>> = Vec::new();
    let mut right_g: Vec<Vec<i32>> = Vec::new();
    let mut left_g: Vec<Vec<i32>> = Vec::new();
    let mut left_snake: Vec<Vec<i32>> = Vec::new();
    let mut right_snake: Vec<Vec<i32>> = Vec::new();
    let mut short_stick: Vec<Vec<i32>> = Vec::new();
    let mut medium_stick: Vec<Vec<i32>> = Vec::new();
    let mut junior: Vec<Vec<i32>> = Vec::new();
    let mut triforce: Vec<Vec<i32>> = Vec::new();
    stick.push(vec![1, 1, 1, 1]);
    square.push(vec![2, 2]);
    square.push(vec![2, 2]);
    triangle.push(vec![0, 3, 0]);
    triangle.push(vec![3, 3, 3]);
    right_g.push(vec![4, 0, 0]);
    right_g.push(vec![4, 4, 4]);
    left_g.push(vec![0, 0, 5]);
    left_g.push(vec![5, 5, 5]);
    left_snake.push(vec![6, 0]);
    left_snake.push(vec![6, 6]);
    left_snake.push(vec![0, 6]);
    right_snake.push(vec![0, 7]);
    right_snake.push(vec![7, 7]);
    right_snake.push(vec![7, 0]);
    short_stick.push(vec![1, 1]);
    medium_stick.push(vec![1, 1, 1]);
    junior.push(vec![1]);
    triforce.push(vec![0, 2]);
    triforce.push(vec![2, 2]);
    vec![
        stick,
        square,
        triangle,
        right_g,
        left_g,
        left_snake,
        right_snake,
        //short_stick,
        //medium_stick,
        //junior,
        //triforce,
    ]
}

fn touches_stuff(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, x: usize, y: usize) -> bool {
    for i in 0..figure.len() {
        for j in 0..figure[0].len() {
            if i + x > board.len() - 1 || j + y > board[0].len() - 1 {
                return true;
            }
            if i + x == board.len() - 1 {
                return true;
            }
            if figure[i][j] == 0 {
                continue;
            }
            if board[i + x + 1][j + y] > 0 {
                return true;
            }
        }
    }
    false
}

fn get_lowest_x(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, x: usize, y: usize) -> usize {
    let mut res: usize = x;
    while !touches_stuff(board, figure, res, y) {
        res += 1;
    }
    res
}

fn occupied(
    board: &Vec<Vec<i32>>,
    figure: &Vec<Vec<i32>>,
    x: usize,
    y: usize,
    pos_x: usize,
    pos_y: usize,
) -> bool {
    if board[pos_x][pos_y] > 0 {
        return true;
    } else if figure.len() > 0
        && pos_x >= x
        && pos_x < figure.len() + x
        && pos_y >= y
        && pos_y < figure[0].len() + y
        && figure[pos_x - x][pos_y - y] > 0
    {
        return true;
    }
    false
}

fn rows(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut res: i32 = 0;
    for i in 0..board.len() {
        let mut curr: i32 = 0;
        for j in 0..board[0].len() {
            if occupied(board, figure, x, y, i, j) {
                curr += 1;
            }
        }
        if curr == (board[0].len() as i32) {
            res += 1;
        }
    }
    res
}

fn holes(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut res: i32 = 0;
    for j in 0..board[0].len() {
        let mut top: i32 = 0;
        for i in 1..board.len() {
            if occupied(board, figure, x, y, i, j) {
                top += 1;
            } else if top > 0 {
                res += 10 + 5*top;
            }
        }
    }
    res
}

fn height(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut res: i32 = board.len() as i32;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if occupied(board, figure, x, y, i, j) {
                return res;
            }
        }
        res -= 1;
    }
    res
}

fn get_longest_row(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut res: i32 = 0;
    for i in 0..board.len() {
        let mut curr: i32 = 0;
        for j in 0..board[0].len() {
            if occupied(board, figure, x, y, i, j) {
                curr += 1;
            }
        }
        if curr > res {
            res = curr;
        }
    }
    res
}

fn almist_complete_rows(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut res: i32 = 0;
    for i in 0..board.len() {
        let mut curr: i32 = 0;
        for j in 0..board[0].len() {
            if occupied(board, figure, x, y, i, j) {
                curr += 1;
            }
        }
        if curr == 16 {
            res += 100;
        } else if curr == 15 {
            res += 10;
        } else if curr == 14 {
            res += 2;
        } else if curr == 13 {
            res += 1;
        }
    }
    res
}

fn eval_position(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, x: usize, y: usize) -> i64 {
    let low_x: usize = get_lowest_x(board, figure, x, y);
    let my_rows: i64 = rows(board, figure, low_x, y) as i64;
    let my_holes: i64 = holes(board, figure, low_x, y) as i64;
    let my_height: i64 = height(board, figure, low_x, y) as i64;
    let my_longest_row: i64 = get_longest_row(board, figure, low_x, y) as i64;
    let long_rows: i64 = almist_complete_rows(board, figure, low_x, y) as i64;
    let res: i64 = my_rows - my_holes * 5 - my_height * 10 + my_longest_row + long_rows * 10;
    //println!("rows: {}, holes: {}, height: {}, long_row: {}, eval: {}", my_rows, my_holes, my_height, my_longest_row, res);
    res
}

fn spawn_new_figure<'a>(
    board: &'a Vec<Vec<i32>>,
    figure: &'a Vec<Vec<i32>>,
) -> (bool, usize, usize) {
    let rotations: Vec<Vec<Vec<i32>>> = get_rotations(figure);
    let mut all_are_touching: bool = true;
    let mut curr_pos: usize = 0;
    let mut max_val: i64 = i64::MIN;
    let mut res: usize = 0;
    for i in 0..rotations.len() {
        for pos in 0..board[0].len() {
            if touches_stuff(board, &rotations[i], 0, pos) {
                continue;
            }
            all_are_touching = false;
            let curr_val: i64 = eval_position(board, &rotations[i], 0, pos);
            if curr_val > max_val {
                curr_pos = pos;
                max_val = curr_val;
                res = i;
            }
        }
    }
    //println!("Eval: {}", max_val);
    return (all_are_touching, res, curr_pos);
}

fn colour(clr: i32) -> String {
    let mut res: String = String::new();
    if clr == 1 {
        res = res + "\x1b[41m";
    } else if clr == 2 {
        res = res + "\x1b[42m";
    } else if clr == 3 {
        res = res + "\x1b[43m";
    } else if clr == 4 {
        res = res + "\x1b[44m";
    } else if clr == 5 {
        res = res + "\x1b[45m";
    } else if clr == 6 {
        res = res + "\x1b[46m";
    } else if clr == 7 {
        res = res + "\x1b[47m";
    }
    res = res + "#";
    if clr >= 1 && clr <= 7 {
        res = res + "\x1b[0m";
    }
    res
}

fn print_board(board: &Vec<Vec<i32>>, figure: &Vec<Vec<i32>>, is_figure: bool, x: usize, y: usize) {
    let mut res: String = "================\n".to_string();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] > 0 {
                res += &colour(board[i][j]);
            } else if is_figure
                && (i >= x && i < x + figure.len())
                && (j >= y && j < y + figure[0].len())
                && figure[i - x][j - y] > 0
            {
                res += &colour(figure[i - x][j - y]);
            } else {
                res = res + ".";
            }
        }
        res = res + "\n";
    }
    res += "================";
    println!("{}", res);
}

fn transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }
    let m = matrix.len();
    let n = matrix[0].len();
    let mut transposed = vec![Vec::with_capacity(m); n];
    for i in 0..n {
        for j in 0..m {
            transposed[i].push(matrix[j][i].clone());
        }
    }
    transposed
}

fn rotate_right<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = transpose(matrix);
    for row in &mut transposed {
        row.reverse();
    }
    transposed
}

fn rotate_left<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = transpose(matrix);
    transposed.reverse();
    transposed
}

fn rotate_180<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = matrix.clone();
    for row in &mut result {
        row.reverse();
    }
    result.reverse();
    result
}

fn get_rotations(piece: &Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let original = piece.clone();
    let left = rotate_left(piece);
    let right = rotate_right(piece);
    let upside_down = rotate_180(piece);
    vec![original, left, right, upside_down]
}

fn eliminate_row(board: &mut Vec<Vec<i32>>, row: usize) {
    for i in 0..row {
        for j in 0..board[0].len() {
            board[row - i][j] = board[row - i - 1][j];
        }
    }
    for j in 0..board[0].len() {
        board[0][j] = 0;
    }
}

fn fix_figure(
    board: &mut Vec<Vec<i32>>,
    figure: &Vec<Vec<i32>>,
    x: usize,
    y: usize,
    score: &mut u32,
) {
    for i in 0..figure.len() {
        for j in 0..figure[0].len() {
            if figure[i][j] > 0 {
                board[x + i][y + j] = figure[i][j];
            }
        }
    }
    let mut eliminated_lines: u32 = 0;
    for i in 0..board.len() {
        let mut empty: bool = false;
        for j in 0..board[0].len() {
            if board[i][j] == 0 {
                empty = true;
                break;
            }
        }
        if !empty {
            eliminate_row(board, i);
            if eliminated_lines == 0 {
                eliminated_lines = 100;
            } else {
                eliminated_lines *= 2;
            }
        }
    }
    *score += eliminated_lines;
}

fn tetris() {
    let mut board: Vec<Vec<i32>> = empty_board();
    let figures: Vec<Vec<Vec<i32>>> = all_the_pieces();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut we_lost: bool = false;
    let mut figure: &Vec<Vec<i32>> = &Vec::new();
    let mut rotation: usize;
    let mut rotations: Vec<Vec<Vec<i32>>>;
    let mut valid_figure: bool = false;
    let mut score: u32 = 0;
    loop {
        if !valid_figure {
            x = 0;
            let chosen_figure: &Vec<Vec<i32>> = figures.choose(&mut rand::thread_rng()).unwrap();
            (we_lost, rotation, y) = spawn_new_figure(&board, chosen_figure);
            rotations = get_rotations(chosen_figure);
            figure = &rotations[rotation];
            valid_figure = true
        } else {
            x += 1;
            if touches_stuff(&board, figure, x, y) {
                fix_figure(&mut board, figure, x, y, &mut score);
                valid_figure = false;
            }
        }
        if we_lost {
            break;
        }
        print_board(&board, figure, figure.len() > 0, x, y);
        println!("%%%%%%%%%%%%%%%%%%%% {}", score);
        thread::sleep(time::Duration::from_millis(30));
    }
    println!("Final score: {}", score);
}

fn main() {
    tetris();
}

use std::{env, fs};

fn draw(board: &[[i32; 9]; 9]) {
    for i in 0..9{
        if i % 3 == 0{
            println!("-------------------------");
        }
        for j in 0..9{
            if j % 3 == 0{
                print!("| ");
            }
            print!("{} ",board[i][j]);
        }
        println!("|");
    }
    println!("-------------------------");
}

fn find_spot(board: &[[i32; 9]; 9]) -> Option<(usize,usize)>{
    for i in 0..9{
        for j in 0..9{
            if board[i][j] == 0{
                return Some((i,j));
            } 
        }
    }
    None
}

fn start_box(i: usize) -> usize{
    // give start of x , y
    // 0 1 2 -> 0
    // 3 4 5 -> 3
    // 6 7 8 -> 6
    let mut x : usize;

    x = i / 3;
    x = x * 3;

    x
}

fn is_correct(board: [[i32; 9]; 9],i: usize,j: usize, num: i32) -> bool{
    // row
    for k in 0..9{
        if board[i][k] == num{
            return false;
        }
    }

    //column
    for k in 0..9{
        if board[k][j] == num{
            return false;
        }
    }

    //box
    let x = start_box(i);
    let y = start_box(j);
    for k in x..x+3{
        for l in y..y+3{
            if board[k][l] == num{
                return  false;
            }
        }
    }

    true
}


fn solve(board: &mut [[i32; 9]; 9]) -> bool{

    let x: usize;
    let y: usize;
    let res = find_spot(&board);

    match res {
        None => return true,
        Some(values) => {
            (x,y) = values;

            for num in 1..10{
                if is_correct(*board, x, y, num){
                    
                    board[x][y] = num;
                    if solve(board) {
                        return true;
                    }

                    board[x][y] = 0;
                }
            }
            
            false
        }
    }
}

fn  read_file(filename: &str) -> [[i32; 9];9]{

    let mut board = [
        [3,0,8,0,0,0,0,9,0],
        [6,0,0,0,0,0,5,0,0],
        [0,4,0,0,0,7,0,0,0],
        [0,5,1,0,0,9,0,0,0],
        [0,0,0,0,0,0,0,0,0],
        [4,0,9,8,1,0,0,0,6],
        [0,1,0,3,0,8,0,0,0],
        [0,0,6,0,5,0,0,0,0],
        [5,0,0,4,7,1,0,0,3]
    ];

    let content = fs::read_to_string(filename)
        .expect("Failed to read file");

    let mut i = 0;
    for line in content.lines() {
        let mut j = 0;
        for num in line.split(","){

            board[i][j] = num.parse().unwrap();
            j = j + 1;
        }
        i = i + 1;
    }


    board
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut board = read_file(args[1].as_str());

    solve(&mut board);
    draw(&board);
}

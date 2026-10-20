

fn draw(board: &[[i32; 9]; 9]) {
    println!("---------------------");
    for i in 0..9{
        print!("| ");
        for j in 0..9{
            print!("{} ",board[i][j])
        }
        println!("|")
    }
    println!("---------------------");
}

fn find_spot(board: &mut [[i32; 9]; 9]) -> Option<(usize,usize)>{
    for i in 0..9{
        for j in 0..9{
            if board[i][j] == 0{
                return Some((i,j));
            } 
        }
    }
    return None;
}

fn start_box(i: usize) -> usize{
    let x : usize;

    x = i;


    return x;
}

fn is_correct(board: [[i32; 9]; 9],i: usize,j: usize, num: i32) -> bool{
    // row
    for k in 0..j{
        if board[i][k] == num{
            return false;
        }
    }

    //column
    for k in 0..i{
        if board[k][j] == num{
            return false;
        }
    }

    //box
    // give start of x , y
    // 0 1 2 -> 0
    // 3 4 5 -> 3
    // 6 7 8 -> 6


    return true;
}

fn main() {
    
    // let mut board = [
    //     [3,0,8,0,0,0,0,9,0],
    //     [6,0,0,0,0,0,5,0,0],
    //     [0,4,0,0,0,7,0,0,0],
    //     [0,5,1,0,0,9,0,0,0],
    //     [0,0,0,0,0,0,0,0,0],
    //     [4,0,9,8,1,0,0,0,6],
    //     [0,1,0,3,0,8,0,0,0],
    //     [0,0,6,0,5,0,0,0,0],
    //     [5,0,0,4,7,1,0,0,3]
    // ];

    // draw(&board);

    for i in 0..9{

        println!("{} -> {}",i,start_box(i));
    }
}

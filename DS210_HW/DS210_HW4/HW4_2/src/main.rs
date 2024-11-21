fn main() {
    println!("Hello, world!");
    let size = 20;
    let first_cells = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
    let mut game = Grid::create(size, &first_cells);

    println!("First Stage:");
    game.print_board();

    for steps in 1..=20 {
        game.next_stage();
        println!("Step {}:", steps);
        game.print_board();
        println!("-------------");
    }
}

/*enum status {
    Alive,
    Dead,
}*/

struct Grid {
    board: Vec<Vec<bool>>,
    size: usize,
}

impl Grid {
    fn create(size: usize, initial_alive: &[(usize, usize)]) -> Self {
        let mut board = vec![vec![false; size]; size];
        for &(x, y) in initial_alive {
            board[x][y] = true;
        }
        return Grid{board, size};
    }

    fn next_stage(&mut self) {
        let mut new_board = self.board.clone();
        for i in 0..self.size {
            for j in 0..self.size {
                let still_alive = self.count_alive(i, j);
                if still_alive == 2 {
                    new_board[i][j] = self.board[i][j];
                } else if still_alive == 3 {
                    new_board[i][j] = true;
                } else {
                    new_board[i][j] = false;
                }   
                }
            };
        self.board = new_board;
        }

    fn count_alive(&self, x: usize, y: usize) -> usize {
        let mut cumulative_counts = 0;
        // check the neighbor's status by 
        for x_check in -1..=1 {
            for y_check in -1..=1 {
                if x_check == 0 && y_check == 0 {
                    continue;
                }
                //to prevent the overflow: isize? 
                let new_x = (((x as isize + x_check) + (self.size as isize)) % self.size as isize) as usize;
                let new_y = (((y as isize + y_check) + (self.size as isize)) % self.size as isize) as usize;
                if self.board[new_x][new_y] {
                    cumulative_counts += 1;
                }
            };
        }
        return cumulative_counts;
    }    
    fn print_board(&self) {
        //new board for the next step
        for row in &self.board {
            for &cell in row {
            print!("{}", if cell { "*" } else { "." });
        }
        println!(); // Move to the next line after each row
    }
}
}   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_alive() {
        let initial_cells = [(0,1), (1,0), (1,1), (1,2)];
        let mut game = Grid::create(4, &initial_cells);

        assert_eq!(game.count_alive(0,1), 3);
        assert_eq!(game.count_alive(1,1), 3);
        assert_eq!(game.count_alive(1,2), 2);
    }
}
    /*#[test]
    fn test_count_alive() {
        let mut game = Grid::create(4, &[(1,1),(3,1)]);
        assert_eq!(game.count_alive(1,1), 0);
        assert_eq!(game.count_alive(3,1), 0);
    }*/



//coordinates are for positions;
//in each coordinates, it shows alive or dead
//est target variable make one: (0,0)
//alive = true = 1
//each iteration, create new one(for clarity) or update; remove previous grid
/*impl Grid {
    let mut target = (x: usize, y: usize);
    Grid = //either Alive or Dead
    fn calculate_neighbors(&self, x: usize, y: usize) -> usize {
        if target(x+1, y) == 
            
        }
    }*/
//Vec<Vec<i64>>
//Code 20*20
//Vec<Vec<0;20>>
//Boolean to show live or dead(true: live; false: dead)
//Vec<Vec<bool>>
//Live cell's location
//let init_array = [(0,1), (1,2), (2,0), (2,1), (2,2)];
//Iteration
//for i in 1..=20;

/*match {
            if x > 20 {
                x = x % 20
            }
            else if y > 20 {
                y = y % 20
            }
            else {
                self.target = 
            }
        }
        match */

/*
If Sum(Neighbors) == 2 then Target stays as is
else If Sum(Neighbors) == 3 then Target becomes alive
else Target becomes dead
*/

//Calculation of liveness for a square
/*impl 

fn live_or_dead -> {
    if 
}*/
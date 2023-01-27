use std::io;
use std::fs;
use std::collections::HashMap;
use std::io::Write;

struct Game {
    board: Vec<Vec<i32>>,
    players: HashMap<i32, String>,
    current_player: i32,
    column_count: usize,
    row_count: usize,
}

impl Game {
    fn new() -> Self {
        println!("Enter the number of columns:");
        let mut column_input = String::new();
        io::stdin()
            .read_line(&mut column_input)
            .expect("Failed to read column input");
        let column_count: usize = column_input.trim().parse().expect("Please enter a valid column count");

        println!("Enter the number of rows:");
        let mut row_input = String::new();
        io::stdin()
            .read_line(&mut row_input)
            .expect("Failed to read row input");
        let row_count: usize = row_input.trim().parse().expect("Please enter a valid row count");

        let mut players = HashMap::new();
        players.insert(1, "X".to_string());
        players.insert(2, "O".to_string());

        let board = vec![vec![0; column_count]; row_count];

        Self {
            board,
            players,
            current_player: 1,
            column_count,
            row_count,
        }
    }

    fn play(&mut self) {
        let mut winner = None;

        while winner.is_none() {
            print!("{}[2J", 27 as char);
            self.print_board();

            let player_name = self.players.get(&self.current_player).unwrap();
            println!("{}'s turn", player_name);
            println!("Enter s to save!");

            let mut column = String::new();
            io::stdin()
                .read_line(&mut column)
                .expect("Failed to read column");

            if column.trim() == "s" {
                self.save();
                continue;
            }

            let column: usize = column.trim().parse().expect("Please enter a valid column");

            if !self.make_move(column) {
                println!("Invalid move! Please try again.");
                continue;
            }

            self.current_player = if self.current_player == 1 { 2 } else { 1 };

            winner = self.check_winner();
        }

        self.print_board();

        if let Some(player) = winner {
            println!("-----------------------------------");
            println!("{} wins!", self.players.get(&player).unwrap());
            println!("-----------------------------------");
        } else {
            println!("It's a draw!");
        }
    }

    fn make_move(&mut self, column: usize) -> bool {
        if column >= self.column_count {
            return false;
        }

        for row in (0..self.row_count).rev() {
            if self.board[row][column] == 0 {
                self.board[row][column] = self.current_player;
                return true;
            }
        }

        false
    }

    fn check_winner(&self) -> Option<i32> {

        for row in 0..self.row_count {
            let mut last_value = 0;
            let mut streak = 0;
    
            for col in 0..self.column_count {
                let value = self.board[row][col];
    
                if value != last_value || value == 0 {
                    last_value = value;
                    streak = 1;
                } else {
                    streak += 1;
                }
    
                if streak == 4 && value != 0 {
                    return Some(value);
                }
            }
        }
    
        for col in 0..self.column_count {
            let mut last_value = 0;
            let mut streak = 0;
    
            for row in 0..self.row_count {
                let value = self.board[row][col];
    
                if value != last_value || value == 0 {
                    last_value = value;
                    streak = 1;
                } else {
                    streak += 1;
                }
    
                if streak == 4 && value != 0 {
                    return Some(value);
                }
            }
        }
    
        for row in 0..self.row_count - 3 {
            for col in 0..self.column_count - 3 {
                let value = self.board[row][col];
    
                if value == 0 {
                    continue;
                }
    
                if self.board[row + 1][col + 1] == value
                    && self.board[row + 2][col + 2] == value
                    && self.board[row + 3][col + 3] == value
                {
                    return Some(value);
                }
            }
        }
    
        for row in 3..self.row_count {
            for col in 0..self.column_count - 3 {
                let value = self.board[row][col];
    
                if value == 0 {
                    continue;
                }
    
                if self.board[row - 1][col + 1] == value
                    && self.board[row - 2][col + 2] == value
                    && self.board[row - 3][col + 3] == value
                {
                    return Some(value);
                }
            }
        }
    
        None
    }

    fn print_board(&self) {
        for row in 0..self.row_count {
            print!("||");
            for col in 0..self.column_count {
                let value = self.board[row][col];
                let symbol = if value == 0 { " " } else { self.players.get(&value).unwrap() };
                print!(" {} |", symbol);
            }
            println!("|");
            for i in 0..self.row_count{
                print!("------");
            }
            println!();
        }
    }

    fn save(&self) {
        
        let mut data = String::new();

        data.push_str(&format!("{}\n", self.row_count));
        data.push_str(&format!("{}\n", self.column_count));
        data.push_str(&format!("{}\n", self.current_player));
        for row in self.board.iter() {
            for value in row.iter() {
                data.push_str(&format!("{} ", value));
            }
            data.push('\n');
        }

        fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open("save1.txt")
                .expect("Failed to create new file");

        fs::write("save1.txt", data)
            .expect("Failed to write to file");
    }
    
    fn load() -> Self {
        let data = fs::read_to_string("save1.txt")
            .expect("Failed to read file");

        let mut lines = data.lines();

        let row_count: usize = lines.next().unwrap().parse().unwrap();
        let column_count: usize = lines.next().unwrap().parse().unwrap();
        let mut current_player: i32 = lines.next().unwrap().parse().unwrap();

        let mut board = Vec::new();
        for _ in 0..row_count {
            let mut row = Vec::new();
            for value in lines.next().unwrap().split_whitespace() {
                row.push(value.parse().unwrap());
            }
            board.push(row);
        }

        let mut players = HashMap::new();
        players.insert(1, "X".to_string());
        players.insert(2, "O".to_string());

        Self {
            board,
            players,
            current_player,
            column_count,
            row_count,
        }
    }

}

fn main() {
    let mut game = Game::new();

    loop {
        println!("Hello User, And welcome to:");
        println!("----------------------------------------------------------------");
        println!("               C O N N E C T          F O U R                   ");
        println!("----------------------------------------------------------------");
        println!();
        println!();
        println!("Please select one of the given below: ");
        println!("1.   Play New Game");
        println!("2.   Load Game");
        

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: i32 = input.trim().parse().expect("Please enter a valid option");

        match input {
            1 => game.play(),
            2 => game = Game::load(),
            _ => println!("Invalid option! Please try again."),
        }
    }
}
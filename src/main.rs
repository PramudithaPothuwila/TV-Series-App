mod db_functions;

use std::io::Write;

fn main() {
    loop {
        let menu_option = menu();
        match menu_option {
            Menu::List => db_functions::list(),
            Menu::Add => db_functions::add(),
            Menu::Exit => break,
        }
    }
}

fn menu() -> Menu {
    println!("\n--- Menu ---");
    println!("1. List TV Series");
    println!("2. Add TV Series");
    println!("3. Exit");
    println!("-------------");
    print!("Enter : ");
    match std::io::stdout().flush() {
        Ok(_) => {},
        Err(error) => {
            println!("Error : {}", error);
            return menu();
        }
    }

    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {
            match line.trim().parse::<i32>() {
                Ok(data) => {
                    match data {
                        1 => Menu::List,
                        2 => Menu::Add,
                        3 => Menu::Exit,
                        _ => {
                            println!("\nInvalid input\n");
                            menu()
                        }
                    }
                },
                Err(_) => {
                    println!("\nInvalid input\n");
                    menu()
                }
            }
        },
        Err(error) => {
            println!("Error : {}", error);
            menu()
        }
    }
}

fn search() {
    println!("--- Search TV Series ---");
}
enum Menu {
    List,
    Add,
    Exit,
}
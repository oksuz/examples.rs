use std::io;
use std::io::Write;

use crate::todo::ToDoList;

pub struct AppMenu {
    todo_list: ToDoList,
}

impl AppMenu {
    fn create_new_todo(&mut self) {
        print!("enter: ");
        let _ = io::stdout().flush();
        let mut t = String::new();
        match io::stdin().read_line(&mut t) {
            Ok(_) => {
                let id = self.todo_list.create_new_todo(t.trim().to_string());
                println!("todo added {}", id);
            }
            _ => {
                println!("error on adding todo");
            }
        };
    }

    fn make_todo_done(&mut self) {
        print!("enter id: ");
        let _ = io::stdout().flush();
        let mut t = String::new();

        let result = match io::stdin().read_line(&mut t) {
            Ok(_) => match t.trim().parse() {
                Ok(n) => self.todo_list.mark_as_done(n),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        };

        if let Ok(_) = result {
            println!("todo is done")
        } else {
            println!("an error occured");
        }
    }

    fn list_todos(&self) {
        println!("{}", "=".to_string().repeat(20));
        self.todo_list
            .get_all()
            .iter()
            .for_each(|f| println!("{}) {} => {}", f.id, f.label, f.done));
        println!("{}", "=".to_string().repeat(20));
    }

    pub fn start_loop(mut self) {
        loop {
            println!("1. Create new ToDo");
            println!("2. Make ToDo Done");
            println!("3. List ToDos");
            println!("4. Exit");
            print!("Choose: ");
            let _ = io::stdout().flush();

            let mut choice_raw = String::new();

            match io::stdin().read_line(&mut choice_raw) {
                Ok(_) => match choice_raw.trim().parse() {
                    Ok(choice) => match choice {
                        1 => self.create_new_todo(),
                        2 => self.make_todo_done(),
                        3 => self.list_todos(),
                        4 => break,
                        _ => {
                            println!("invalid option");
                            continue;
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                    }
                },
                _ => {
                    println!("cannot capture the user input");
                }
            }
        }
    }
}

impl Default for AppMenu {
    fn default() -> AppMenu {
        AppMenu {
            todo_list: ToDoList::default(),
        }
    }
}

use self::notes::Notes;
use std::io::stdin;
use termion::{color, style};
fn main() {
    let path = String::from("notes.txt");
    let mut notes = Notes::new(path);
    print_menu();
    loop {
        let command = read_input();
        match command.trim() {
            "show" => {
                clear_screen();
                show_notes(&notes.list());
                print_menu();
            }
            "add" => {
                clear_screen();
                println!("input note: ");
                notes.add(read_input());
                print_menu();
            }
            _ => break,
        }
    }
}

fn print_menu() {
    println!();
    println!("{}", color::Fg(color::Cyan));
    println!("**** PROGRAM MENU ****");
    println!("Enter command: ");
    println!(
        "'{}show{}' - show all notes",
        color::Fg(color::Yellow),
        style::Reset
    );
    println!(
        "'{}add{}' - add new note",
        color::Fg(color::Yellow),
        style::Reset
    );
    println!("{}other{} - exit ", color::Fg(color::Red), style::Reset);
    println!("**********************");
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}
fn read_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn show_notes(notes: &Vec<String>) {
    for (i, note) in notes.iter().enumerate() {
        println!("{}.{note}", i + 1);
    }
}
mod notes {
    use std::io::Write;

    pub struct Notes {
        path: String,
    }

    impl Notes {
        pub fn new(path: String) -> Self {
            Self { path }
        }

        pub fn add(&mut self, new_note: String) {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.path)
                .unwrap();

            file.write_all(new_note.trim().as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        pub fn list(&self) -> Vec<String> {
            let file_content = std::fs::read_to_string(&self.path).unwrap_or(String::new());
            file_content.lines().map(|s| s.to_string()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Notes;

    #[test]
    fn add_note() {
        let path = String::from("test_notes1.txt");
        let mut notes = Notes::new(path);
        let note = String::from("Aboba");
        notes.add(note.clone());
        assert_eq!(&note, notes.list().last().unwrap())
    }

    #[test]
    fn notes_len() {
        const COUNT: usize = 10;
        let path = String::from("test_notes2.txt");
        let mut notes = Notes::new(path);
        for counter in 0..COUNT {
            notes.add(counter.to_string());
        }
        assert_eq!(notes.list().len(), COUNT);
    }
}

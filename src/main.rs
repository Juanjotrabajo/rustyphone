use std::io;
use regex::Regex;
use std::env;

struct PhoneEntry {
    name: String,
    number: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let exit_string = String::from("exit");
    let exit_regex = Regex::new(".*exit.*").unwrap();
    let print_regex = Regex::new(".*print.*").unwrap();


    println!("Phone number Agenda");
    let mut user_input = String::new();

    let phones_list = [626,434,545,657,434];
    let phoneentry1 = PhoneEntry {
        name: "Juan".to_string(),
        number: "626028667".to_string(),
    };
    let phoneentry2 = PhoneEntry {
        name: "Juana".to_string(),
        number: "626028627".to_string(),
    };
    let phone_entries_list = [phoneentry1,phoneentry2];

    let no_interactive = String::from("no_interactive");
//    let arg1 = args[0];
    println!("Argumento: {}",args[1]);
    if args[1] == no_interactive {
//        println!("No interactive");
        std::process::exit(1);
        //return Err ("Leaving app".into());
    }


    while !(user_input.to_lowercase().contains (&exit_string)) {
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    if print_regex.is_match (&user_input) {
        for element in &phone_entries_list {
            println!("Agenda entry: {} - Tel: {}",element.name, element.number);
                }  
    }          
    //if user_input.to_lowercase().contains (&exit_string) {
    if exit_regex.is_match (&user_input) {
        println!("Input was {user_input}, so closing the app");
    } else {
        println!("Input was {user_input}, continuing processing");
        user_input.clear();
    }
    }
}


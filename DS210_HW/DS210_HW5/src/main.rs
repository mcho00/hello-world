use std::fs::File;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet; 

fn main() {
    println!("Hello, world!");
    let file_name = "categories_ingredients.txt";
    let result = read_file(file_name);
    for (name, items) in &result {
        println!("Name: {}", name);
        println!("Items: {}", items: HashSet<String>);
    }
}

//1st: categories_ingredients.txt
//2nd: people_categories.txt
//3rd: recipes.txt

fn read_file(file_name: &str) -> HashMap<String, HashSet<String>> {
    let mut line_read: HashMap<String, HashSet<String>> = HashMap::new();
    let file = File::open(file_name).expect("Check the file location.");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_string = line.expect("Read Error");
        if let Some((name, items)) = line_string.split_once(": ") {
            let name = name.trim().to_string(); 
            let items: HashSet<String> = items.split(", ")
            .map(|item| item.trim().to_string()).collect();
            line_read.insert(name, items); 
        }
    }
    line_read
}


//three files routine
//~~~: ~~, ~~, ~~,

//except categories_ingredients: 
//~~~: ~~, ~~, ~~
//~~~: ~~, ~~, ~~


//b
/*
if <input> = 'popular recipes' {
    //produce list of 5 most popular recipes
}
//when recipes are liked equally --> select those whose name starts
//with a lteer that ranks the lowest ('a' is the lowest rank)
*/
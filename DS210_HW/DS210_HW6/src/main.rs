use std::fs::File;
use std::fs::read_to_string;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet; 

fn main() {
    println!("Hello, world!");
    let file_1st = "categories_ingredients.txt";
    let file_2nd = "people_categories.txt";
    let file_3rd = "recipes.txt";

    let categories = read_1st(file_1st);
    let people = read_2nd(file_2nd);
    let recipes = read_3rd(file_3rd);

    let ex_1a = like_recipe(
        "Halle Vaughn",
        "Recipe1",
        &categories,
        &people,
        &recipes,
    );

    let ex_1b = what_recipes(
        "Halle Vaughn",
        &categories,
        &people,
        &recipes,
    );

    let ex_2 = five_most(
        "popular recipes",
        &categories,
        &people,
        &recipes,
    );

    println!("{:?}", ex_1a);
    println!("{:?}", ex_1b);
    println!("{:?}", ex_2);

    
}

//1st: categories_ingredients.txt
//2nd: people_categories.txt
//3rd: recipes.txt

//Auxiliary Functions
//Function for 1st:
fn read_1st(file_name: &str) -> HashMap<String, HashSet<String>> {
    let mut line_read: HashMap<String, HashSet<String>> = HashMap::new();
    let file = File::open(file_name).expect("Check the file location.");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_string = line.expect("Read Error");
        if let Some((name, items)) = line_string.split_once(": ") {
            let name = name.trim().to_string().to_lowercase(); 
            let items: HashSet<String> = items.split(", ")
            .map(|item| item.trim().to_lowercase()).collect();
        line_read.insert(name, items); 
        }
    }
    line_read
}

//Function for 2nd:
fn read_2nd(file_name: &str) -> HashMap<String, HashSet<String>> {
    let mut line_read: HashMap<String, HashSet<String>> = HashMap::new();
    let file = File::open(file_name).expect("Check the file location.");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_string = line.expect("Read Error");
        if let Some((name, items)) = line_string.split_once(": ") {
            let name = name.trim().to_lowercase();
            let items: HashSet<String> = items.split(",")
            .map(|item| item.trim().to_string().to_lowercase()).collect();
        line_read.insert(name, items);
        }
    }
    line_read
}

//Function for 3rd: 
fn read_3rd(file_name: &str) -> HashMap<String, HashSet<String>> {
    let mut line_read: HashMap<String, HashSet<String>> = HashMap::new();
    let file = File::open(file_name).expect("Check the file location.");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_string = line.expect("Read Error");
        if let Some((name, items)) = line_string.split_once(": ") {
            let name = name.trim().to_lowercase(); 
            let items: HashSet<String> = items.split(",")
            .map(|item| item.trim().to_lowercase()).collect();
        line_read.insert(name, items); 
        }
    }
    line_read
}

//function for 1-a, 1-b, 2
//1-a) : find the set of food categories the person likes 
fn like_recipe(
    person: &str,
    recipe: &str,
    categories: &HashMap<String, HashSet<String>>,
    people: &HashMap<String, HashSet<String>>,
    recipes: &HashMap<String, HashSet<String>>,
) -> bool {
    let person = person.to_lowercase();
    let recipe = recipe.to_lowercase();
    if let (Some(person_subject), Some(favorite_ingredients)) = (people.get(&person), recipes.get(&recipe)) {
        let favorite_counts = favorite_ingredients.iter()
        .filter(|each_ingredient| {
            categories.iter()
                .any(|(category, ingredients)| person_subject.contains(&category.to_lowercase()) && ingredients.contains(&each_ingredient.to_lowercase()))
        })
            //favorite_ingredients.contains(&(each_ingredient as String)))
        .count();
        return favorite_counts >= favorite_ingredients.len()/2;
    }
    false
    }


//1-b) Person input, and print out what recipes that person likes
//what determines like? = if more than half of the the person's ingredient
//is listed in the recipe (so use the 1-a))?

fn what_recipes(
    person: &str,
    categories: &HashMap<String, HashSet<String>>,
    people: &HashMap<String, HashSet<String>>,
    recipes: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    let mut liked_lists = Vec::new();
    for recipe in recipes.keys() {
        if like_recipe(person, recipe, categories, people, recipes) {
        //as this will return true or false, 
            liked_lists.push(recipe.to_string());
        }
    }
    liked_lists
}

//2): if input name is "popular recipes", produce a 
//list of the five most popular recipes (=liked by most people)
//in case 
fn five_most(input: &str,
    categories: &HashMap<String, HashSet<String>>,
    people: &HashMap<String, HashSet<String>>,
    recipes: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    if input != "popular recipes" {
        return Vec::new();
    }
    let mut each_recipe_likes: HashMap<String, usize> = HashMap::new();
        //get ready for each recipe's liked number
    for person in people.keys() {
        for recipe in recipes.keys() {
            if like_recipe(person, recipe, categories, people, recipes) {
                //this shows the person actually likes the recipe or not
                *each_recipe_likes.entry(recipe.to_string()).or_insert(0) += 1; 
            }
        }
    }
    let mut recipes_final: Vec<_> = each_recipe_likes.into_iter().collect();
    recipes_final.sort_by(|a, b| {
        b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))
    });
    recipes_final.into_iter().take(5).map(|(recipe_name, _)| recipe_name).collect()
}

    

//ARCHIVES

/*let categories = read_file("categories_ingredients.txt", FileType::Categories);
    let people = read_file("people_categories.txt", FileType::People);
    let recipes = read_file("recipes.txt", FileType::Recipes);

    for (name, items) in &categories {
        println!("Name: {}", name);
        println!("Items: {:?}", items);
    }
    
    let result_1st = read_1st(file_1st);
    for (name, items) in &result_1st {
        println!("Name: {}", name);
        println!("Items: {:?}", items);
        //println!("Hashes: {:?}", result_1st);
    }
    
    let result_2nd = read_1st(file_2nd);
    for (name, items) in &result_2nd {
        println!("Name: {}", name);
        println!("Items: {:?}", items);
        //println!("Hashes: {:?}", result_2nd);
        //above runs infinitely while the 1st one doesn't; why is this?
    }

    let result_3rd = read_1st(file_3rd);
    for (name, items) in &result_3rd {
        println!("Name: {}", name);
        println!("Items: {:?}", items);
        //println!("Hashes: {:?}", result_3rd);
        //this code shows random result output; 
    }*/
    //don't call these, (these are needed for storing the data)


/*enum FileType {
    Categories,
    People,
    Recipes,
}


//read file
/*//.map transforms each item in an iterator
        //|item|: one argument which represents each item in the iterator
        //.trim(): removes whitespace
        //.collect(): gathers all the results from map and combines
        //into a new collection type */
fn read_file(file_name: &str, file_type: FileType) -> HashMap<String, HashSet<String>> { 
    let mut line_read: HashMap<String, HashSet<String>> = HashMap::new();
    let file = File::open(file_name).expect("Check the file location.");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_string = line.expect("Read Error");
        let divider = match file_type{
            FileType::Categories => ", ",
            FileType::People => ",",
            FileType::Recipes => ",",
        };
        if let Some((name, items)) = line_string.split_once(": ") {
            let name = name.trim().to_string(); 
            let items: HashSet<String> = items.split(",")
            .map(|item| item.trim().to_string()).collect();
        line_read.insert(name, items); 
        }
    }
    line_read
}*/

//1-a)
/*//1-a) 해당 레시피에 절반 이상 그 사람이 좋아하는 ingredients 있으면 name이랑 recipe 도출 */
//input name = 'popular recipes' --> produce a list of 5 most popular recipes

//fn read_3rd(file_name: &str) -> 

//three files routine
//~~~: ~~, ~~, ~~,

//except categories_ingredients: 
//~~~: ~~, ~~, ~~
//~~~: ~~, ~~, ~~


//b
/*
//let mut count = 0;
    //when look at recipe,
    //if more than half ingredients is in the person's ingredient list,
    //the person likes it. 
if <input> = 'popular recipes' {
    //produce list of 5 most popular recipes
}
//when recipes are liked equally --> select those whose name starts
//with a lteer that ranks the lowest ('a' is the lowest rank)

//let mut favorite_counts = favorite_ingredients.len() / 2;
        //let match_count = 
    }
    //let match_count = 
    //let mut number_of_ingredients = 
*/

//2)
/*let mut five_recipes = Vec::new();
    //count how many people like each recipe. 
    
    //use hashmap (key: recipe; value: counts of how many people like recipes)
    //iterate over people
   for person in people.keys() {

    }
    //sorting: after getting counts, sort the recipes based on the counts; but need to sort by count first and then by name in case of ties
    //after sort, slice the sorted list to get the top five recipes
    
}*/
use std::error::Error;
use std::fmt;
use std::process;

#[derive(Debug, Clone)]
enum ColumnVal {
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

#[derive(Debug)]
struct DataFrame {
    // Need to implement
    columns: 
    num_rows; usize,
}

// For returning errors
#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

impl DataFrame {
    fn new() -> Self {
        unimplemented!();
    }

    fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;
        let mut first_row = true;
        for result in rdr.records() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let r = result.unwrap();
            let mut row: Vec<ColumnVal> = vec![];
            if first_row {
                for elem in r.iter() {
                    // These are the labels
                }
                first_row = false;
                continue;
            }
            for (i, elem) in r.iter().enumerate() {
                match types[i] {
                    1 => row.push(ColumnVal::One(elem.to_string())),
                    2 => row.push(ColumnVal::Two(elem.parse::<i64>().unwrap() != 0)),
                    3 => row.push(ColumnVal::Three(elem.parse::<f64>().unwrap())),
                    4 => row.push(ColumnVal::Four(elem.parse::<i64>().unwrap())),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }
            }
            // Put the data into the dataframe
        }
        Ok(())
    }

    fn unimplemented() {}

    fn print(&self) {
        // print the labels
        println!();
        // print the data
    }

    fn add_column(&mut self) -> () {
        unimplemented!();
    }

    // Need to add more function parameters and fix the return type
    fn merge_frame(&mut self) -> () {
        unimplemented!();
    }

    // Need to add more function parameters and fix the return type
    fn find_columns(&mut self) -> () {
        unimplemented!();
    }

    // Need to add more function parameters and fix the return type
    fn restrict_columns(&mut self) -> () {
        unimplemented!();
    }

    fn filter(
        &mut self,
        label: &str,
        operation: fn(&ColumnVal) -> bool,
    ) -> Result<Self, Box<dyn Error>> {
        unimplemented!();
    }

    fn column_op(
        &mut self,
        labels: &[String],
        op: fn(&[Vec<ColumnVal>]) -> Vec<ColumnVal>,
    ) -> Vec<ColumnVal> {
        unimplemented!();
    }
}

fn main() {}

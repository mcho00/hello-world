use std::error::Error;
use std::fmt;
use std::process;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug, Clone)]
enum ColumnVal {
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

#[derive(Debug, Clone)]
struct DataFrame {
    // Need to implement
    labels: Vec<String>,
    data: Vec<Vec<ColumnVal>>,
    column_types: Vec<u32>,
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
    fn new(labels: Vec<String>, column_types: Vec<u32>) -> Self {
        DataFrame {
            labels,
            data: Vec::new(),
            column_types,
        }
        //let data = labels.iter().map(|label| (label.clone(), vec![])).collect();
        //DataFrame {labels, data}
        //unimplemented!();
    }

    fn read_csv(&mut self, path: &str, column_types: Vec<u32>) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;
        //added to assign each instances to specific role
        self.column_types = column_types;
        self.data.clear();
        self.labels.clear();
        
        let mut first_row = true;
        //loop the rdr by result in one
        for result in rdr.records() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            //I didn't put anything for type hint??????????????????
            //let record = result?;
            //above one can be replaced to the below one
            let each_result = result?;
            //let r = result.unwrap();
            let mut row: Vec<ColumnVal> = Vec::new();
            if first_row {
                for element in each_result.iter() {
                    // These are the labels
                    self.labels.push(element.to_string());
                }
                first_row = false;
                continue;
            }
            for (i, element) in each_result.iter().enumerate() {
                match self.column_types[i] {
                    //some changes, but not reflected in this code
                    1 => row.push(ColumnVal::One(element.to_string())),
                    2 => row.push(ColumnVal::Two(element.parse::<i64>().unwrap() != 0)),
                    3 => row.push(ColumnVal::Three(element.parse::<f64>().unwrap())),
                    4 => row.push(ColumnVal::Four(element.parse::<i64>().unwrap())),
                    _ => return Err(Box::new(MyError("Type unknown".to_string()))),
                }
                //row.push(column_val);
            }
            self.data.push(row);
            // Put the data into the dataframe
            /*for (i, value) in row.into_iter().enumerate() {
                self.columns[i].1.push(value); 
            }
            self.row_count += 1;*/
        }
        Ok(())
    }

    fn unimplemented() {}
    //why does this exist

    fn print(&self) {
        for label in &self.labels {
            print!("{}\t", label);
        }
        println!();
        for row in &self.data {
            for each_value in row {
                match each_value {
                    ColumnVal::One(val) => print!("{}\t", val),
                    ColumnVal::Two(val) => print!("{}\t", val),
                    ColumnVal::Three(val) => print!("{}\t", val),
                    ColumnVal::Four(val) => print!("{}\t", val),
                }
            }
            println!();
        }
        // print the data
    }
        /*
        //loop the row upto row_count (I made)
        for row in 0..self.row_count {
            for (_, column) in &self.columns {
                match &column[row] {
                    ColumnVal::One(val) => print("{:<15}", val),
                    ColumnVal::Two(val) => print("{:<15}", val),
                    ColumnVal::Three(val) => print("{:<15}", val),
                    ColumnVal::Four(val) => print("{:<15}", val),
                }
            }
            println!();
        }*/
    

    //edited the output type from () to below, and added some fields that function requires
    fn add_column(&self, new_name: String, column_types: Vec<ColumnVal>) -> Result<DataFrame, Box<dyn Error>> {
        if column_types.len() != self.data.len() {
            return Err(Box::new(MyError("Column length != DataFrame's number of rows".to_string())));
        }
        let mut new_df = self.clone();
        new_df.labels.push(new_name);
        for (i, one_row) in new_df.data.iter_mut().enumerate() {
            one_row.push(column_types[i].clone());
        }
        Ok(new_df)
    }

    // Need to add more function parameters and fix the return type
    // added other, and changed output type from () to below
    fn merge_frame(&self, new_merge: &DataFrame) -> Result<DataFrame, Box<dyn Error>> {
        if self.labels != new_merge.labels || self.column_types != new_merge.column_types {
            return Err(Box::new(MyError("Two DataFrames are not matched.".to_string())));
        }

        let mut new_df = self.clone();
        for each_row in &new_merge.data {
            new_df.data.push(each_row.clone());
        }
        Ok(new_df)
        /*let mut merged_data = self.data.clone();
        merged_data.extend_from_slice(&other.data);*/
        
        /*for (i, columns) in other.columns.iter().enumerate() {
            if column.0 != self.columns[i].0 {
                return Err(Box::new(MyError("Column names not match".to_string())));
            }
            new_columns[i].1.extend_from_slice(&column.1);
        }*/

        /*Ok(DataFrame {
            data: merged_data,
            labels: self.labels.clone(),
            types: self.types.clone(),
        })
        //unimplemented!();*/
    }

    // Need to add more function parameters and fix the return type
    //added labels, and output type from () to below
    fn find_columns(&mut self, target_columns: Vec<&str>) -> Result<DataFrame, Box<dyn Error>> {
        let mut index = Vec::new();
        for column in target_columns {
            match self.labels.iter().position(|label| label == column) {
                Some(one_index) => index.push(one_index),
                None => return Err(Box::new(MyError(format!("Column '{}' - not exist", column)))),
            }
        }
        //create new sections inside the dataframe
        let new_label = index.iter().map(|&i| self.labels[i].clone()).collect();
        let new_column_type = index.iter().map(|&i| self.column_types[i]).collect();
        let new_data = self
            .data.iter()
            .map(|row| index.iter().map(|&i| row[i].clone()).collect()).collect();

        Ok(DataFrame {
            labels: new_label,
            data: new_data,
            column_types: new_column_type,
        })
    }
        //unimplemented!();

    // Need to add more function parameters and fix the return type
    // added labels, and return from ()
    //this function will help 
    fn restrict_columns(&mut self, restrict_columns: Vec<&str>) -> Result<DataFrame, Box<dyn Error>> {
        let mut index = Vec::new();
        for column in restrict_columns {
            match self.labels.iter().position(|label| label == column) {
                Some(one_index) => index.push(one_index),
                None => return Err(Box::new(MyError(format!("Column {} - not exist", column)))),
            }
        }
        //
        let new_label = index.iter().map(|&i| self.labels[i].clone()).collect();
        let new_column_type = index.iter().map(|&i| self.column_types[i]).collect();
        let new_data = self.data.iter().map(|row| index.iter().map(|&i| row[i].clone()).collect()).collect();

        Ok(DataFrame {
            labels: new_label,
            data: new_data,
            column_types: new_column_type,
        })
        //unimplemented!();
    }

    //added <F>, and changed operation from fn(&ColumnVal)
    //changed output to DataFrame from Self
    fn filter<F>(&self, label: &str, operation: F) -> Result<DataFrame, Box<dyn Error>>
    where
        F: Fn(&ColumnVal) -> bool,
    {
        let index_of_column = match self.labels.iter().position(|each_label| each_label == label) {
            Some(index) => index,
            None => return Err(Box::new(MyError(format!("{} column not exist", label)))),
        };

        let refreshed_data: Vec<Vec<ColumnVal>> = self.data.iter()
            .filter(|row| operation(&row[index_of_column])).cloned().collect();

        Ok(DataFrame {
            labels: self.labels.clone(),
            data: refreshed_data,
            column_types: self.column_types.clone(),
        })
    }

    fn column_op(
        &mut self,
        labels: &[String],
        op: fn(&[Vec<ColumnVal>]) -> Vec<ColumnVal>,
    ) -> Vec<ColumnVal> {
        let index_of_columns: Vec<_> = labels.iter().filter_map(|label| self.labels.iter().position(|l| l == label))
        .collect();
        let selected_columns: Vec<_> = index_of_columns.iter()
        .map(|&i| self.data.iter().map(|row| row[i].clone()).collect())
        .collect();

        op(&selected_columns)
        //unimplemented!();
    }

    fn average(&mut self, target_label: &str) -> Result<f64, Box<dyn Error>> {
        let op = |columns: &[Vec<ColumnVal>]| -> Vec<ColumnVal> {
            let selected_column = &columns[0];
            let sum: f64 = selected_column.iter().filter_map(|val| match val {
                ColumnVal::Three(value) => Some(*value),
                ColumnVal::Four(value) => Some(*value as f64),
                _ => None,
            }).sum();

            let count = selected_column.iter().filter(|value| matches!(value, ColumnVal::Three(_) | ColumnVal::Four(_))).count();
            vec![ColumnVal::Three(sum / count as f64)]
        };

        let result = self.column_op(&[target_label.to_string()], op);
        match result.get(0) {
            Some(ColumnVal::Three(avg)) => Ok(*avg),
            _ => Err(Box::new(MyError("Invalid column type for average calculation".to_string()))),
        }
    }

    fn add_rows(&mut self, label1: &str, label2: &str) -> Result<Vec<ColumnVal>, Box<dyn Error>> {
        let op = |columns: &[Vec<ColumnVal>]| -> Vec<ColumnVal> {
            let (first_column, second_column) = (&columns[0], &columns[1]);
            first_column.iter().zip(second_column).map(|(first_value, second_value)| match (first_value, second_value) {
                (ColumnVal::Three(val1), ColumnVal::Three(val2)) => ColumnVal::Three(val1 + val2),
                (ColumnVal::Four(val1), ColumnVal::Four(val2)) => ColumnVal::Four(val1 + val2),
                (ColumnVal::Three(val1), ColumnVal::Four(val2)) => ColumnVal::Three(val1 + *val2 as f64),
                (ColumnVal::Four(val1), ColumnVal::Three(val2)) => ColumnVal::Three(*val1 as f64 + val2),
                _ => ColumnVal::One("Invalid types for addition".to_string()),
            }).collect()
        };
        let result = self.column_op(&[label1.to_string(), label2.to_string()], op);
        Ok(result)
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // Define column types: 1 = String, 4 = i64, 3 = f64, 2 = bool
    let column_types = vec![1, 4, 3, 4, 4, 2]; 

    // 1. read_csv Test
    let mut df = DataFrame {
        labels: Vec::new(),
        data: Vec::new(),
        column_types: Vec::new(),
    };
    df.read_csv("pizza.csv", column_types.clone())?;
    println!("CSV converted to DataFrame:");
    df.print();

    // 2. "add_column" Test
    let the_best_column = vec![
        ColumnVal::Two(true),
        ColumnVal::Two(false),
        ColumnVal::Two(true),
        ColumnVal::Two(true),
        ColumnVal::Two(true),
    ];
    let df_added_column = df.add_column("Hall_of_fame".to_string(), the_best_column)?;
    println!("\nDataFrame - After adding 'Hall_of_fame' column:");
    df_added_column.print();

    // 3. "merge_frame" Test
    let mut second_df = DataFrame {
        labels: Vec::new(),
        data: Vec::new(),
        column_types: Vec::new(),
    };
    second_df.read_csv("pizza2.csv", column_types.clone())?;
    let merged_df = df.merge_frame(&second_df)?;
    println!("\nDataFrame - After merge:");
    merged_df.print();

    // 4. 'find_columns' Test
    let find_col_df = df.find_columns(vec!["Name", "PPG"])?;
    println!("\nDataFrame - 'Name' and 'PPG' columns:");
    find_col_df.print();

    // 5. "restrict_columns" Test
    let restrict_df_test = df.restrict_columns(vec!["Name", "YearBorn"])?;
    println!("\n Restricted DataFrame with Name and Yearborn:");
    restrict_df_test.print();

    // 6. 'filter' Test
    let filter_df = df.filter("LikesPizza", |val| match val {
        ColumnVal::Two(true) => true,
        _ => false,
    })?;
    println!("\n DataFrame - Who likes pizza:");
    filter_df.print();
    /*fn likes_pizza(val: &ColumnVal) -> bool {
        match val {
            ColumnVal::Two(true) => true,
            _ => false,
        }
    }
    let filtered_df = df.filter("LikesPizza", likes_pizza)?;
    println!("\nDataFrame with players who like pizza:");
    filtered_df.print();*/

    // 7. 'column_op Test
    fn average_op(columns: &[Vec<ColumnVal>]) -> Vec<ColumnVal> {
        let sum: f64 = columns[0]
            .iter()
            .filter_map(|val| match val {
                ColumnVal::Three(value) => Some(*value),
                ColumnVal::Four(value) => Some(*value as f64),
                _ => None,
            })
            .sum();
        let count = columns[0].len();
        vec![ColumnVal::Three(sum / count as f64)]
    }
    //commit the average from the average_op help
    let indirect_avg_ppg = df.column_op(&["TotalPoints".to_string()], average_op);
    println!("\n Average PPG from average_op help: {:?}", indirect_avg_ppg);

    // 8. "avg" Test directly
    let direct_avg_ppg = df.average("TotalPoints")?;
    println!("\n Average PPG directly from function avg: {}", direct_avg_ppg);

    // 9. 'add_rows' Test
    let row_sums = df.add_rows("PPG", "TotalPoints")?;
    println!("\n Summation of rows of PPG and Total Points: {:?}", row_sums);

    Ok(())
}

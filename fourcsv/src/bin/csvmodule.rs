use csv::{Reader, StringRecord};
use std::collections::HashMap;
use std::path::Iter;
use std::{error::Error, io, process};
use std::io::{BufRead, Read};
use std::fs::File;

fn example(csv_name: File) -> Result<(), Box<dyn Error>> {
    let mut rdr:Reader<File> = csv::Reader::from_reader(csv_name.into());
    let headers: Vec<String> = rdr.headers()?.iter().map(|c| c.to_string()).collect();
    let mut hashmp: HashMap<String, Vec<String>> = HashMap::new(); 
    
     
    for header in &headers {
        hashmp.insert(header.clone(), vec![]);
    }
 
    for result in rdr.records() {
        let r = result?;
        
        for (header, field) in headers.iter().zip(r.iter()) {
            if let Some(value) = hashmp.get_mut(header) {
                value.push(field.to_string());
            }
        }
    }
    
    println!("{:?}", hashmp);
    
    let find_by_name = |name: &str| {
        if let Some(res) = hashmp.get(name) {
            println!("---- {:?} ----", name);
            for v in res {
                println!("| {:<30} |", v);
            }
        }
    };
    
   let field_iterator = hashmp.iter().map(|(header, field)| {
        format!("| {:?} \n| {:?} |\n", header, find_by_name(header));
    }); 
    
    let print_lines = || {
        for formatted_string in field_iterator {
            print!("\n{:?}", formatted_string);
        }
    };
    
    //find_by_name("Email");
    print_lines();
    Ok(()) 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut user_input = String::new();
     
    println!("type input: ");
    let mut buffer = [0; 1]; //reading to buffer -> valide the byte.
    let stdin = io::stdin(); 
    let mut handle = stdin.lock(); 
    
    loop {
        match handle.read_exact(&mut buffer) {
            Ok(_) => { 
                let character = buffer[0] as char;
                if character == '\n' {break;}
                user_input.push(character);
            }
            Err(_) => break,
        }
    }
    
    println!("looking for: {}", user_input);
    let file_name = File::open(&user_input)?;
    example(file_name);

    return Ok(());
}
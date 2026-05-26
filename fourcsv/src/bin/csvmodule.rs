use crossterm::style::Print;
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
    
   let get_size: Result<usize, Box<dyn Error>> = (|| {
        let size_headers = rdr.headers()?.len();
        Ok(size_headers) // Wrap in Ok() to match the Result type
    })(); 
    
    let mut incrementer: usize = 0;
    let mut keys: Vec<usize> = vec![];
    let mut fields: Vec<String> = vec![];
    let mut indexed_map: HashMap<usize, Vec<String>> = HashMap::new(); 
    
    let mut find_by_name = |name: &str| {
        //let mut lines: HashMap<Vec<String>, Vec<String>> = HashMap::new();
        
        if let Ok(field_size) = get_size {
            if let Some(headers) = hashmp.get(name) {
                for (index, head) in headers.iter().enumerate() {
                    //println!("{:?} | {:?}", index, field_size);
                    
                    if index % field_size == 0 {
                        keys.push(incrementer); 
                        
                        //println!("{:?} | {:?} <<[{}]", index, field_size, incrementer);
                        incrementer = incrementer + 1;
                    } else {
                        fields.push(head.into()); 
                        indexed_map.entry(index).or_insert_with(Vec::new).push(head.into());
                    }
            }
            
            
        }
        }
        let mut sorted_elements: Vec<(&usize, &Vec<String>)> = indexed_map.iter().collect();
        
        sorted_elements.sort_by_key(|&(key, _)| key);
        
        let cell_width = 4; 
        println!("\n┌─ Data Grid ───────────────────────────────────┐");
        
        for (key, values) in sorted_elements {
            // Print the Excel-like row index (e.g., Row #1)
            print!("| Row {:<3} | ", key);
            
            // Print each cell inside the row, padding it to 'cell_width' characters
            for cell in values {
                // The '<' aligns text to the left. The 'width' forces fixed spacing.
                print!("{:<width$} | ", cell, width = cell_width);
            }
            println!(); // Move to the next line
        }
        
        println!("└────────────────────────────────────────────────────────┘"); 
    };
    
     
    let field_iterator = hashmp.iter().map(|(header, field)| {
            find_by_name(header);
    }); 
    
    let print_lines = || {
        for formatted_string in field_iterator {
            print!("\n{:?}", formatted_string);
        }
    };
    
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
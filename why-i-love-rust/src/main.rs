#![allow(unused)]

use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use thiserror::Error;

#[derive(Debug)]
struct CustomerData {
    id: u32,
    name: String,
    revenue: Decimal,
}

trait HasRevenue {
    fn revenue(&self) -> Decimal;
}

impl HasRevenue for CustomerData {
    fn revenue(&self) -> Decimal {
        self.revenue
    }
}

impl HasRevenue for Decimal {
    fn revenue(&self) -> Decimal {
        *self
    }
}

fn main() {
    let filename = "data.csv";
    let content = read_file(filename);
    println!("File content:\n{}", content);

    match parse_csv_lines(&content) {
        Ok(customers) => {
            // Get average revenue
            let total_revenue: Decimal = customers.iter().map(|c| c.revenue).sum();
            let average_revenue = total_revenue / Decimal::from(customers.len());
            println!("Average revenue: {}", average_revenue);
        }
        Err(e) => {
            println!("Error parsing CSV: {:?}", e);
        }
    }

    let mut data: Vec<Box<dyn HasRevenue>> = parse_csv_lines(&content)
        .unwrap()
        .into_iter()
        .map(|c| Box::new(c) as Box<dyn HasRevenue>)
        .collect();
    data.push(Box::new(dec!(42.0)));
    let total_revenue: Decimal = data.iter().map(|d| d.revenue()).sum();
    println!("Total revenue: {}", total_revenue);
}

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    if content.trim().is_empty() {
        "".to_string()
    } else {
        content
    }
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("Error parsing int")]
    ParseIntError(#[from] ParseIntError),
    #[error("Error parsing decimal")]
    ParseDecimalError(#[from] rust_decimal::Error),
    #[error("Invalid number of fields")]
    InvalidNumberOfFields,
}

fn parse_csv_line(line: &str) -> Result<CustomerData, ParseError> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 3 {
        return Err(ParseError::InvalidNumberOfFields);
    }

    let id: u32 = parts[0].parse::<u32>()?;
    let name = parts[1].to_string();
    let revenue: Decimal = parts[2].parse()?;
    Ok(CustomerData { id, name, revenue })
}

#[derive(Debug)]
struct ParseFileError {
    line: usize,
    error: ParseError,
}

fn parse_csv_lines(content: &str) -> Result<Vec<CustomerData>, ParseFileError> {
    content.lines().skip(1).enumerate().map(|(i, line)| {
        parse_csv_line(line).map_err(|e| ParseFileError { line: i + 1, error: e })
    }).collect()
}

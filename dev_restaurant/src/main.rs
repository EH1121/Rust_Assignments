use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path:: PathBuf
};
use structopt::StructOpt;
use thiserror::Error;
use chrono;

// Struct for items
// Consist of: ID, Item, Quantity, and Price
#[derive(Clone, Debug, PartialEq)]
struct Item{
    id: i64,
    name: String,
    quantity: i64,
    price: i64
}

struct Items{
    list: HashMap<i64, Item>
}

#[derive(Clone)]
struct Report{
    // Date: Taken from Chrono in YYYY-MM-DD, then to String
    id: i64,
    date: String,
    quantity: i64,
    income: i64
}

struct Reports{
    list: HashMap<i64, Report>
}

#[derive(Error, Debug, PartialEq)]
enum ItemError{
    #[error("value must be a number {0}")]
    InvalidValue(#[from] std::num::ParseIntError),

    #[error("makanan tidak ditemukan {0} not found")]
    ItemNotFound(String),

    #[error("not enough stock, {0} < {0}")]
    NotEnoughStock(i64, i64),
}

#[derive(Error, Debug)]
enum ReportError{
    #[error("value must be a number {0}")]
    InvalidValue(#[from] std::num::ParseIntError),

    #[error("report tidak ditemukan")]
    ReportNotFound(String),
}


impl Items{
    fn new() -> Self{
        Self{
            list: HashMap::new()
        }
    }

    fn find_item(&self, name: &str) -> Result<Item, ItemError>{
        let x = name.to_lowercase();
        for (_, item) in &self.list{
            if item.name.to_lowercase() == x{
                return Ok(item.clone());
            }
        }
        Err(ItemError::ItemNotFound(name.to_string()))
    }

    fn add(&mut self, name: &str, quantity: i64, price: i64, id: i64, from_file: bool){
        let mut new_id: i64 = 0;
        
        // If being read from file
        if from_file {
            new_id = id;

            // If not from file and there is existing item in hashmap
        } else if !self.list.is_empty(){
            let max_id =  self.list.keys().max().unwrap();
            new_id = *max_id + 1;
        };  

        // Creates new item entry with the item id
        let new_item = 
            Item{
                id: new_id,
                name: name.to_string().to_lowercase(),
                quantity: quantity,
                price: price,
            };

        self.list.insert(new_item.id, new_item);
    }

    // Sets quantity and price of item
    fn update(&mut self, id: i64, name: &str, quantity: i64, price: i64){
        let x = Item{
            id,
            name: name.to_string(),
            quantity,
            price,
        };
        self.list.insert(x.id, x);
    }

    // Helper function for adding new or updating existing items
    fn add_or_update(&mut self, name: &str, quantity: i64, price: i64){
        match self.find_item(name){
            // if item exists
            Ok(item) => {
                self.update(item.id, &item.name, quantity, price);
            },
            // if item does not exist
            Err(_e) => {
                self.add(name, quantity, price, 0, false);
            },
        }
    }

    // Uses find_item, if item is found, then go to update, with reduction in quantity of the specified item
    fn buy(&mut self, name: &str, quantity: i64) -> Result<i64, ItemError>{
        match self.find_item(name){
            Ok(e) => {
                if e.quantity >= quantity{
                    self.update(e.id, &e.name, e.quantity - quantity, e.price);
                    return Ok(e.price);
                }
                // If quantity is lower than stock
                Err(ItemError::NotEnoughStock(e.quantity, quantity))
            }
            Err(_) => Err(ItemError::ItemNotFound(name.to_string())),
        }
    }

    // Deletes a key without doing anything else
    fn delete(&mut self, name: &str) -> Result<i32, ItemError>{
        match self.find_item(name){
            Ok(x) => {
                self.list.remove(&x.id);
                Ok(1)
            },
            Err(_e) => {
                Err(ItemError::ItemNotFound(name.to_string()))
            }
        }
    }

    // For saving
    fn get_item_list(self) -> Vec<Item>{
        let mut items: Vec<_> = self.list.into_values().collect();
        items.sort_by_key(|item| item.id);
        items
    }

    fn repeat_char(times: usize, ch: char) -> String{
        let mut str = String::new();
        for _ in 0..times{
            str.push(ch);
        }
        str
    }

    // Helper function to print all items
    fn print_items(self){
        if self.list.is_empty(){
            println!("Data kosong atau file tidak ditemukan");
            return;
        }
        println!("ID |      Name      |      Stock      |   Price");
        for item in &self.get_item_list(){
            let name_length = "      Name      ".len() - item.name.len() - 1;
            let quantity_length = "      Stock      ".len() - item.quantity.to_string().len() - 1;
            println!("{}  | {}{}| {}{}| Rp.{}", item.id, item.name, Items::repeat_char(name_length, ' '),  item.quantity, Items::repeat_char(quantity_length, ' '), item.price)
        }
    }
}

#[allow(unused_assignments)]
impl Reports{
    fn new() -> Self{
        Self{
            list: HashMap::new()
        }
    }

    // Helper function to search for a report
    fn find_report(&self, date: &str) -> Result<Report, ReportError>{
        for (_, report) in &self.list{
            if report.date.to_lowercase() == date.to_string().to_lowercase(){
                return Ok(report.clone());
            }
        }
        Err(ReportError::ReportNotFound(date.to_string()))
    }

    // Either add a new date entry or update existing, with the actual update being the quantity and income being added
    fn add_or_update(&mut self, date: &str, quantity: i64, income: i64, id: i64, from_file: bool){
        match self.find_report(date) {
            Ok(report) => {
                // Update Existing report
                let actual_income = quantity * income;

                let updated_report = 
                    Report{ 
                        id: report.id, 
                        date: report.date,
                        quantity: report.quantity + quantity, 
                        income: report.income + actual_income,
                    };

                self.list.insert(report.id, updated_report);
            },

            Err(_) => {
                // New Entry
                let mut x = income;
                let mut tmp = &id;
                let mut new_id: i64 = id;

                if from_file{
                    // If being read from file
                    new_id = id;
                } else
                if !self.list.is_empty(){
                    // If list has entries
                    tmp = self.list.keys().max().unwrap();
                    new_id = *tmp + 1;
                    x = x * quantity;
                } else {
                    // If list does not have anything
                    new_id = 0;
                    x = x * quantity;
                }

                let new_report = 
                    Report{ 
                        id: new_id, 
                        date: date.to_string(),
                        quantity, 
                        income: x,
                    };
                self.list.insert(id, new_report);
            },
        }
    }

    // Helper function for saving
    fn get_report_list(self) -> Vec<Report>{
        let mut reports: Vec<_> = self.list.into_values().collect();
        reports.sort_by_key(|rep| rep.id);
        reports
    }

    fn repeat_char(times: usize, ch: char) -> String{
        let mut str = String::new();
        for _ in 0..times{
            str.push(ch);
        }
        str
    }

    // Helper function to print all reports
    fn print_reports(self){
        if self.list.is_empty(){
            println!("Data kosong atau file tidak ditemukan");
            return;
        }
        println!("ID |      Date      |   Quantity   |   Income");
        for report in &self.get_report_list(){
            let date_length = "      Date      ".len() - report.date.len() - 1;
            let quantity_length = "   Quantity   ".len() - report.quantity.to_string().len() - 1;
            println!("{}  | {}{}| {}{}| Rp.{} ", report.id, report.date, Reports::repeat_char(date_length, ' '), report.quantity, Reports::repeat_char(quantity_length, ' '), report.income);
        }
    }
}

#[derive(Error, Debug)]
enum ParseError{
    #[error("value must be a number {0}")]
    InvalidValue(#[from] std::num::ParseIntError),

    #[error("empty item")]
    EmptyItem,

    #[error("missing field {0}")]
    MissingField(String)
}


// Item
fn parse_line(buffer: &str, n2: &str, n3: &str, n4: &str) -> Result<(i64, String, i64, i64), ParseError>{
    let fields: Vec<&str> = buffer.split(',').collect();
    
    // ID
    let f1 = match fields.get(0){
        Some(id) => i64::from_str_radix(id, 10)?,
        None => return Err(ParseError::EmptyItem),
    };
    
    // Name(Item) / Date(Report)
    let f2 = match fields.get(1){
        Some(name) => name.to_string(),
        None => return Err(ParseError::MissingField(n2.to_string())),
    };

    // Quantity
    let f3 = match fields.get(2){
        Some(quantity) => i64::from_str_radix(quantity, 10)?,
        None => return Err(ParseError::MissingField(n3.to_string())),
    };
    
    // Price(Item) / Income(Report)
    let f4 = match fields.get(3){
        Some(price) => i64::from_str_radix(price.trim(), 10)?,
        None => return Err(ParseError::MissingField(n4.to_string())),
    };

    Ok((f1, f2, f3, f4))
}

// name, quantity, price
fn parse_items(buffer: String, verbose: bool) -> Items{
    let mut items = Items::new();
    
    // Read each line
    for (line_number, item) in buffer.split('\n').enumerate(){
        // Not empty line
        if item != ""{
            // Parse each line
            match parse_line(item, "name", "quantity", "price"){
                // Add to items data
                Ok((item_id, item_name, item_quantity, item_price)) => {
                    if verbose {
                        println!("Adding {} | {} | {} | {} to item list", item_id, item_name, item_quantity, item_price);
                    };
                    items.add(
                        &item_name,
                        item_quantity,
                        item_price,
                        item_id,
                        true
                    );
                    },
                // Ignore line if error
                Err(e) => 
                    if verbose{
                        println!("Error parsing item line {}: {:?}", line_number + 1, e)
                    }
            }
        }
    }
    items
}

fn load_items_csv(csv_file: PathBuf, verbose: bool) -> std::io::Result<Items>{
    let mut file = File::open(csv_file)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_items(buffer, verbose))
}

// Report 
fn parse_reports(buffer: String, verbose: bool) -> Reports{
    let mut reports = Reports::new();
    
    for (line_number, item) in buffer.split('\n').enumerate(){
        if item != ""{
            match parse_line(item, "date", "quantity", "income"){
                Ok((id, date, quantity, income)) => {
                    if verbose {
                        println!("Adding {} | {} | {} | {} to reports list", id, date, quantity, income);
                    };
                    reports.add_or_update(
                        &date,
                        quantity,
                        income,
                        id,
                        true
                    );
                    },
                Err(e) => 
                    if verbose {
                        println!("Error parsing report line {}: {:?}", line_number + 1, e)
                    }
            }
        }
    }
    reports
}

fn load_reports_csv(csv_file: PathBuf, verbose: bool) -> std::io::Result<Reports>{
    let mut file = File::open(csv_file)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_reports(buffer, verbose))
}

// Save items list, create a new file if it doesnt exist
fn save_items(file_name: PathBuf, items: Items) -> std::io::Result<()>{
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)?;

    file.write(b"id,name,stock,price\n")?;

    for item in items.get_item_list().into_iter(){
        let line = format!("{},{},{},{}\n", item.id, item.name, item.quantity, item.price);
        file.write(line.as_bytes())?;
    }
    Ok(())
}

// Save reports list, create a new file if it doesnt exist
fn save_reports(file_name: PathBuf, reports: Reports) -> std::io::Result<()>{
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)?;

    file.write(b"id,date,quantity,income\n")?;

    for report in reports.get_report_list().into_iter(){
        let line = format!("{},{},{},{}\n", report.id, report.date, report.quantity, report.income);
        file.write(line.as_bytes())?;
    }
    Ok(())
}


#[derive(StructOpt, Debug)]
enum Command{
    Add {
        name: String,
        quantity: String,
        price: String
    },
    Buy {
        name: String,
        quantity: String
    }, 
    Delete {
        name: String
    }, 
    Report {},
    List {}
}


#[derive(StructOpt, Debug)]
#[structopt(about = "Dev Restaurant CLI, For Assignment 4 by Eric Hartanto")]
struct Opt{
    #[structopt(short, parse(from_os_str), default_value = "food.csv")]
    items_csv: PathBuf,
    #[structopt(short, parse(from_os_str), default_value = "report.csv")]
    reports_csv: PathBuf,
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short, help = "verbose")]
    verbose: bool
}

fn run(opt: Opt) -> Result<(), std::io::Error>{
    let mut items = 
        match load_items_csv(opt.items_csv.clone(), opt.verbose.clone()){
            Ok(item) => item,
            Err(_) => Items::new(),
        };

    let mut reports = 
        match load_reports_csv(opt.reports_csv.clone(), opt.verbose.clone()){
            Ok(report) => report,
            Err(_) => Reports::new(),
        };
    
    let curr_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    match opt.cmd{
        // Add or update item
        Command::Add { name, quantity, price } => {
            let q = match quantity.parse::<i64>(){
                Ok(e) => e,
                Err(_) => {
                    println!("Invalid value found on quantity parameter: {}", quantity);
                    return Ok(());
                },
            };

            let p = match price.parse::<i64>(){
                Ok(e) => e,
                Err(_) => {
                    println!("Invalid value found on price parameter: {}", price);
                    return Ok(());
                },
            };
            items.add_or_update(&name, q, p);
            save_items(opt.items_csv, items)?;
            println!("Berhasil menambahkan {} ke list makanan", name);
        },

        // Updates existing item, by reducing the quantity then updates the report with quantity * price for that particular date
        Command::Buy { name, quantity } => {

            let q = match quantity.parse::<i64>(){
                Ok(e) => e,
                Err(_) => {
                    println!("Invalid value found on quantity parameter: {}", quantity);
                    return Ok(());
                },
            };


            let income = match items.buy(&name, q){
                Ok(price) => { 
                    price * q
                }
                Err(e) => {
                    match e{
                        ItemError::ItemNotFound(_) => {
                            println!("Tidak ada makanan dengan nama \"{}\"", name);
                            return Ok(());
                        },
                        ItemError::NotEnoughStock(q1, q2) => {
                            println!("Maaf, kuantitas makanan tidak mencukupi, hanya tersedia stok {} dari {}", q1, q2);
                            return Ok(());
                        },
                        _ => {return Ok(());}
                        
                    }
                },
            };

            reports.add_or_update(&curr_date, q, income, 0, false);

            save_items(opt.items_csv, items)?;
            save_reports(opt.reports_csv, reports)?;

            println!("Berhasil membeli makanan {} dengan kuantitas {} dan total {}", name, quantity, income);
        },

        // Deletes existing entry
        Command::Delete { name } => {
            match items.delete(&name){
                Ok(_) => {
                    save_items(opt.items_csv, items)?;
                    println!("Berhasil menghapus {} dari list makanan", name);
                },
                Err(_) => println!("Makanan dengan nama \"{}\" tidak ditemukan", name),
            }
        },
        
        // Show Reports
        Command::Report {  } => {
            reports.print_reports();
        },

        // Show Item List
        Command::List {  } => {
            items.print_items();
        },
    }
    Ok(())
}

fn main(){
    let opt = Opt::from_args();
    if let Err(e) = run(opt){
        println!("an error occured: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Items, ItemError, Reports};
    
    #[test]
    fn add_items_test(){
        let mut items = Items::new();
        items.add("Risoles", 164, 25000, 0, false);
        items.add("Ikan Asin", 512, 25182, 0, false);
        items.add("Sayur Asin", 59999, 9999999, 0, false);
        items.add("Telur", 9999999999, 99999999999, 0, false);

        assert_eq!(items.list.get(&0).unwrap().name, "risoles");
        assert_eq!(items.list.get(&1).unwrap().name, "ikan asin");
        assert_eq!(items.list.get(&2).unwrap().name, "sayur asin");
        assert_eq!(items.list.get(&3).unwrap().name, "telur");

        assert_eq!(items.list.get(&0).unwrap().quantity, 164);
        assert_eq!(items.list.get(&1).unwrap().quantity, 512);
        assert_eq!(items.list.get(&2).unwrap().quantity, 59999);
        assert_eq!(items.list.get(&3).unwrap().quantity, 9999999999);

        assert_eq!(items.list.get(&0).unwrap().price, 25000);
        assert_eq!(items.list.get(&1).unwrap().price, 25182);
        assert_eq!(items.list.get(&2).unwrap().price, 9999999);
        assert_eq!(items.list.get(&3).unwrap().price, 99999999999);
    }

    #[test]
    fn update_items_test(){
        let mut items = Items::new();
        items.add("Risoles", 164, 25000, 0, false);
        items.add("Ikan Asin", 512, 25182, 0, false);
        items.add("Sayur Asin", 59999, 9999999, 0, false);
        items.add("Telur", 9999999999, 99999999999, 0, false);

        items.update(0, "Risoles", 100, 200);
        items.update(1, "Ikan Asin", 500, 600);
        items.update(2, "Sayur Asin", 900, 1);
        items.update(3, "Telur", 18953279, 83274914);

        assert_eq!(items.list.get(&0).unwrap().quantity, 100);
        assert_eq!(items.list.get(&1).unwrap().quantity, 500);
        assert_eq!(items.list.get(&2).unwrap().quantity, 900);
        assert_eq!(items.list.get(&3).unwrap().quantity, 18953279);

        assert_eq!(items.list.get(&0).unwrap().price, 200);
        assert_eq!(items.list.get(&1).unwrap().price, 600);
        assert_eq!(items.list.get(&2).unwrap().price, 1);
        assert_eq!(items.list.get(&3).unwrap().price, 83274914);
    }

    #[test]
    #[allow(unused_must_use)]
    fn buy_items_test(){
        let mut items = Items::new();
        items.add("Risoles", 164, 25000, 0, false);
        items.add("Ikan Asin", 512, 25182, 0, false);
        items.add("Sayur Asin", 59999, 9999999, 0, false);
        items.add("Telur", 9999999999, 99999999999, 0, false);
        
        items.buy("Risoles", 1);
        items.buy("Ikan Asin", 50);
        items.buy("Sayur Asin", 45);
        items.buy("Telur", 800);

        assert_eq!(items.list.get(&0).unwrap().quantity, 163);
        assert_eq!(items.list.get(&1).unwrap().quantity, 462);
        assert_eq!(items.list.get(&2).unwrap().quantity, 59954);
        assert_eq!(items.list.get(&3).unwrap().quantity, 9999999199);
    }

    #[test]
    #[allow(unused_must_use)]
    fn delete_items_test(){
        let mut items = Items::new();
        items.add("Risoles", 164, 25000, 0, false);
        items.add("Ikan Asin", 512, 25182, 0, false);
        items.add("Sayur Asin", 59999, 9999999, 0, false);
        items.add("Telur", 9999999999, 99999999999, 0, false);

        assert_eq!(&items.find_item("Risoles").unwrap(), items.list.get(&0).unwrap());

        items.delete("Risoles");

        assert_eq!(items.find_item("Risoles"), Err(ItemError::ItemNotFound("Risoles".to_string())));
        assert_eq!(&items.find_item("Ikan Asin").unwrap(), items.list.get(&1).unwrap());
        assert_eq!(&items.find_item("Sayur Asin").unwrap(), items.list.get(&2).unwrap());
        assert_eq!(&items.find_item("Telur").unwrap(), items.list.get(&3).unwrap());
    }

    #[test]
    fn add_reports_test(){
        let mut reports = Reports::new();
        reports.add_or_update("22-11-2022", 512, 512, 0, false);
        reports.add_or_update("01-02-2022", 761, 314, 1, false);
        reports.add_or_update("04-07-2022", 71234, 233, 2, false);
        reports.add_or_update("28-04-2022", 123456, 51, 3, false);

        assert_eq!(reports.list.get(&0).unwrap().date, "22-11-2022".to_string());
        assert_eq!(reports.list.get(&1).unwrap().date, "01-02-2022".to_string());
        assert_eq!(reports.list.get(&2).unwrap().date, "04-07-2022".to_string());
        assert_eq!(reports.list.get(&3).unwrap().date, "28-04-2022".to_string());
    }
    #[test]
    fn update_reports_test(){
        let mut reports = Reports::new();
        reports.add_or_update("22-11-2022", 512, 5, 0, false);
        reports.add_or_update("01-02-2022", 761, 10, 1, false);
        reports.add_or_update("04-07-2022", 71234, 20, 2, false);
        reports.add_or_update("28-04-2022", 123456, 30, 3, false);

        reports.add_or_update("22-11-2022", 612, 5, 0, false);
        reports.add_or_update("01-02-2022", 515, 6, 1, false);
        reports.add_or_update("04-07-2022", 3571234, 7, 2, false);
        reports.add_or_update("28-04-2022", 1243611, 8, 3, false);

        // quantity * income
        assert_eq!(reports.list.get(&0).unwrap().quantity, 1124);
        assert_eq!(reports.list.get(&1).unwrap().quantity, 1276);
        assert_eq!(reports.list.get(&2).unwrap().quantity, 3642468);
        assert_eq!(reports.list.get(&3).unwrap().quantity, 1367067);
        
        assert_eq!(reports.list.get(&0).unwrap().income, 5620);
        assert_eq!(reports.list.get(&1).unwrap().income, 10700);
        assert_eq!(reports.list.get(&2).unwrap().income, 26423318);
        assert_eq!(reports.list.get(&3).unwrap().income, 13652568);
    }
}

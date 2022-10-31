use std::num::ParseIntError;

use chrono::{prelude::*, Duration};

// Implementasi:
// Trait
// DateTime
// Enum Struct
// Loop
// Vectors (Add, Update, Delete)
// Handling process with Result<T> (convert_to_integer())

trait Tenant{
    fn calculate_end_date(&self) -> DateTime<Utc>; // Return waktu kapan lease berakhir
    fn get_end_date(&self) -> String; // Kembalikan kapan selesai
    fn print(&self) -> String; // Print semua data tenant
    fn extend_lease(&mut self, lease_extension_days: i64); // Perpanjang lease
    fn get_tenant_name(&self) -> String; // Kembalikan nama tenant
}

enum Room{ // Ruangan yang ada
    Room1,
    Room2,
    Room3,
    Room4,
    Room5
}

enum Floor{ // Berapa lantai
    Floor1,
    Floor2,
    Floor3
}

fn room_number(room: &Room) -> String{ // Konversi dari enum ke string
    match room{
        Room::Room1 => String::from("1"),
        Room::Room2 => String::from("2"),
        Room::Room3 => String::from("3"),
        Room::Room4 => String::from("4"),
        Room::Room5 => String::from("5"),
    }
}

fn floor_number(floor: &Floor) -> String{
    match floor{
        Floor::Floor1 => String::from("1"),
        Floor::Floor2 => String::from("2"),
        Floor::Floor3 => String::from("3"),
    }
}

struct StudentTenant{ // Student
    name: String,
    student_id: String,
    floor: Floor,
    room: Room,
    start_date: DateTime<Utc>,
    lease_time: Duration
}

struct TeacherTenant{ // Teacher
    name:String,
    teacher_id: String,
    floor: Floor,
    room: Room,
    start_date: DateTime<Utc>,
    lease_time: Duration
}

impl Tenant for StudentTenant{
    fn calculate_end_date(&self) -> DateTime<Utc> {
        self.start_date + self.lease_time
    }

    fn print(&self) -> String {
        format!("Student\nname: {} \nid: {}\nroom: {}\nfloor: {}\nend lease date: {:?}", 
                self.name, self.student_id, 
                room_number(&self.room), floor_number(&self.floor), 
                self.calculate_end_date())
    }

    fn get_end_date(&self) -> String {
        format!("{}", self.calculate_end_date())
    }

    fn extend_lease(&mut self, lease_extension_days: i64) {
        let lease_extension = Duration::days(lease_extension_days);
        self.lease_time = self.lease_time + lease_extension;
    }

    fn get_tenant_name(&self) -> String {
        format!("{}", self.name)
    }
}

impl Tenant for TeacherTenant{
    fn calculate_end_date(&self) -> DateTime<Utc> {
        self.start_date + self.lease_time
    }

    fn print(&self) -> String {
        format!("Teacher\nname: {} \nid: {}\nroom: {}\nfloor: {}\nend lease date: {:?}", 
                self.name, self.teacher_id, room_number(&self.room), 
                floor_number(&self.floor), self.calculate_end_date())
    }

    fn get_end_date(&self) -> String {
        format!("{}", self.calculate_end_date())
    }

    fn extend_lease(&mut self, lease_extension_days: i64) {
        let lease_extension = Duration::days(lease_extension_days);
        self.lease_time = self.lease_time + lease_extension;
    }

    fn get_tenant_name(&self) -> String {
        format!("{}", self.name)
    }

}

// Konversi string ke integer, kalau berhasil kembalikan i64, kalau gagal kembalikan ParseIntError
fn convert_to_integer(to_parse: &str) -> Result<i64, ParseIntError>{
    let integer = to_parse.parse::<i64>()?;
    Ok(integer)
}

// Ambil input nama
fn get_name(line: String) -> String{
    loop {
        let mut nm = String::new();
        
        println!("{}", line);
        std::io::stdin().read_line(&mut nm).expect("Failed to input name");
    
        if nm.trim() != String::from("") {
            return String::from(nm.trim());
        }
    };
}

// Ambil input id
fn get_id(line: String) -> String{ 
    loop {
        let mut id = String::new();
        
        println!("{}", line);
        std::io::stdin().read_line(&mut id).expect("Failed to input student id");
    
        if id.trim() != String::from("") {
            return String::from(id.trim());
        }
    };
}

// Ambil input lantai dimana hanya akan menerima lantai 1 - 3
fn get_floor() -> Floor{ 
    loop {
        let mut flr = String::new();
        
        println!("Input Floor [1 - 3]: ");
        std::io::stdin().read_line(&mut flr).expect("Failed to input floor");
    
        match flr.trim(){
            "1" => return Floor::Floor1,
            "2" => return Floor::Floor2,
            "3" => return Floor::Floor3,
            _ => return Floor::Floor1,
        }
    };
}

// Ambil input ruang dimana hanya akan menerima ruang 1 - 5
fn get_room() -> Room{
    loop {
        let mut rm = String::new();
        
        println!("Input Room [1 - 5]: ");
        std::io::stdin().read_line(&mut rm).expect("Failed to input room");
    
        return match rm.trim(){
            "1" => Room::Room1,
            "2" => Room::Room2,
            "3" => Room::Room3,
            "4" => Room::Room4,
            "5" => Room::Room5,
            _ => Room::Room1
        };
    };
}

// Ambil input seberapa lama waktu lease, dimana inputan ini dikonversikan ke hari
fn get_lease_time() -> i64{
    loop {
        let mut lt = String::new();
        
        println!("Input Duration [in Days]: ");
        std::io::stdin().read_line(&mut lt).expect("Failed to input lease time");
    
        match convert_to_integer(&lt.trim()){
            Ok(e) => {
                return e;
            },
            Err(_e) => println!("Please input the number of days")
        };
    };
}

// Buat tenant student baru
fn create_new_student_tenant() -> StudentTenant{
    let name = get_name(String::from("Input New Tenant Student Name"));
    let student_id = get_id(String::from("Input New Tenant Student ID"));
    let floor = get_floor();
    let room = get_room();
    let lease_time: i64 = get_lease_time();

    StudentTenant{
        name: name,
        student_id: student_id,
        floor: floor,
        room: room,
        start_date: Utc::now(),
        lease_time: Duration::days(lease_time)
    }

}

// Buat tenant teacher baru
fn create_new_teacher_tenant() -> TeacherTenant{
    let name = get_name(String::from("Input New Tenant Teacher Name"));
    let teacher_id = get_id(String::from("Input New Tenant Teacher ID"));
    let floor = get_floor();
    let room = get_room();
    let lease_time: i64 = get_lease_time();
    
    TeacherTenant{
        name: name,
        teacher_id: teacher_id,
        floor: floor,
        room: room,
        start_date: Utc::now(),
        lease_time: Duration::days(lease_time)
    }
}

// Perlihatkan semua tenant student/teacher
fn list_tenant(tenant: &Vec<impl Tenant>){
    let mut i = 1;
    if tenant.len() == 0{
        println!("=============================================");
        println!("No Tenants Found!");
        println!("=============================================\n");
        return;
    }

    for ten in tenant.iter(){
        println!("=============================================");
        println!("{}. {}", i, ten.print());
        println!("=============================================\n");
        i += 1;
    }
}

// Dapatkan index tenant dari vektor
fn get_tenant_index(tenant: &mut Vec<impl Tenant>) -> usize{
    loop {
        let mut input = String::from("");
        println!("Pick between 1 to {}", tenant.len());
        
        std::io::stdin().read_line(&mut input).expect("Failed to input number");

        let curr = match convert_to_integer(&input.trim()){
            Ok(e) => e,
            Err(_e) => -1
        };

        if curr > 0 && curr <= tenant.len().try_into().unwrap(){
            return (curr - 1) as usize;
        } 
    }
}


// Tambah waktu lease
fn extend_lease(tenant: &mut Vec<impl Tenant>){
    if tenant.len() == 0{
        println!("=============================================");
        println!("No Tenants Found!");
        println!("=============================================\n");
        return;
    }

    list_tenant(&tenant);

    let index: usize = get_tenant_index(tenant);
    let days = get_lease_time();
    let old_lease = tenant[index].get_end_date();
    tenant[index].extend_lease(days);
    println!("Extended tenant's lease from {} to {}", old_lease, tenant[index].get_end_date());
}

// Hapus tenant
fn delete_tenant(tenant: &mut Vec<impl Tenant>){
    if tenant.len() == 0{
        println!("=============================================");
        println!("No Tenants Found!");
        println!("=============================================\n");
        return;
    }

    list_tenant(&tenant);

    let index: usize = get_tenant_index(tenant);
    let tenant_name = tenant[index].get_tenant_name();
    tenant.remove(index);
    println!("Deleted Tenant {} from data", tenant_name);
    println!("=============================================\n");
}


fn main(){
    let mut student_tenants:Vec<StudentTenant> = vec![];
    let mut teacher_tenants:Vec<TeacherTenant> = vec![];

    loop {
        println!("Made by Eric Hartanto");
        println!("For Task 2");
        println!("=============================================\n");
        println!("1. Add Student Tenant");
        println!("2. Add Teacher Tenant");
        println!("3. Delete Student Tenant");
        println!("4. Delete Teacher Temamt");
        println!("5. Extend Student Lease");
        println!("6. Extend Teacher Lease");
        println!("7. List all Student Tenants");
        println!("8. List all Teacher Tenants");
        println!("9. Exit");
        println!("Your Input: ");
        
        let mut input = String::from("");

        std::io::stdin().read_line(&mut input).expect("Failed to input room");

        match input.trim() {
            "1" => student_tenants.push(create_new_student_tenant()),
            "2" => teacher_tenants.push(create_new_teacher_tenant()),
            "3" => delete_tenant(&mut student_tenants),
            "4" => delete_tenant(&mut teacher_tenants),
            "5" => extend_lease(&mut student_tenants),
            "6" => extend_lease(&mut teacher_tenants),
            "7" => list_tenant(&student_tenants),
            "8" => list_tenant(&teacher_tenants),
            "9" => break,
            _ => ()
        }
    }
}

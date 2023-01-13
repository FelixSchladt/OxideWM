use std::error::Error;


// Testing iterating over structs 
#[derive(Debug)]
struct MyStruct {
    a: u8,
    b: u8,
    c: String
}

fn error_checker() -> Result< MyStruct, Box<dyn Error>> {
    let my_struct = MyStruct{ a: 1, b: 2, c: "String".to_string()};
    
    for field_value in my_struct{
        if field_value.is_empty(){
            eprintln!("Error: this field is required!");
        }
        println!("All fields are filled. :)");
    }
    Ok(MyStruct)
}
fn main(){
    error_checker();
}

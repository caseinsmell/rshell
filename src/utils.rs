pub static DEBUG: bool = false;

// Should I make this a macro?
pub fn debug(s : String) -> i32 {
    if DEBUG {
        println!("DEBUG: {}", s);
    }
    0
} 


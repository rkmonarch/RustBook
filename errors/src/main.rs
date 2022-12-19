use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
// let mut f = File::open("Hello.txt")?;


// let mut s = String::new();
// // match f.read_to_string(&mut s) {
// //     Ok(_) => Ok(s),
// //     Err(e) => return Err(e),

// // }
// f.read_to_string(&mut s)?;
// Ok(s)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

 fn a(){
    b();
}

 fn b(){
    c(22);
}

 fn c(num:i32){
    if num == 22{
        panic!("Don't pass 22")
    }
}
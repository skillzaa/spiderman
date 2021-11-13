use spiderman::{SpiderMan,At};
use std::error::Error;
#[cfg(test)]
#[test]
fn welcome(){
 let string_data  = std::fs::read_to_string("./abc.txt").unwrap();
 let mut spiderman = SpiderMan::new(string_data);
    let start_triggers = spiderman
    .triggers.add("start_trigger",
        String::from("###<"),
            At::LineStart,
                start_handler);
// ("end_trigger");
let end_triggers = spiderman
.triggers.add("end_trigger",
    String::from("###>"),
        At::LineStart,
            end_handler);
spiderman.execute();
    // ok(true)
}

fn start_handler()->bool{
    println!("start Handler Working!!!!!!");
    true
}
fn end_handler()->bool{
    println!("End Handler Working!!!!!!");
    true
}

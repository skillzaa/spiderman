use spiderman::SpiderMan;
use std::error::Error;
#[cfg(test)]
#[test]
fn welcome(){
 let string_data  = std::fs::read_to_string("./enigma.txt").unwrap();
 let spiderman = SpiderMan::new(string_data);
let start_triggers = spiderman.triggers.add("start_trigger", "###<", "Anywhere", start_handler);
// ("start_trigger");
spiderman.execute();
    ok(true)
}

fn start_handler()->bool{
    println!("start Handler Working");
    true
}

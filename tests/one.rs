use spiderman::{SpiderMan,At,SpiderPack};
#[cfg(test)]
#[test]
fn welcome(){
 let string_data  = std::fs::read_to_string("./abc.txt").unwrap();
 let mut spiderman = SpiderMan::new(string_data);
 let _ = spiderman.recordings.add("first_recorder");

    let start_triggers = spiderman
    .triggers.add("start_trigger",
        String::from("###<"),
            At::LineStart,
                start_handler);
let end_triggers = spiderman
.triggers.add("end_trigger",
    String::from("###>"),
        At::LineStart,
            end_handler);
spiderman.execute();
}

fn start_handler(spider_pack:&mut SpiderPack)->bool{
    match spider_pack.recordings.get("first_recorder") {
        Some(f)=>{
            f.start();
            return true;
        },
        None=>{
            return true;
        }
    }
}
fn end_handler(spider_pack:&mut SpiderPack)->bool{
    match spider_pack.recordings.get("first_recorder") {
        Some(f)=>{
            f.stop();
        },
        None=>{
            
        }
    }
    true
}

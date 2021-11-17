#[cfg(test)]
use spiderman::{SpiderMan,At,SpiderPack};
#[test]
fn test_uno(){
 let incomming_data  = 
 std::fs::read_to_string("./one.txt");
 println!("{:?}",incomming_data);
 assert!(incomming_data.is_ok());
 //-----------------------------
 let string_data  = incomming_data.unwrap();
 let mut spiderman = SpiderMan::new(string_data);
 //--create a record
 let rec_result = 
 spiderman.spider_pack.recorders.add("first_recorder");
assert_eq!(rec_result,false); //Hashmap return None on first time entry of an elm 

//--create trigger one (start_trigger) 
    let start_trigger = spiderman
    .triggers.add("start_trigger",
        String::from("###<").as_str(),
            At::LineStart,
                start_handler);
assert_eq!(start_trigger,false);
//--create trigger two (end_trigger)                
let end_trigger = spiderman
.triggers.add("end_trigger",
    &String::from("###>"),
        At::LineStart,
            end_handler);
assert_eq!(end_trigger,false);            
spiderman.execute();
}

fn start_handler(spider_pack:&mut SpiderPack)->bool{
    let fr = spider_pack.recorders.get("first_recorder");
    match fr {
        Some(f)=>{
            let tf = f.start();
            assert_eq!(tf,true);
            return true;
        },
        None=>{
            return true;
        }
    }
}
fn end_handler(spider_pack:&mut SpiderPack)->bool{
    match spider_pack.recorders.get("first_recorder") {
        Some(f)=>{
            f.append(&spider_pack.current_line);
            f.stop();
        },
        None=>{
            
        }
    }
    true
}

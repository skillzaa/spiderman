use std::fmt::Debug;

use super::At;
use crate::SpiderErrors;
use crate::SpiderPack;
// #[derive(Debug)]
/// The Trigger Struct look_for any given pattern with 
/// in a line.
/// Keep in mind that if it finds a pattern (as per the 
/// look_for property) **Twice** in one line, it will 
/// trigger just once. This will happen with the 
/// At::Anywhere clause. 
/// The execute method will strip empty space from a line end and start thus **Do not form triggers based on empty spaces"
pub struct Trigger {
    name: String,
    look_for:String,
    at:At,
    pointer:fn(&mut SpiderPack)->bool,
}

impl Trigger {
    pub fn new(name:&str,look_for:&str,at:At,pointer:fn(&mut SpiderPack)->bool)->Self{
        Trigger {
            name: String::from(name),
            look_for: look_for.to_string(),
            at,
            pointer
        }
    }
    fn  line_start(&self, line:&String)->bool{
        let no_of_chars = self.look_for.len();
        let line_first:String = line.chars().take(no_of_chars).collect();
        if line_first == self.look_for {
            true
        } else {
            false
        }
    }
    fn line_end(&self, line:&String)->bool{
        let no_of_chars = self.look_for.len();
        let line_end:String = line.chars().rev().take(no_of_chars).collect();
        if line_end == self.look_for {
            true
        } else {
            false
        }
    }
    fn anywhere(&self, line:&String)->bool{
        if line.contains(&self.look_for){
            true
        }else{false}
    }    
    fn run_trigger(&self,line:&String,spider_pack:&mut SpiderPack)->Result<bool,SpiderErrors>{
            match (self.pointer)(spider_pack) {
                true=>{return Ok(true);},
                false=>{
                    Err(SpiderErrors::UserReturnedFalse)
                },
            }
    }
    pub fn execute(&self,line:&String,spider_pack:&mut SpiderPack)->Result<bool,SpiderErrors>{
        let line = String::from(line.trim());
        match self.at {
            At::LineStart=>{
                if self.line_start(&line){
                    let r = 
                    self.run_trigger(&line,spider_pack);
                    r
                }else {Ok(false)}//means no trigger
            },
            At::LineEnd=>{
                if self.line_end(&line){
                    let r 
                    = self.run_trigger(&line,spider_pack);
                    r
                }else{Ok(false)}//means no trigger
            },
            At::Anywhere=>{
                if self.anywhere(&line){
                    let r 
                    = self.run_trigger(&line,spider_pack);
                    r
                }else{Ok(false)}//means no trigger
            },
        }
    }
}

#[cfg(test)]
mod basics {
//keep in mind that the execute fn strip blank spaces frmo line but calling line_start etc directly does not strip    
use super::*;
    #[test]
    fn trig_line_start() {
        let tri = Trigger::new("new_trig",
        "!", At::LineStart,eve_handle_fn );
            assert_eq!(tri.name,"new_trig");
            assert_eq!(tri.look_for,"!");
    let line_start_result = 
    tri.line_start(&String::from("! some thing"));
    assert_eq!(line_start_result,true);    
    let line_start_result02 = 
    tri.line_start(&String::from("s!ome thing"));
    assert_eq!(line_start_result02,false);    
    let line_start_result03 = 
    tri.line_start(&String::from("!!!ome thing"));
    assert_eq!(line_start_result03,true);   
    } 
    // This is open trigger fn for all tests        
    fn eve_handle_fn (_spider_pack:&mut SpiderPack)->bool {
        println!("eve_handle_fn was run....!!!!");
        true
    }
    #[test]
    fn trig_line_end(){
        let trig = Trigger::new("second",">",
            At::LineEnd,eve_handle_fn);

    let res = trig.line_end(&String::from("some text with >"));
    assert_eq!(res,true);    

    }
    #[test]
    fn trig_line_end02(){
        //it has extra space at the end
        let trig = Trigger::new("second",">",
            At::LineEnd,eve_handle_fn);

    let res = trig.line_end(&String::from("some text with"));
    assert_eq!(res,false);    

    }
    #[test]
    fn anywhere_one(){
        //it has extra space at the end
        let trig = Trigger::new("second",">",
            At::Anywhere,eve_handle_fn);

    let res = trig.anywhere(&String::from("some >text with"));
    assert_eq!(res,true);    
    let res = trig.anywhere(&String::from(">some text with"));
    assert_eq!(res,true);    
    let res = trig.anywhere(&String::from("some text with>"));
    assert_eq!(res,true);    
    let res = trig.anywhere(&String::from("some> text with>"));
    assert_eq!(res,true);    

    }
}//mod tests
#[cfg(test)]
mod execute {

use super::*;
    #[test]
    fn trig_line_start() {
        let tri = Trigger::new("new_trig",
        "!", At::LineStart,eve_handle_fn );
            assert_eq!(tri.name,"new_trig");
            assert_eq!(tri.look_for,"!");
            //line begin with SPACE -shd be ok
    let line_start_result = 
    tri.execute(&String::from(" ! some thing"));
    assert_eq!(line_start_result,true);    
    let line_start_result02 = 
    tri.line_start(&String::from("s!ome thing"));
    assert_eq!(line_start_result02,false);
    //space added at start    
    let line_start_result03 = 
    tri.line_start(&String::from(" !!!ome thing"));
    assert_eq!(line_start_result03,true);   
    } 
    // This is open trigger fn for all tests        
    fn eve_handle_fn (_spider_pack:&mut SpiderPack)->bool {
        println!("eve_handle_fn was run....!!!!");
        true
    }
    #[test]
    fn trig_line_end(){
        let trig = Trigger::new("second",">",
            At::LineEnd,eve_handle_fn);

    let res = trig.line_end(&String::from("some text with >"));
    assert_eq!(res,true);    

    }
    #[test]
    fn trig_line_end02(){
        //it has extra space at the end
        let trig = Trigger::new("second",">",
            At::LineEnd,eve_handle_fn);

    let res = trig.line_end(&String::from("some text with"));
    assert_eq!(res,false);    

    }
    #[test]
    fn anywhere_one(){
        //it has extra space at the end
        let trig = Trigger::new("second",">",
            At::Anywhere,eve_handle_fn);

    let res = trig.anywhere(&String::from("some >text with"));
    assert_eq!(res,true);    
    let res = trig.anywhere(&String::from(">some text with"));
    assert_eq!(res,true);    
    let res = trig.anywhere(&String::from("some text with>"));
    assert_eq!(res,true);    
    let res = trig.anywhere(&String::from("some> text with>"));
    assert_eq!(res,true);    

    }
}//mod tests

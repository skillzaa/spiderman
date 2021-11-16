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
/// Anywhere clause. 
/// The execute method will strip empty space from a line end and start thus **Do not form triggers based on empty spaces"
pub struct Trigger {
    name: String,
    look_for:String,
    at:At,
    
}
// pointer:fn(&mut SpiderPack)->bool,
    //pointer:fn(&mut SpiderPack
impl Trigger {
    pub fn new(name:&str,look_for:&str,at:At)->Self {
        Trigger {
            name: String::from(name),
            look_for: look_for.to_string(),
            at,
        }
    }    
    // fn run_trigger(&self,line:&String,spider_pack:&mut SpiderPack)->Result<bool,SpiderErrors>{
    //         match (self.pointer)(spider_pack) {
    //             true=>{return Ok(true);},
    //             false=>{
    //                 Err(SpiderErrors::UserReturnedFalse)
    //             },
    //         }
    // }
    pub fn execute(&self,line:&String)->bool{
        let line = String::from(line.trim());
        match self.at {
            At::LineStart=>{
                let no_of_chars = self.look_for.len();
                let line_first:String = line.chars().take(no_of_chars).collect();
                if line_first == self.look_for {
                true
                    } else {
                false
                }
            },
            At::LineEnd=>{
                let no_of_chars = self.look_for.len();
                let execute:String = line.chars().rev().take(no_of_chars).collect();
                    if execute == self.look_for {
                    true
                        } else {
                    false
                    }            },
            At::Anywhere=>{
                if line.contains(&self.look_for){
                true
                }else{false}            
            },
        }
    }
}

#[cfg(test)]
mod basics {
use super::*;
    #[test]
    fn trig_line_start() {
    //================    
        let tri = Trigger::new("new_trig",
        "!", At::LineStart);
            assert_eq!(tri.name,"new_trig");
            assert_eq!(tri.look_for,"!");
    //===========================        
    assert_eq!(tri.execute(&String::from("! some thing")),true);    
    assert_eq!(tri.execute(&String::from("s!ome thing")),false);    
    assert_eq!(tri.execute(&String::from("!!!ome thing")),true);    
    assert_eq!(tri.execute(&String::from(" !!!ome thing")),true);    
    assert_eq!(tri.execute(&String::from("      !!!ome thing")),true);    
    } 
    #[test]
    fn trig_line_end() {
    //================    
        let tri = Trigger::new("new_trig",
        "!", At::LineEnd);
            assert_eq!(tri.name,"new_trig");
            assert_eq!(tri.look_for,"!");
    //===========================        
    assert_eq!(tri.execute(&String::from("! some thing!")),true);    
    assert_eq!(tri.execute(&String::from("s!ome thing")),false);    
    assert_eq!(tri.execute(&String::from("some thing!!!")),true);    
    assert_eq!(tri.execute(&String::from("ome thing!!!   ")),true);    
    assert_eq!(tri.execute(&String::from("      !!!ome thing !,   ")),false);    
    } 
    #[test]
    fn trig_anywhere() {
    //================    
        let tri = Trigger::new("new_trig",
        "!", At::Anywhere);
            assert_eq!(tri.name,"new_trig");
            assert_eq!(tri.look_for,"!");
    //===========================        
    assert_eq!(tri.execute(&String::from("some thing!")),true);    
    assert_eq!(tri.execute(&String::from("some thing")),false);    
    assert_eq!(tri.execute(&String::from("some thin!g")),true);    
    assert_eq!(tri.execute(&String::from("  !ome thing   ")),true);    
    assert_eq!(tri.execute(&String::from("      ome thing ,   ")),false);    
    } 
    
}//mod tests

use crate::At;
mod trigger;
use std::collections::HashMap;
use trigger::Trigger;
use crate::{SpiderErrors}; 
use crate::spiderpack::SpiderPack; 

pub struct Triggers {
    triggers:HashMap<String,Trigger>,
}

impl Triggers {
    pub fn new()->Self{
        Triggers {
            triggers:HashMap::new(),
        }
    }
    pub fn add(&mut self,name:&str,look_for:&str,at:At,event_handler:fn(spider_pack:&mut SpiderPack)->bool)->bool{
    let trig = Trigger::new(name,look_for,at,event_handler);
    let x = self.triggers.insert(name.to_string(), trig);
        if x.is_some() {true}else{false}
    }
    pub fn get(&mut self,name:&str)->Option<&mut Trigger>{
                self.triggers.get_mut(name)
    }
    pub fn kill(&mut self,name:&str)->bool{
        let r =self.triggers.remove(name);
        if r.is_some() {true}else{false}
    }
    pub fn execute(&mut self,the_line:&String,spider_pack:&mut SpiderPack)->Result<bool,SpiderErrors>{
        for (_name,trig) in self.triggers.iter() {
            if trig.execute(the_line) {
              //----add to spider pack here
              spider_pack.current_line = String::from(the_line);  
            //   spider_pack. 
              //process the event here
              (trig.event_handler)(spider_pack); 
            }
        }
    Ok(true)    
    }
   
}

fn get_line_including(line:String,pattern:String)->String{
    let first_char = pattern.chars().nth(0).unwrap();
    let ret = String::from("");
    for ch in line.chars() {
        if ch == first_char {
            
        }
    }
line
}
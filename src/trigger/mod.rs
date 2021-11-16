mod at;
mod trigger;
use std::collections::HashMap;
use trigger::Trigger;
use crate::{SpiderErrors}; 
use crate::spiderpack::SpiderPack; 
pub use at::At;

pub struct Triggers {
    trigger:HashMap<String,Trigger>,
}

impl Triggers {
    pub fn new()->Self{
        Triggers {
            trigger:HashMap::new(),
        }
    }
    pub fn add(&mut self,name:&str,look_for:&str,at:At,event_handler:fn(spider_pack:&mut SpiderPack)->bool)->bool{
    let trig = Trigger::new(name,look_for,at,event_handler);
    let x = self.trigger.insert(name.to_string(), trig);
        if x.is_some() {true}else{false}
    }
    pub fn get(&mut self,name:&str)->Option<&mut Trigger>{
                self.trigger.get_mut(name)
    }
    pub fn kill(&mut self,name:&str)->bool{
        let r =self.trigger.remove(name);
        if r.is_some() {true}else{false}
    }
    pub fn execute(&mut self,the_line:&String,spider_pack:&mut SpiderPack)->Result<bool,SpiderErrors>{
        for (_name,trig) in self.trigger.iter() {
            //trig.execute(the_line,spider_pack)?;
        }
    Ok(true)    
    }
}


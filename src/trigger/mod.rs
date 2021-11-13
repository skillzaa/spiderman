use std::collections::HashMap;
use std::io::Error;
mod trigger;
use trigger::{Trigger,At};
pub struct Triggers {
    trigger:HashMap<String,Trigger>,
}

impl Triggers {
    pub fn new()->Self{
        Triggers {
            trigger:HashMap::new(),
        }
    }
    pub fn add(&mut self,name:&str,look_for:String,at:At,handler:fn()->bool)->bool{
    let trig = Trigger::new(name,look_for,at,handler);
    let x = self.trigger.insert(name.to_string(), trig);
        if x.is_some() {true}else{false}
    }
    pub fn get(&mut self,name:&str)->Option<&mut Record>{
        self.trigger.get_mut(name)
    }
    
    pub fn kill(&mut self,name:&str)->bool{
        let r =self.trigger.remove(name);
        if r.is_some() {true}else{false}
    }
}

// pub fn abc(){
//     // let a = Recordings::
// }
mod recording;
mod trigger;
mod process;
mod flag;
use std::error::{Error};
pub use trigger::At;
use trigger::Triggers;
use recording::Recordings;
// use flag::Flags;
//------------------
pub struct SpiderMan {
pub recordings :Recordings,
pub triggers :Triggers,
    string_data:String
}
impl SpiderMan {
    pub fn new(string_data:String)->Self{
        SpiderMan {
            recordings : Recordings::new(),
            triggers : Triggers::new(),
            string_data,
        }  
    }
    /// The fn execute will run the triggers for each line
    pub fn execute(&mut self)->bool{
        for the_line in self.string_data.lines(){
            match self.triggers.execute(&the_line){
                Ok(_t)=>{continue;},
                Err(_e)=>{return false}
            }
        }
        true
    }
}

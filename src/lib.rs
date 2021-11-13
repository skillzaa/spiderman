mod recording;
mod trigger;
mod process;

use trigger::Triggers;
use recording::Recording;
pub struct At {u:u32}
impl At {
   pub fn new(u:u32)->Self{
       At{u}
   } 
   pub fn abc(&self,u:u32)->u32{
       u +100
   } 
   pub fn xyz(&self,u:u32)->u32{
    u + 200
   }
}
pub struct SpiderMan {
    recordings :Recording,
pub triggers :Triggers,
    string_data:String
}
impl SpiderMan {
    pub fn new(string_data:String)->Self{
        SpiderMan {
            recordings : Recording::new(),
            triggers : Triggers::new(),
            string_data,
        }  
    }
    pub fn execute(&mut self){
        for the_line in self.string_data.lines(){
            self.triggers.run(&the_line);
        }
    }
}

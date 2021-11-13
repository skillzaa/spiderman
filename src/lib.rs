mod recording;
mod trigger;
mod process;
mod flag;
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
    pub fn execute(&mut self){
        for the_line in self.string_data.lines(){
            self.triggers.run(&the_line);
        }
    }
}

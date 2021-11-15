mod recording;
mod trigger;
mod process;
mod flag;
mod errors;
mod spiderpack;
pub use spiderpack::SpiderPack;
use errors::SpiderErrors;
pub use trigger::At;
use trigger::Triggers;
use recording::Recordings;
//------------------
pub struct SpiderMan {
pub recordings :Recordings,
pub triggers :Triggers,
    spider_pack:SpiderPack,
    string_data:String,
}
impl SpiderMan {
    pub fn new(string_data:String)->Self{
        SpiderMan {
            recordings : Recordings::new(),
            triggers : Triggers::new(),
            spider_pack:SpiderPack::new(),
            string_data,
        }  
    }
    /// The fn execute will run the triggers for each
    /// line until the end of the file
    pub fn execute(&mut self)->bool{

        for the_line in self.string_data.lines(){
            let line_string = String::from(the_line);
            //-- process the triggers for current line
            match self.triggers.execute(&line_string,&mut self.spider_pack){
                Ok(_t)=>{continue;},
                Err(_e)=>{return false}
            }
            //-- process the records for current line
            for (name,record) in &self.recordings.records{
                record.append(String::from(the_line));
            }
        }
        self.eof();
        true
    }
    fn eof(&self){
        for (name,record) in &self.recordings.records{
            println!("EOF :: {}",record.read());
        }
    }
}

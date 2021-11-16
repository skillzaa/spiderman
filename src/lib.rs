mod recorders;
mod trigger;
mod process;
mod flag;
mod errors;
mod spiderpack;
pub use spiderpack::SpiderPack;
use errors::SpiderErrors;
pub use trigger::At;
use trigger::Triggers;
use recorders::Recorders;
//------------------
pub struct SpiderMan {
pub recorders :Recorders,
pub triggers :Triggers,
    spider_pack:SpiderPack,
    string_data:String,
}
impl SpiderMan {
    pub fn new(string_data:String)->Self{
        SpiderMan {
            recorders : Recorders::new(),
            triggers : Triggers::new(),
            spider_pack:SpiderPack::new(),
            string_data,
        }  
    }
    /// The fn execute will run the triggers for each
    /// line until the end of the file
    pub fn execute(&mut self)->bool{
        let mut line_number = 1;
        for the_line in self.string_data.lines(){
            let line_string = String::from(the_line);
            //-- process the triggers for current line
            match self.triggers.execute(&line_string,&mut self.spider_pack){
                Ok(_t)=>{},
                Err(_e)=>{return false}
            }
            //-- process the records for current line
            for (_name, record) in &mut self.recorders.records {
                record.append(String::from(the_line));
                // println!("{}: {}", name, record);
            }
        }
        self.eof();
        true
    }
    fn eof(&self){
        for (name,record) in &self.recorders.records{
            println!("EOF :: {}",record.copy());
        }
    }
}

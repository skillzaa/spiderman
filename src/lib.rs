mod recorders;
mod trigger;
mod process;
// mod flag;
mod errors;
mod spiderpack;
mod flags;
use flags::Flags;
pub use spiderpack::SpiderPack;
use errors::SpiderErrors;
mod at;
pub use at::At;
use trigger::Triggers;
use recorders::Recorders;
//------------------
pub struct SpiderMan {
pub spider_pack :SpiderPack,
pub triggers :Triggers,
    string_data:String,
}
impl SpiderMan {
    pub fn new(string_data:String)->Self{
        SpiderMan {
            spider_pack :SpiderPack::new(),
            triggers : Triggers::new(),
            string_data,
        }  
    }
    pub fn add_record(){

    }
    /// The fn execute will run the triggers for each
    /// line until the end of the file
    pub fn execute(&mut self)->bool{

        for the_line in self.string_data.lines(){
            let line_string = String::from(the_line);
            
            //=========== The Execution

            let _r = 
            self.triggers.execute(&line_string, 
                &mut self.spider_pack);
            
            //-- process the records for current line
            for (_name, record) in &mut self.spider_pack.recorders.records {
                record.append(&String::from("\n"));
                record.append(&String::from(the_line));
                // println!("{}: {}", name, record);
            }
        }
        self.eof();
        true
    }
    fn eof(&self){
        for (_name,record) in &self.spider_pack.recorders.records{
            println!("EOF :: {}",record.copy());
        }
    }
    
}

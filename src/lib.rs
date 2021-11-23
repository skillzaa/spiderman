mod recorders;
mod trigger;
mod process;
mod errors;
mod spiderpack;
mod flags;
mod at;
use flags::Flags;
use errors::SpiderErrors;
use trigger::Triggers;
use recorders::Recorders;
pub use spiderpack::SpiderPack;
pub use at::At;
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

/// The execute fn is the heart of the app. This runs 
/// all the events and starts the process.
pub fn execute(&mut self)->bool{

for the_line in self.string_data.lines(){
    let line_string = String::from(the_line);
    
//=========== The Execution
//--the triggers obj will loop all the triggers in it
    let _r = 
    self.triggers.execute(
        &line_string, 
        &mut self.spider_pack
    );
    
    //-- process the records for current line
    for (_name, record) in &mut self.spider_pack.recorders.records {
        // record.append(&String::from("\n"));
        record.append(&String::from(the_line));
        // println!("{}: {}", name, record);
    }
    //-- process the records for EVERY line
    for (name, record) in &mut self.spider_pack.recorders.records {

        // record.append(&String::from("\n"));
        // record.append(&String::from(the_line));
        println!("{}: {}", name, record.copy());
    }
}
// self.eof();
true
}
//===================================
// fn eof(&self){
//     for (_name, record) in &self.spider_pack.recorders.records {
//         record.run_eof(&mut self.spider_pack);
//     }
// }
//-----------lib ends    
}

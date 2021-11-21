// This worked 
use crate::spiderpack::{self, SpiderPack};
// but not this  
// use crate::spiderpack; 
// since in lib.rs only spiderpack::SpiderPack is inscope and not spiderpack
pub struct Recorder {
    name:String,
    data:String,
    mode:bool,
    write_to_file:bool,
    skip_empty_lines:bool,
    skip:u32,
    auto_append:bool,
    every_line: Option<fn(&mut SpiderPack)->bool>,
    eof: Option<fn(&mut SpiderPack)->bool>,
}

impl Recorder {
    pub fn new(name:&str)->Self{
        let name = String::from(name);
        Recorder {
            name: String::from(name),
            data: String::from(""),
            mode:false,
            write_to_file:false,
            skip_empty_lines:false,
            auto_append:true,
            skip:0,
            every_line: None,
            eof: None,
        }
    }
    /// The start fn will make the recorder recording all the 
    pub fn start(&mut self)->bool{
        self.mode = true; 
        self.mode
    }
    pub fn stop(&mut self)->bool{
        self.mode = false;
        self.mode
    }
    pub fn is_start(&self)->bool{
        self.mode
    }
    pub fn clear(&mut self){
        self.data = String::from("");
    }
    /// The append fn will append (add) the provided 
    /// data to the recorder buffer **only if the 
    /// recorder is start/open**, once the data is added 
    /// it will return true.
    /// Incase the recorder is closed / stop, no data 
    /// will be added and **false** will be returned.
    /// append function will return true only if the 
    /// recorder is open AND the append goes 
    /// successfully. 
    pub fn append(&mut self,data:&String)->bool{
        if self.mode {
            self.data.push_str(&data);
            return true;
       }
        false
    }
    /// The copy fn will return a copy of the data from the recorder 
    /// buffer.*It is a copy not a reference to the original 
    /// buffer*
    pub fn every_line(&mut self,
    event_handler : fn(&mut SpiderPack)->bool ){
        self.every_line = Some(event_handler);
    }
    pub fn eof(&mut self,
    event_handler : fn(&mut SpiderPack)->bool ){
        self.eof = Some(event_handler);
    }
    pub fn skip(&mut self,number_of_lines:u32)->u32{
        self.skip = number_of_lines;
        self.skip
    }
    fn skip_minus_one(&mut self)->u32{
        if self.skip >= 1 {
            self.skip = self.skip - 1;
            self.skip
        }else{
            self.skip
        } 
    }
    pub fn get_skip(&self)->u32{
        self.skip
    }
    pub fn auto_append(&mut self,true_false:bool)->bool{
        self.auto_append = true_false;
        self.auto_append
    }
    pub fn get_auto_append(&self)->bool{
        self.auto_append
    }
    pub fn skip_empty_lines(&mut self,true_false:bool)->bool{
        self.skip_empty_lines = true_false;
        self.skip_empty_lines
    }
    pub fn get_skip_empty_lines(&self)->bool{
        self.skip_empty_lines
    }
    pub fn write_to_file(&mut self,true_false:bool)->bool{
        self.write_to_file = true_false;
        self.write_to_file
    }
    pub fn get_write_to_file(&self)->bool{
        self.write_to_file
    }
    pub fn run_eof(&self,spider_pack:&mut SpiderPack)->bool{
        match self.eof {
            Some(h)=>{
                let r =(h)(spider_pack);
                r
            },
            None=> false
        }
    }
    pub fn copy(&self)->String{
        let copy = self.data.clone();
        copy
    }
}

#[cfg(test)]
mod basic {
use super::*;
#[test]
fn one(){
    let mut recorder = Recorder::new("one");
    recorder.start();
    recorder.append(&"first entry..first entry..first entry..first entry..first entry..first entry..".to_string());
    assert!(!recorder.copy().is_empty());
}
#[test]
fn two(){
    let data = r#"1
    2
    3
    4
    5
    6
    7
    8
    9"#;
    let data = String::from(data);
    let mut counter = 0;
    let mut rec= Recorder::new("oddeven");
    for line in data.lines(){
        if counter % 2 == 0 {
            rec.start();
            rec.append(&String::from(line));
        }else {
            rec.stop();
            rec.append(&String::from(line));
        }
        counter = counter + 1;    
    }
    // println!("{}",rec.copy());
    assert!(rec.copy().contains("1"));
    assert!(rec.copy().contains("3"));
    assert!(rec.copy().contains("5"));
    assert!(rec.copy().contains("7"));
    assert!(rec.copy().contains("9"));
}
#[test]
fn three(){
    let data = r#"1
    2
    3
    4
    5
    6
    7
    8
    9"#;
    let data = String::from(data);
    let mut counter = 0;
    let mut rec= Recorder::new("oddeven");
    for line in data.lines(){
        if !(counter % 2 == 0) {
            rec.start();
            rec.append(&String::from(line));
        }else {
            rec.stop();
            rec.append(&String::from(line));
        }
        counter = counter + 1;    
    }
    // println!("{}",rec.copy());
    assert!(rec.copy().contains("2"));
    assert!(rec.copy().contains("4"));
    assert!(rec.copy().contains("6"));
    assert!(rec.copy().contains("8"));
}
    
}//mod
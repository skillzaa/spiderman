use super::At;
use crate::SpiderErrors;
// use errors::
// use std::io::{Error,ErrorKind};
// use at::At;
struct SpiderPack{
    flag:bool,
    }
impl SpiderPack{
    pub fn new()->Self{
        SpiderPack {
           flag:true, 
        }
    }
}    
pub struct Trigger {
    name: String,
    look_for:String,
    at:At,
    pointer:fn()->bool,
}

impl Trigger {
    pub fn new(name:&str,look_for:String,at:At,pointer:fn()->bool)->Self{
        Trigger {
            name: String::from(name),
            look_for,
            at,
            pointer
        }
    }
    fn  line_start(&self, line:&String)->bool{
        //let no_of_chars = self.look_for.len();
        let no_of_chars = 4;
        let line_first:String = line.chars().take(no_of_chars).collect();
        if line_first == self.look_for {
            true
        } else {
            false
        }
    }
    fn line_end(&self, line:&String)->bool{
        true
    }
    fn anywhere(&self, line:&String)->bool{
        true
    }    
    fn run_trigger(&self,line:&String)->Result<bool,SpiderErrors>{
        let _spiderPact = SpiderPack::new();
            match (self.pointer)() {
                true=>{return Ok(true);},
                false=>{
                    Err(SpiderErrors::UserReturnedFalse)
                },
            }
    }
    pub fn execute(&self,line:&String)->Result<bool,SpiderErrors>{
        match self.at {
            At::LineStart=>{
                if self.line_start(line){
                    let r = self.run_trigger(line);
                    r
                }else {Ok(true)}
            },
            At::LineEnd=>{
                if self.line_end(line){
                    let r = self.run_trigger(line);
                    r
                }else{Ok(true)}
            },
            At::Anywhere=>{
                if self.anywhere(line){
                    let r = self.run_trigger(line);
                    r
                }else{Ok(true)}
            },
        }
    }
}

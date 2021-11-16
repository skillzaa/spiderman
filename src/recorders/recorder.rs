
pub struct Recorder {
    name:String,
    data:String,
    flag:bool,
}

impl Recorder {
    pub fn new(name:&str)->Self{
        let name = String::from(name);
        Recorder {
            name: String::from(name),
            data: String::from(""),
            flag:false,
        }
    }
    /// The start fn will make the recorder recording all the 
    pub fn start(&mut self)->bool{
        self.flag = true; 
        self.flag
    }
    pub fn stop(&mut self)->bool{
        self.flag = false;
        self.flag
    }
    pub fn is_start(&self)->bool{
        self.flag
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
    pub fn append(&mut self,data:String)->bool{
        if self.flag {
            self.data.push_str(&data);
            return true;
       }
        false
    }
    /// The copy fn will return a copy of the data from the recorder 
    /// buffer.*It is a copy not a reference to the original 
    /// buffer*
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
    recorder.append("first entry..first entry..first entry..first entry..first entry..first entry..".to_string());
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
            rec.append(String::from(line));
        }else {
            rec.stop();
            rec.append(String::from(line));
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
            rec.append(String::from(line));
        }else {
            rec.stop();
            rec.append(String::from(line));
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
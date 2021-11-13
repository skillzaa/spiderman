use std::collections::HashMap;
use std::io::Error;
mod record;
use record::Record;
pub struct Recording {
    records:HashMap<String,Record>,
}

impl Recording {
    pub fn new()->Self{
        Recording {
            records:HashMap::new(),
        }
    }
    pub fn add(&mut self,name:&str)->bool{
        let rec = Record::new(name);
        let x = self.records.insert(name.to_string(), rec);
        if x.is_some() {true}else{false}
    }
    pub fn get(&mut self,name:&str)->Option<&mut Record>{
        self.records.get_mut(name)
    }
    
    pub fn kill(&mut self,name:&str)->bool{
        let r =self.records.remove(name);
        if r.is_some() {true}else{false}
    }
}

// pub fn abc(){
//     // let a = Recordings::
// }
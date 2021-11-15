use std::collections::HashMap;
mod record;
use record::Record;
pub struct Recordings {
pub records:HashMap<String,Record>,
}

impl Recordings {
    pub fn new()->Self{
        Recordings {
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

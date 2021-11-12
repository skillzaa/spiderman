use std::collections::HashMap;
mod record;
use record::Record;
pub struct Recording {
    records:HashMap<String,Record>,
}

impl Recording {
    pub fn add(&mut self,name:&str)->Option<Record>{
        let rec = Record::new(name);
        let x = self.records.insert(name.to_string(), rec);
        x
    }
    pub fn get(&self,name:&str)->Option<&Record>{
        self.records.get(name)
    }
    
    pub fn kill(&mut self,name:&str)->bool{
        let r =self.records.remove(name);
        if r.is_some() {true}else{false}
    }
}

// pub fn abc(){
//     // let a = Recordings::
// }
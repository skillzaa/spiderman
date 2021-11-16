use std::collections::HashMap;
mod recorder;
use recorder::Recorder;
/// The Recorders Struct is the collection of all the 
/// recorders that the user has created.
/// Any time an event occur (a trigger is triggered) 
/// the Recorders collection is provided to the user 
/// (inside SpiderPack Struct) for his use in his event 
/// handler function.
/// Once a recorder is created. A recorder can be be in 
/// **start** or **stop** state.
/// A recorder in a Start state will record / capture  
/// each line processed by the app unless it is *stoped*.
pub struct Recorders {
pub records:HashMap<String,Recorder>,
}

impl Recorders {
    pub fn new()->Self{
        Recorders {
            records:HashMap::new(),
        }
    }
    pub fn add(&mut self,name:&str)->bool{
        let rec_name = String::from(name);
        let rec = Recorder::new(&rec_name);
        let x = self.records.insert(rec_name, rec);
        if x.is_some() {true}
        else{false}
    }
    pub fn get(&mut self,name:&str)->Option<&mut Recorder>{
        self.records.get_mut(name)
    }    
    pub fn kill(&mut self,name:&str)->bool{
        let r =self.records.remove(name);
        if r.is_some() {true}else{false}
    }
}

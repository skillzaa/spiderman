use std::collections::HashMap;
pub struct Flag {
    flag:bool,
}
impl Flag {
    pub fn new()->Self{
        Flag {
            flag:false,
        }
    }
    pub fn set_true(&mut self)->bool{
        self.flag = true; 
        self.flag
    }
    pub fn set_false(&mut self)->bool{
        self.flag = false;
        self.flag
    }
}
pub struct Flags{
    flags: HashMap<String,Flag>,
}

impl Flags {
    pub fn new()->Self{
        Flags {
            flags:HashMap::new(),
        }
    }
    pub fn add(&mut self,name:&str)->bool{
        let flag_name = String::from(name);
        let new_flag = Flag::new();
        let x = self.flags.insert(flag_name, new_flag);
        match x { // since None is ok 
            Some(x)=>return false,
            None=>return true,
        }
    }
    pub fn get(&mut self,name:&str)->Option<&mut Flag>{
        self.flags.get_mut(name)
    }    
    pub fn kill(&mut self,name:&str)->bool{
        let r =self.flags.remove(name);
        if r.is_some() {true}else{false}
    } 
    
}

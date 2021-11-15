
pub struct Record {
    name:String,
    data:String,
    flag:bool,
}

impl Record {
    pub fn new(name:&str)->Self{
        Record {
            name: String::from(name),
            data: String::from(""),
            flag:false,
        }
    }
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
    pub fn append(&mut self,data:String)->bool{
        if self.is_start(){
            self.data.push_str(&data);
            return true;
       }
        false
    }
    pub fn read(&self)->String{
        let copy = self.data.clone();
        copy
    }
}


pub struct Record {
    name:String,
    data:Vec<char>,
    flag:bool,
}

impl Record {
    pub fn new(name:&str)->Self{
        Record {
            name: String::from(name),
            data: Vec::<char>::new(),
            flag:false,
        }
    }
    pub fn start(&mut self){
        self.flag = true; 
    }
    pub fn stop(&mut self){
        self.flag = true;
    }
    pub fn clear(&mut self){
        self.data = Vec::<char>::new();
    }
    pub fn append(&mut self,data:String){
        for  i in data.chars() {
            self.data.push(i);
        }
    }
    pub fn read(&self)->Vec<char>{
        let copy = self.data.clone();
        copy
    }
}

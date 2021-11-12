
pub struct Record {
    name:String,
    t: u32,
    data:Vec<char>,
}

impl Record {
    pub fn new(name:&str)->Self{
        Record {
            name: String::from(name),
            data: Vec::new(),
            t:0,
        }
    }
    pub fn start(&mut self){
        self.t = self.t + 40; 
    }
    pub fn stop(&self){
        todo!();
    }
    pub fn clear(&self){
        todo!();
    }
    pub fn write(&self){
        todo!();
    }
    pub fn read(&self){
        todo!();
    }
}

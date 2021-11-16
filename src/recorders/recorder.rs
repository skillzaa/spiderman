
pub struct Recorder {
    name:String,
    data:String,
    flag:bool,
}

impl Recorder {
    pub fn new(name:&String)->Self{
        Recorder {
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
    /// The copy fn will return a copy of the data from the recorder buffer. 
    /// *It is a copy not a reference to the original 
    /// buffer*
    pub fn copy(&self)->String{
        let copy = self.data.clone();
        copy
    }
}

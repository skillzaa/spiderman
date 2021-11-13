pub enum At {
    LineStart,
    LineEnd,
    Anywhere,
}
pub struct Trigger {
    name: String,
    look_for:String,
    at:At,
    handle:fn()->bool,
}

impl Trigger {
    pub fn new(name:&str,look_for:String,at:At,handle:fn()->bool)->Self{
        Trigger {
            name: String::from(name),
            look_for,
            at,
            handle
        }
    }
    fn  line_start(&self, line:&String)->bool{
        //let no_of_chars = self.look_for.len();
        let no_of_chars = 4;
        let line_first:String = line.chars().take(no_of_chars).collect();
        if line_first == self.look_for {
            true
        } else {
            false
        }
    }
    fn line_end(&self, line:&String)->bool{
        true
    }
    fn anywhere(&self, line:&String)->bool{
        true
    }    
    fn run_trigger(&self,line:&String){
        (self.handle)();
    }
    pub fn execute(&self,line:&String){
        match self.at {
            At::LineStart=>{
                if self.line_start(line){
                    self.run_trigger(line);
                }
            },
            At::LineEnd=>{
                if self.line_end(line){
                    self.run_trigger(line);
                }
            },
            At::Anywhere=>{
                if self.anywhere(line){
                    self.run_trigger(line);
                }
            },
        }
    }
}

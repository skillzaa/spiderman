pub enum At {
    line_start,
    line_end,
    anywhere,
}
pub struct Trigger {
    name: String,
    look_for:String,
    at:At,
    handler:fn()->bool,
}

impl Trigger {
    pub fn new(name:&str,look_for:String,at:At,handler:fn()->bool)->Self{
        Trigger {
            name: String::from(name),
            look_for,
            at,
            handler
        }
    }
    pub fn process_line(&self,line:String){
        todo!();
    }
}

use crate::Recorders;
// #[derive(Debug)]
pub struct SpiderPack<'a>{
pub  recorders:&mut Recorders,
pub  current_line:String,
}
impl SpiderPack{
    pub fn new(recorders:&mut Recorders,current_line:String)->Self{
        SpiderPack {
          recorders, 
          current_line, 
        }
    }
}    
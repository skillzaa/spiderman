use crate::Recorders;
use crate::Flags;
// #[derive(Debug)]
pub struct SpiderPack{
  pub recorders :Recorders,
  pub flags :Flags,
  pub  current_line:String,
  pub  line_after_event_including:String,
  pub  line_after_event_excluding:String,
}
impl SpiderPack{
    pub fn new()->Self{
        SpiderPack {
          flags : Flags::new(),
          recorders : Recorders::new(),
          current_line : String::from(""), 
          line_after_event_including : String::from(""), 
          line_after_event_excluding : String::from(""), 
        }
    }
}    
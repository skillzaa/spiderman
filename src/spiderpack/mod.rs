use crate::Recorders;
// #[derive(Debug)]
pub struct SpiderPack{
  pub recorders :Recorders,
  pub  current_line:String,
  pub  line_after_event_including:String,
  pub  line_after_event_excluding:String,
}
impl SpiderPack{
    pub fn new()->Self{
        SpiderPack {
          recorders : Recorders::new(),
          current_line : String::from(""), 
          line_after_event_including : String::from(""), 
          line_after_event_excluding : String::from(""), 
        }
    }
}    
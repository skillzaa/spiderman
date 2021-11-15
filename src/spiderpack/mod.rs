use crate::Recordings;
// #[derive(Debug)]
pub struct SpiderPack{
pub  recordings:Recordings,
}
impl SpiderPack{
    pub fn new()->Self{
        SpiderPack {
          recordings:Recordings::new(),  
        }
    }
}    
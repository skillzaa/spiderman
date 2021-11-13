mod recording;
mod trigger;
mod process;
pub use recording::Recording;
pub struct At {u:u32}
impl At {
   pub fn new(u:u32)->Self{
       At{u}
   } 
   pub fn abc(&self,u:u32)->u32{
       u +100
   } 
   pub fn xyz(&self,u:u32)->u32{
    u + 200
   }
}
pub struct SpiderMan {
    recordings :Recording,
}
impl SpiderMan {
    // pub fn new(odd:fn(),even:fn())->Self{
    pub fn new()->Self{
        SpiderMan {
            recordings : Recording::new(),
            
        }  
    }

}
// pub fn welcome(f: fn() -> String)->Option<u32>{
pub fn welcome(){
let name = "abc";    
let mut spiderman = SpiderMan::new();
let tf = spiderman.recordings.add(name);
let rec = spiderman.recordings.get(name)
.unwrap();
rec.start();
rec.start();
rec.start();
rec.start();
rec.start();
println!("{:?}",rec.read());
}

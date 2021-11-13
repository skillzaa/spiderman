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
    string_data:String
}
impl SpiderMan {
    // pub fn new(odd:fn(),even:fn())->Self{
    pub fn new(string_data:String)->Self{
        SpiderMan {
            recordings : Recording::new(),
            string_data,
        }  
    }
    pub fn execute(&mut self){
        for the_line in self.string_data.lines(){
            process_line(the_line);
        }
    }
    

}
// pub fn welcome(f: fn() -> String)->Option<u32>{
fn process_line(the_line){


}
pub fn welcome(){
let name = "abc";    
let mut spiderman = SpiderMan::new("sssewsaz".to_string());
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

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
    odd:fn(),
    even:fn(),
    // my_at:At,
}
impl SpiderMan {
    pub fn new(odd:fn(),even:fn())->Self{
        SpiderMan {
            odd,
            even,
            // my_at : At::new(),
        }
    }
    pub fn look_for(&self,u:u32)->At{
        At::new(u)
    }
    pub fn run(&self){
    let a = [0,1,2,3,4,5,6,7,8,9];

    for n in a {
        if ( n % 2 == 0 ) {
            (&self.odd)();
        }
        else {
            (&self.even)();
        }
    }

    }
}
// pub fn welcome(f: fn() -> String)->String{
//     // let ff = f
//     let mut s = String::from("Hi...");
//     s.push_str(f().as_str());
//     s
// }

// pub fn run(){

// }
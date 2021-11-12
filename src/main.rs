mod lib;
fn main(){
    println!("Begin");
    
    let sm = lib::SpiderMan::new(odd,even);
    let total = sm.look_for(2).abc(5);

    sm.run();
    // let magic = lib::welcome(say_somthing);
    println!("{:?}",total);
    println!("End");
}

// fn say_somthing()->String{
//     String::from("My name is mike")
// }
fn odd(){
    println!("This is odd");
}
fn even(){
    println!("This is even");
}
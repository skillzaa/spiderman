use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::allow_numbers_only(&String::from("1234567890"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::allow_numbers_only(&String::from("abc!dews"));
// println!("{:?}",r);    
 assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::allow_numbers_only(&String::from("34dews"));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::allow_numbers_only(&String::from("*ab5432cdews"));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::allow_numbers_only(&String::from(" "));
assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::allow_numbers_only(&String::from("_1234"));
assert_eq!(false,r);
}
#[test]
fn g() {

let r = qndr::allow_numbers_only(&String::from("abcdefghijlkmnopqrstuvwxyzABCDEFGHIJKLMNOPQRESTUVWXYZ"));
    assert_eq!(false,r);
}
#[test]
fn h() {

let r = qndr::allow_numbers_only(&String::from("`~!@#$%^&*()_+abcdews"));
    assert_eq!(false,r);
}


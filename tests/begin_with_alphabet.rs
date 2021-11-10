use qndr;


#[cfg(test)]
#[test]
fn a() {

let r = qndr::begin_with_alphabet(&String::from("a1"));
    assert_eq!(true,r);
}
#[cfg(test)]
#[test]

fn b() {

let r = qndr::begin_with_alphabet(&String::from("lgtv5"));
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn c() {

let r = qndr::begin_with_alphabet(&String::from("Z4$#Ds5"));
assert_eq!(true,r);

}
#[cfg(test)]
#[test]

fn d() {

let r = qndr::begin_with_alphabet(&String::from(" Q432cdews "));
    assert_eq!(false,r);
}
#[cfg(test)]
#[test]

fn e() {

let r = qndr::begin_with_alphabet(&String::from(" "));
assert_eq!(false,r);}
#[cfg(test)]
#[test]

fn f() {

let r = qndr::begin_with_alphabet(&String::from("_1234?"));
assert_eq!(false,r);}
#[test]
fn g() {

let r = qndr::begin_with_alphabet(&String::from("!1!"));
    assert_eq!(false,r);
}
#[test]
fn h() {
 //space
let r = qndr::begin_with_alphabet(&String::from(" 5`~!@#$9%^&*()_+abcde6 "));
assert_eq!(false,r);
}


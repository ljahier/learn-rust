fn print_str(str: &String) -> () {
    println!("{}", str);
}

fn prend_et_rend(str: String) -> String {
    str
}

fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = prend_et_rend(String::from("world"));

    print_str(&s2);
    print_str(&s2);
    print_str(&s1);
    print_str(&s1);
    print_str(&s3);
    print_str(&s3);
}

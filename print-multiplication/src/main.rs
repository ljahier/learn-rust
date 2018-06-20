fn main() {
    multiplication_function();
}

fn multiplication_function() {
    for x in 1..11 {
        for y in 1..11 {
            print!("{:4}", x * y);
        }
        print!("\n");
    }
}
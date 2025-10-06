fn main() {
    box_1();
}

fn box_1() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;

    drop(a);
    let d = Box::new(1);
    let result2 = *b + *c + *d;
    println!("{result1} {result2}");
}

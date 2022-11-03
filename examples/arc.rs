use std::sync::Arc;

fn main() {

    let mut v = Arc::new(vec![1,2]);
    println!("{:#?}", v);
    v.push(3);

    println!("{:#?}", v);
    v.remove(0);
    println!("{:#?}", v);
}

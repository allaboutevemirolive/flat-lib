// https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

struct Value(i32);

impl Value {
    fn print(&self) {
        println!("it works! {}", self.0);
    }
}

fn main() {
    let v = Value(0);

    // variant to call print method
    v.print();          // 1
    let _ = &v.print(); // 2
    Value::print(&v);   // 3
    (&v).print();       // 4
}
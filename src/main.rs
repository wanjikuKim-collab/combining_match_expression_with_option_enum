fn main() {
    let current_age = Some(25);
}

fn add_one(x: Option<i32>) -> Option<i32>{
    match x {
        _ => None,
        Some(i) => Some(i + 1),
    }
}

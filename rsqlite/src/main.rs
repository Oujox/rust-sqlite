use rsqlite_macros::query;

fn main() {
    println!("{}", query!(2));
}
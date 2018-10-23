mod reader;

fn main() {
    let s = reader::read_file("text.txt");
    println!("{}", s);
}

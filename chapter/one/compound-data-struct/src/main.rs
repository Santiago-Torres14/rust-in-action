use std::str;

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    data: Vec<u8>,
}

impl<'a> File<'a> {
    fn new(name: &'a str, data: Vec<u8>) -> Self {
        Self { name, data }
    }

    fn read_data(&self) -> Option<&str> {
        str::from_utf8(&self.data).ok()
    }
}

fn main() {
    let f2 = File::new("file2", Vec::from([114, 117, 115, 116, 33]));

    println!("{:?}", f2.read_data());
}

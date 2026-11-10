use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    let mut rng = rand::rng();
    rng.random_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        read_length
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_mag = String::from("Permisson denied");
        return Err(err_mag);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_mag = String::from("Interrupted by signal!");
        return Err(err_mag);
    }
    Ok(f)
}

fn main() {
    let f4_data = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer = vec![];

    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer);
    let f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}

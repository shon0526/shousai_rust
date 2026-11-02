fn main() {
    let buffer = &mut [0u8; 1024];
    let user1 = "OK".to_string();
    read_secrets(&user1, buffer);
    store_secrets(buffer);

    let user2 = "Good".to_string();

    read_secrets(&user2, buffer);
    store_secrets(buffer);
}

fn read_secrets(user: &String, buf: &mut [u8]) {
    println!("buf {:?}", buf);
}
fn store_secrets(buf: &mut [u8]) {
    println!("buf {:?}", buf);
}

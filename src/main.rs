mod lib;

fn main() {
    let encoded = String::from("HB2NB2OISKNO3347UQVL7Y5VZV43WBDS");
    match lib::decode_to_string(encoded) {
        Some(r) => println!("{}", r),
        None => ()
    }
}

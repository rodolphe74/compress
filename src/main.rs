use std::fs::File;

use compress::packbits::{compress, uncompress};

fn main() {

    {
        let mut input = match File::open("test.bin") {
            Ok(f) => f,
            Err(e) => { 
                eprintln!("Erreur d'ouverture : {}", e);
                return;
            }
        };

        let mut output = match File::create("compressed.bin") {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Erreur d'ouverture : {}", e);
                return;
            }
        };

        compress(&mut input, &mut output);
    }

    {
        let mut input = match File::open("compressed.bin") {
            Ok(f) => f,
            Err(e) => { 
                eprintln!("Erreur d'ouverture : {}", e);
                return;
            }
        };

        let mut output = match File::create("uncompressed.bin") {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Erreur d'ouverture : {}", e);
                return;
            }
        };

        uncompress(&mut input, &mut output);
    }

}

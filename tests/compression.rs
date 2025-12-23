// use std::fmt::Result;
use std::io::{Read, Result};
use std::fs::File;
use std::io::{Write};
use rand::Rng;
use compress::packbits::{compress, uncompress};


#[test]
fn always_good() {
    assert_eq!(4, 2 + 2);
}

fn compare_files(f: &mut File, g: &mut File) -> Result<bool> {

    let mut b: [u8; 1] = [0];
    let mut c: [u8; 1] = [0];

    match f.read_exact(&mut b) {
        Ok(_) => {
            match g.read_exact(&mut c) {
                Ok(_) => {
                    if b[0] != c[0] {
                        return Ok(false);
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(e);
                }
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            return Err(e);
        }
    }

    Ok(true)
}

fn create_binary(number_of_blocks: usize, max_block_size: usize) -> Result<File> {
    let mut file = File::create("exemple_test.bin")?;
    let mut rng = rand::thread_rng();
    let mut buf: [u8;1] = [0];

    for _ in 0..number_of_blocks {
        let repeat_or_diff = rng.gen_bool(0.5);
        let block_size = rng.gen_range(1..=max_block_size);

        if repeat_or_diff {
            buf[0] = rng.r#gen();
            for _ in 0..block_size {
                file.write_all(&buf)?;
            }
        } else {
            for _ in 0..block_size {
                buf[0] = rng.r#gen();
                file.write_all(&buf)?;
            }
        }
    }

    Ok(file)
}

#[test]
fn test_compress() {
    for _ in 0..1000 {
        {
            let _ = match create_binary(10, 100) {
                Ok(f) => f, 
                Err(e) => {
                    eprintln!("{}", e);
                    assert_eq!("", e.to_string());
                    return;
                }
            };
        }   // close sample file

        {
            let mut f = match File::open("exemple_test.bin") {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{}", e);
                    assert_eq!("", e.to_string());
                    return;
                }
            };


            let mut g = match File::create("compressed_test.bin") {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{}", e);
                    assert_eq!("", e.to_string());
                    return;
                }
            };

            compress(&mut f, &mut g);
        }


        {
            let mut f = match File::open("compressed_test.bin") {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            let mut g = match File::create("uncompressed_test.bin") {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            uncompress(&mut f, &mut g);
        }

        {
            let mut f = match File::open("exemple_test.bin") {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            let mut g = match File::open("uncompressed_test.bin") {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            let is_same = compare_files(&mut f, &mut g);
            match is_same {
                Ok(b) => {
                    assert!(b);
                },
                Err(e) => {
                    eprintln!("{}", e);
                    assert_eq!("", e.to_string());
                    return;
                }
            }
        }
    }
}

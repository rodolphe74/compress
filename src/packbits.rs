#![allow(dead_code)]
use std::{fs::File, io::Read, io::Seek, io::SeekFrom};
use std::io::{self, Write};

fn read_ahead(f: &mut File, read_char: &mut u8) -> io::Result<i16> {
    let mut count_same :i16 = 1;
    let mut buffer: [u8; 1] = [0];

    let n = f.read(&mut buffer)?;
    let current_char = buffer[0];
    *read_char = buffer[0];
    if n == 0 { 
        return Ok(0);
    } 

    while let Ok(n) = f.read(&mut buffer) {


        if n == 0 {
            return Ok(-count_same);
        }

        else if count_same == 128 /*127*/ {
            break;
        }

        else if buffer[0] == current_char {
            count_same += 1;
        } 

        else {
            break;
        }

    }

    f.seek(SeekFrom::Current(-1))?;

    Ok(count_same)
}

fn display_buffer(b: &mut [u8; 128]) {
    for c in b {
        print!("  {};", c);
    }
    println!();
}

fn write_diff(f: &mut File, s: &[u8]) {
    let d: i8 = ((s.len() as u8) - 1) as i8;

    match f.write_all(&[d as u8]) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error {}", e);
            return;
        }
    }

    match f.write_all(s) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error {}", e);
        }
    }
}

fn write_same(f: &mut File, c: u8, s :usize) {
    let b = [c; 1];

    let e: i16 = 1 - s as i16;
    let d: i8 = e as i8;

    match f.write_all(&[d as u8]) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error {}", e);
            return;
        }
    }

    match f.write_all(&b) {
        Ok(()) => {},
        Err(e) => {
            eprintln!("Error {}", e);
        }
    }
}

pub fn compress(f: &mut File, g: &mut File) {
    let mut read_char: u8 = 0;
    let mut count_diff = 0;
    let mut diff_buffer :[u8; 128] = [0; 128]; 

    while let Ok(res) = read_ahead(f, &mut read_char) {

        if res < 0 {
            // end of file reached in read_ahead
            let last_res = -res;
            if last_res == 1 {
                diff_buffer[count_diff] = read_char;
                count_diff += 1;
                write_diff(g, &diff_buffer[0..count_diff]);
                // display_buffer(&mut diff_buffer);
            } else if last_res > 1 {
                if count_diff > 0 {
                    write_diff(g, &diff_buffer[0..count_diff]);
                    // display_buffer(&mut diff_buffer);
                }
                write_same(g, read_char, last_res as usize);
            }
            break;
        }

        else if res == 1 {
            diff_buffer[count_diff] = read_char;
            count_diff += 1;
            // TODO:
            if count_diff == 128 {
                write_diff(g, &diff_buffer[0..count_diff]);
                // display_buffer(&mut diff_buffer);
                count_diff = 0;
            }
        } else if res > 1 {
            if count_diff > 0 {
                write_diff(g, &diff_buffer[0..count_diff]);
                // display_buffer(&mut diff_buffer);
            } 
            write_same(g, read_char, res as usize);
            count_diff = 0;
        } else if res == 0 {
            // should not happen unless empty file
            break;
        }
    }
}

pub fn uncompress(f: &mut File, g: &mut File) {
    let mut buffer: [u8; 1] = [0];
    while let Ok(n) = f.read(&mut buffer) {
        if n == 0 {
            break;
        }
        else if n > 0 {
            let s: i16 = buffer[0] as i16;
            if s >= 128 {
                let count: i16 = 256 - s;
                match f.read(&mut buffer) {
                    Ok(_n) => {
                        for _ in 0..=count {
                            match g.write_all(&buffer) {
                                Ok(_) => {
                                },
                                Err(e) => {
                                    eprint!("{}", e);
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        return;
                    }
                }
            }
            else {
                let count: i16 = s;
                for _ in 0..=count {
                    match f.read(&mut buffer) {
                        Ok(_n) => {
                            match g.write_all(&buffer) {
                                Ok(_) => {
                                },
                                Err(e) => {
                                    eprint!("{}", e);
                                    return;
                                }
                            }

                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            return;
                        }
                    }
                }
            }
        }
    }
}

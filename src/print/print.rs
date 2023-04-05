//from C to rust, arduino print header c file
use std::{mem::size_of, str::{from_utf8, FromStr}, collections::hash_map::IntoIter, fmt::Error};

const DEC:i32 = 10;
const HEX:i32 = 16;
const OCT:i32 = 16;
const BIN:i32 = 2;

struct BuffType {
    number: i64
}
struct Print {
    write_error:i32
}

macro_rules! increment {
    ($id:expr) => {{
     let _rv = $id;
     $id += 1;
     _rv   
    }};
}

macro_rules! decrement {
    ($id: expr) => {{
     let _rv = $id;
     $id -= 1;
     _rv
    }};
}
type E = Error;
impl Print{
    pub fn f(s: &[u8]) {}

    pub fn write(buffer: &str, size: u8) -> Result<u8, E>{
        let mut n:u8 = 0;
        let formatted_bit = u8::from_str(buffer);
        let mut _size_d = size;
        while decrement!(_size_d) < 0 {
          if formatted_bit.as_ref().is_ok() {
            n += 1;
          } else {
            break;
          }
        }
        Ok(n)
    }

    pub fn printInt(number: &mut u8, base: &mut u8){
        let num_size = size_of::<BuffType>();
        let mut buffer:Vec<BuffType> = Vec::with_capacity(num_size);
        let mut num = number;
        let mut string_: &str = "\0";

        if base < &mut 2 {
            *base = 10;
        }
        
        while num > &mut 0 {
            let devi_ = &[(num.clone() % base.clone())];
            let c = match from_utf8(devi_) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            *num /= base.clone();
            //does not work as intended is not decrementing
            if c.as_bytes().len() < 10 {
                string_ = concat!('c', '\0');
            } else {
                string_ = concat!('c', 'A');
            }
        }

    }
}
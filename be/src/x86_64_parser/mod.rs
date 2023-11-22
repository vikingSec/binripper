



pub mod x86_64_parser {
    use std::{io::Error, fs};

    pub struct parser {
        pub bytes : Vec<u8>
    }
    impl parser {
        pub fn new() -> parser {
            parser {
                bytes: Vec::new()
            }
        }
        pub fn canparse() -> bool {
            // TODO: Figure out what a canparse function looks like for x86_64
            true
        }
        pub fn checkparse(&self)-> String {
            "works".to_string()
        }
        pub fn importfile(&mut self, bin : Vec<u8>) -> Result<usize, String>{
            if(bin.len() < 500){
                Err("Too small".to_string())
            }else{
                self.bytes = bin;
                Ok(self.bytes.len())
            }
        }
        pub fn loadfile(&mut self, fileloc: String) -> Result<usize, String> {
            let bytes = fs::read(fileloc);
            match bytes {
                Ok(b) => {
                    self.bytes = b;
                    Ok(self.bytes.len())
                },
                Err(e) => {
                    println!("{} is the loading error", e.to_string());
                    Err(e.to_string())
                }
            }
        }
        pub fn magicbytes(&self) -> Vec<u8>{
            let mut ret : Vec<u8> = vec![];
            ret.push(self.bytes[0]);
            ret.push(self.bytes[1]);
            ret
        }
    }
    
}
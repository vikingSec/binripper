use std::fs::{self, read, ReadDir};
use std::collections::HashMap;


fn firstlevelparse(filename: String) {
    let opcodes : HashMap<u8, String> = HashMap::from([
        (0xc7 , "MOV".to_string())
        ]);
    let fbytes = read(format!("./testfiles/{filename}")).unwrap();
    let magic = format!("{:x}{:x}",fbytes[0], fbytes[1]);
    println!("{} magic: {:?}", filename, magic);
    let mut index = 0;
    while index < fbytes.len() {
        let initialbyte = fbytes[index];
        let opcode = opcodes.get(&initialbyte);
        match opcode {
            Some(opcodestr) => {
                println!("{}", opcodestr);
            },
            None => {
                //println!("[x] opcode not implemented yet: {:x}", initialbyte)
            }

        }
        index+=1;
    }
    // for bytes in fbytes.chunks(2) {
    //     if bytes.len() > 1 {
    //         let short = u16::from_le_bytes([bytes[0], bytes[1]]);
    //         if short >> 2 == 0x00c7 {
    //             println!("{}\tMOV {:x}", short, short << 2)
    //         }
    //     }
    // }
}

fn main() {
    let testfiles = fs::read_dir("./testfiles");
    let allfiles: ReadDir;
    let mut filenames: Vec<String> = Vec::new();
    match testfiles {
        Ok(files) => {
            allfiles = files;
        }
        Err(_) => {
            println!("[x] There was an error");
            return;
        }
    }
    for f in allfiles.into_iter() {
        let name = f.unwrap().file_name().into_string().unwrap();
        println!("{:?}", name);
        firstlevelparse(name);
    }
}

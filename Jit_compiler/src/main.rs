extern crate libc;

use std::mem;
use std::ops::{Index, IndexMut};
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use mac_address::get_mac_address;
// use std::process::Command;
use std::option::Option;


extern {
    // using libc
    fn memset(s: *mut libc::c_void, c: u32, n: libc::size_t) -> *mut libc::c_void;
}

const PAGE_SIZE: usize = 4096;

struct JitMemory {
    contents : *mut u8
}

impl JitMemory {
    /*
    implementation, using libc so that we can have executable memory
    */
    fn new(num_pages: usize) -> JitMemory {
        let contents : *mut u8;
        unsafe {
            let size = num_pages * PAGE_SIZE;
            let mut _contents : *mut libc::c_void = mem::uninitialized();
            libc::posix_memalign(&mut _contents, PAGE_SIZE, size);
            libc::mprotect(_contents, size, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
            memset(_contents, 0xc3, size);  // for now, prepopulate with 'RET'
            contents = mem::transmute(_contents);
        }
        JitMemory { contents: contents }        
    }
}

impl Index<usize> for JitMemory {
    type Output = u8;
    fn index(&self, _index: usize) -> &u8 {
        unsafe {&*self.contents.offset(_index as isize) }
    }
}

impl IndexMut<usize> for JitMemory {
    fn index_mut(&mut self, _index: usize) -> &mut u8 {
        unsafe {&mut *self.contents.offset(_index as isize) }
    }
}

fn run_jit(binary: &Vec<u8>) -> (fn() -> i64) {
    /*
    runs executable char code from memory
    @param binary : the vector containing the instructions to run
    @ret : unsafe acces to memory unsing a function to run the code
    */

    let mut jit : JitMemory = JitMemory::new(binary.len()/PAGE_SIZE + 1);
    // copy the executable char code into jit struct
    for i in 0..binary.len(){
        jit[i] = binary[i];
        print!("{}", jit[i]);
    }
    unsafe { mem::transmute(jit.contents) }
}

fn create_key() -> Vec<u8> {
    /*
    Creates a key to encode a binary.
    @ret : a key which correponds to a unique mac address from network hardware
    */

    let mut key:Vec<u8> = Vec::new();
    let key = match get_mac_address() {
        Ok(Some(mac_addr)) => {
            // println!("MAC addr = {}", mac_addr);
            // println!("bytes = {:?}", mac_addr.bytes());
            for e in mac_addr.bytes().iter(){
                key.push(*e);
            }
            key
        }
        Ok(None) => panic!("No MAC address found."),
        Err(e) => panic!("{:?}", e),
    };
    // println!("{:?}", key);
    key
}

fn encrypt_binary(input_filename: &str, output_filename: Option<&str>) -> io::Result<Vec<u8>>{
    /*
    This function encrypt an file, and produce the output the encrypted file
    @param input_filename : the name of the file to crypt
    @param output_filename : the name of the new crypted file
    @ret : a result, it's ok if the files are correctly handled without trouble
    */
    
    let mut f = File::open(input_filename)?;
    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;
    // println!("{:?}", buffer);
    let key = create_key();
    // vector where will be stored the bytes of the output file
    let mut res:Vec<u8> = Vec::new();
    for i in 0..buffer.len(){
        // so that the key is not found easily
        match buffer [i] {
            0 => res.push(0),
            _ => {
                if buffer[i] == key[i%key.len()] {
                    res.push(buffer[i]);
                } else {
                    res.push(buffer[i] ^ key[i%key.len()]);
                }
            }
        }
    }
    // println!("{:?}", res);
    // create a file from the modified bytes
    match output_filename {
        Some(output_filename) => {
            let mut out = File::create(output_filename)?;
            out.write_all(&res)?;
        },
        None => {}
    }
    Ok(res)
}

fn main() ->io::Result<()> {
    // read the file from the commad line and the output's name
    let args: Vec<String> = env::args().collect();
    match args.len(){
        2 => (),
        _ => panic!("please input an input file as frist arg,
        and a name for the output file as second arg"),
    }
    let input_filename = &args[1];
    let bin = encrypt_binary(input_filename, None)?;

    let func = run_jit(&bin);
    println!("{}", func());

    Ok(())
}

// fn _change_file_permissions(filename: &str){
//     /* 
//     Changes the inupt file permissions, adding executable
//     */
//     Command::new("chmod")
//     .arg("+x")
//     .arg(filename)
//     .output()
//     .expect("failed to change file permissions");
// }

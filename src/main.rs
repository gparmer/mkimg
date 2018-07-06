extern crate toml;
extern crate pipers;
#[macro_use]
extern crate serde_derive;
extern crate xmas_elf;

mod cossystem;
mod syshelpers;
mod compobject;
mod symbols;
mod compose;

use compose::{ComposeSpec, Compose};
use compobject::{CompObject, ClientSymb, ServerSymb};
use std::env;
use std::process;

fn comp_print<'a>(comp: &'a CompObject) {
    println!("Component {}:", comp.name());
    println!("\t__cosrt_comp_info @ 0x{:x}, __cosrt_upcall_entry @ 0x{:x}", comp.compinfo_addr(), comp.entryfn_addr());
    println!("\tDepended on client stubs:");
    comp.dependencies().iter().for_each(|ref s| println!("\t\t{} @ 0x{:x}, ucap @ 0x{:x}", s.name(), s.func_addr(), s.ucap_addr()));
    println!("\tExported server stubs:");
    comp.exported().iter().for_each(|ref s| println!("\t\t{} @ 0x{:x}", s.name(), s.addr()));
}

pub fn main() -> () {
    let mut args = env::args();
    let program_name = args.next();

    let arg1 = args.next();
    if let None = arg1 {
        println!("usage: {} <sysspec>.toml", program_name.unwrap());
        process::exit(1);
    }

    match ComposeSpec::parse_spec(arg1.unwrap()) {
        Ok(sysspec) => {
            let sys = Compose::parse_binaries(&sysspec).unwrap();
            sys.components().iter().for_each(|ref c| comp_print(&c));
            println!("System Specification:\n{:#?}", sysspec.sysspec_output())
        },
        Err(s) => println!("{}", s)
    }
}

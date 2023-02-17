// This is a build script to generate the field table.

use std::{env, fs, path::Path};

pub struct ElementTable {
    pub exp: [u16; 0x8000],
    pub log: [u16; 0x8000],
}

#[must_use]
pub fn gen_table() -> ElementTable {
    let mut table = ElementTable {
        exp: [0; 0x8000],
        log: [0; 0x8000],
    };

    table.exp[0] = 1;
    for i in 1..0x8000 {
        let mut tmp = u32::from(table.exp[i - 1]) * 2;
        if tmp & 0x8000 != 0 {
            tmp ^= 0x8003 /* GaloisField2p15.POLY */;
        }

        table.exp[i] = tmp as u16;
    }

    for (i, e) in table.exp.iter().enumerate() {
        table.log[*e as usize] = i as u16;
    }

    table
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("table.rs");

    let table = gen_table();

    fs::write(
        dest_path,
        format!(
            "pub mod table{{
             pub struct ElementTable{{pub exp: [u16; 0x8000],pub log: [u16; 0x8000]}}
             pub const TABLE:ElementTable=ElementTable{{
             exp: {:?},
             log: {:?},
             }};}}",
            table.exp, table.log
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

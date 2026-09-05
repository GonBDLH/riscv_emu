use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[cfg(feature = "semihosting")]
const DIR: &'static str = "../semihosting_elf_tests";

#[cfg(feature = "semihosting")]
fn generate_tests(file: &mut File, test_name: &str, name: &str) {
    writeln!(
        file,
        r#"
#[test]
#[timeout(5000)]
fn {test_name}() {{
    let mut interpreter = Interpreter::new_test_elf("{DIR}/{name}", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}}
"#
    )
    .unwrap();
}

fn main() {
    let out_path = Path::new("src/tests.rs");

    let mut file = File::create(out_path).expect("No se pudo crear src/tests.rs");

    // Cabecera del módulo tests y las importaciones necesarias
    writeln!(
        file,
        r#"
#![allow(non_snake_case)]

use crate::interpreter::Interpreter;
use ntest::timeout;
"#
    )
    .unwrap();

    let paths = fs::read_dir(DIR).expect("No se pudo leer el directorio tests");

    for entry in paths {
        let path = entry.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let test_name = name.replace("-", "_").replace(".", "_");

            generate_tests(&mut file, &test_name, name);
        }
    }

    // // Cierre del módulo tests
    // writeln!(file, "}}").unwrap();
}

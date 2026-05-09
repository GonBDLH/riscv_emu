use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[cfg(feature = "semihosting")]
fn generate_tests(file: &mut File, test_name: &str, name: &str) {
    writeln!(
        file,
        r#"
#[test]
#[timeout(2000)]
fn {test_name}() {{
    let mut interpreter = Interpreter::new_test_elf("semihosting_elf_tests/{name}", 0);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0x20026);
}}
"#
    )
    .unwrap();
}

#[cfg(feature = "hitf")]
fn generate_tests(file: &mut File, test_name: &str, name: &str) {
    writeln!(
        file,
        r#"
#[test]
#[timeout(2000)]
fn {test_name}() {{
    let hitf_size = if "{name}".contains("-v-") {{
        8
    }} else {{
        4
    }};

    let mut interpreter = Interpreter::new_test_elf("hitf_elf_tests/{name}", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret.unwrap(), 0);
}}
"#
    )
    .unwrap();
}

#[cfg(feature = "hitf")]
const DIR: &'static str = "hitf_elf_tests";
#[cfg(feature = "semihosting")]
const DIR: &'static str = "semihosting_elf_tests";

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

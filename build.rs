use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let out_path = Path::new("src/tests.rs");

    let mut file = File::create(out_path).expect("No se pudo crear src/tests.rs");

    // Cabecera del módulo tests y las importaciones necesarias
    writeln!(
        file,
        r#"
use crate::interpreter::Interpreter;
"#
    )
    .unwrap();

    let paths = fs::read_dir("rv_tests").expect("No se pudo leer el directorio tests");

    for entry in paths {
        let path = entry.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.ends_with(".hex") {
                let test_name = name
                    .trim_end_matches(".hex")
                    .replace("-", "_")
                    .replace(".", "_");

                let mut to_host: u32 = 0x80001000;

                if test_name == "rv32ui_p_ld_st" {
                    to_host = 0x80002000;
                }

                writeln!(
                    file,
                    r#"
#[test]
fn {test_name}() {{
    let mut interpreter = Interpreter::new();
    interpreter.load_test("rv_tests/{name}");
    interpreter.run();
    assert_eq!(interpreter.read_test_result({to_host:#010X}), 1);
}}
"#
                )
                .unwrap();
            }
        }
    }

    // // Cierre del módulo tests
    // writeln!(file, "}}").unwrap();
}

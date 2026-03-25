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
use ntest::timeout;
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
#[timeout(2000)]
fn {test_name}() {{
    use std::panic;

    let result = panic::catch_unwind(|| {{
        let mut interpreter = Interpreter::new_test({to_host:#08X});
        interpreter.load_hex("rv_tests/{name}");
        interpreter.run();
    }});

    let err = result.expect_err("Expected panic");

    let msg = err.downcast_ref::<&str>()
        .map(|s| *s)
        .or_else(|| err.downcast_ref::<String>().map(|s| s.as_str()))
        .unwrap();

    assert!(msg.contains("PASS"));
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

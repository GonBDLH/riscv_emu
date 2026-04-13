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

    let paths = fs::read_dir("elf_tests").expect("No se pudo leer el directorio tests");

    for entry in paths {
        let path = entry.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let test_name = name.replace("-", "_").replace(".", "_");

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

    let mut interpreter = Interpreter::new_test_elf("elf_tests/{name}", hitf_size);
    let ret = interpreter.run();

    assert_eq!(ret, 0);
}}
"#
            )
            .unwrap();
        }
    }

    // // Cierre del módulo tests
    // writeln!(file, "}}").unwrap();
}

use brainfuck_rust::{run, Config};

#[test]
fn test_hello_world() {
    let config = Config {
        brainfuck_file_path: String::from("tests/hello_world.bf"),
    };
    match run(config) {
        Ok(s) => assert_eq!(String::from("Hello World!\n"), s),
        _ => assert!(
            false,
            "Expected \"Hello World!\\n\" as a return value from brainfuck::run"
        ),
    }
    assert!(true);
}

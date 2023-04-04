pub struct Config {
    pub brainfuck_file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("no path to brainfuck file provided.");
        }

        let brainfuck_file_path = args[1].clone();

        Ok(Config {
            brainfuck_file_path,
        })
    }
}

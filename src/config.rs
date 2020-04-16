pub struct Config {
    pub root_dir: String,
}

impl Config {
    pub fn new<I>(args: I) -> Result<Config, &'static str>
    where I: IntoIterator<Item=String> {
        let mut iter = args.into_iter();

        iter.next();

        let root_dir = match iter.next() {
            Some(arg) => arg,
            None => return Err("Usage: json-file-walker root_dir"),
        };

        Ok(Config { root_dir })
    }
}
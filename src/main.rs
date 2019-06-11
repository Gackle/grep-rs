use std::env;
use std::process;
use std::fs;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else 定义于标准库的 Result<T, E> 上。
    // 当 Result 是 Ok 时，这个方法类似于 unwrap ： 它返回 Ok 内部封装的值。
    // 然而，当其值为 Err 时，该方法会调用一个闭包（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。
    // 传递给闭包中位于两道竖线间的参数 err。闭包中的代码在其运行时可以使用这个 err 值。
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())  // 返回 unit 类型 ()
}
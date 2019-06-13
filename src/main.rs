use std::env;
use std::process;

use grep_rs::Config;


fn main() {
    // unwrap_or_else 定义于标准库的 Result<T, E> 上。
    // 当 Result 是 Ok 时，这个方法类似于 unwrap ： 它返回 Ok 内部封装的值。
    // 然而，当其值为 Err 时，该方法会调用一个闭包（closure），也就是一个我们定义的作为参数传递给 unwrap_or_else 的匿名函数。
    // 传递给闭包中位于两道竖线间的参数 err。闭包中的代码在其运行时可以使用这个 err 值。
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = grep_rs::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

use std::process::exit;

pub fn function_error(msg: &str, namespace: &str, function: &str, file: &str, pos: (usize, usize)) {
    let (line, col) = pos;
    println!("[Error] Function {}.{}: {}\n{}: {},{}\n", namespace, function, msg, file, line, col);
    exit(-1);
}

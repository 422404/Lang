use std::process::exit;

pub fn class_error(msg: &str, namespace: &str, class: &str, file: &str, pos: (usize, usize)) {
    let (line, col) = pos;
    println!("[Error] Class {}.{}: {}\n{}: {},{}\n", namespace, class, msg, file, line, col);
    exit(-1);
}

pub fn class_member_error(msg: &str, namespace: &str, class: &str, member: &str, file: &str, pos: (usize, usize)) {
    let (line, col) = pos;
    println!("[Error] Class member {}.{}.{}: {}\n{}: {},{}\n", namespace, class, member, msg, file, line, col);
    exit(-1);
}
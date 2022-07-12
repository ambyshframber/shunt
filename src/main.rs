use std::io::{stdin, BufRead};
use std::collections::HashMap;

const CONSTS: &[&str] = &["e", "pi"];

fn main() {
    let mut op_stack: Vec<String> = Vec::new();
    let mut operators = HashMap::new();
    operators.insert("+", (2, false));
    operators.insert("-", (2, false));
    operators.insert("*", (3, false));
    operators.insert("/", (3, false));
    operators.insert("**", (4, false));

    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let line = line.trim();
        //eprintln!("{}", line);
        if line.is_empty() {
            continue
        }
        if line.parse::<f64>().is_ok() || CONSTS.iter().any(|c| *c == line) {
            print!("{} ", line)
        }
        else { // operator / function / bracket
            if line == ")" {
                'empty_ops: loop {
                    assert!(op_stack.len() > 0);
                    let op = op_stack.pop().unwrap();
                    if op == "(" {
                        break 'empty_ops
                    }
                    else {
                        print!("{} ", op)
                    }
                }
            }
            else if let Some((self_prec, self_r_assoc)) = operators.get(line) {
                while let Some(op) = op_stack.pop() {
                    if op == "(" {
                        op_stack.push(op);
                        break
                    }
                    else {
                        let (precedence, _) = operators.get(op.as_str()).unwrap_or(&(0, false));
                        if precedence > self_prec || (precedence == self_prec && !self_r_assoc) {
                            print!("{} ", op)
                        }
                    }
                }
                op_stack.push(line.into())
            }
            else { // lbrac or function
                op_stack.push(line.into())
            }
        }
    }

    for op in op_stack {
        assert_ne!(op, ")");
        print!("{} ", op)
    }
    println!(".")
}



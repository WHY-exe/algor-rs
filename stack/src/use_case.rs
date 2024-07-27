use crate::stack::Stack;

pub fn check_par(par: &str) -> bool {
    let mut stack_local = Stack::new();
    let mut balance = true;
    for i in par.chars() {
        if i == '(' {
            stack_local.push(i);
        } else if i == ')' {
            if stack_local.is_empty() {
                balance = false;
            } else {
                stack_local.pop();
            }
        }
    }
    return stack_local.is_empty() && balance;
}

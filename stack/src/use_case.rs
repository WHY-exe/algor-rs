use crate::stack::Stack;

fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closes = ")]}";
    if opens.find(open) == None || closes.find(close) == None {
        return false;
    }
    opens.find(open) == closes.find(close)
}

fn iter_par(par: &str, par_opens: &str, par_closes: &str) -> bool {
    let mut stack_local = Stack::new();
    let mut balance = true;
    for i in par.chars() {
        if par_opens.find(i) != None {
            stack_local.push(i);
        } else if par_closes.find(i) != None {
            if let Some(open) = stack_local.pop() {
                if par_match(open, i) {
                    continue;
                }
            }
            balance = false;
            break;
        }
    }
    stack_local.is_empty() && balance
}

pub fn check_par(par: &str) -> bool {
    iter_par(par, "(", ")")
}

pub fn check_par2(par: &str) -> bool {
    iter_par(par, "([{", ")]}")
}

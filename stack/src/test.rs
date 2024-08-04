#[cfg(test)]
#[test]
fn test_stack_push() {
    use crate::stack::Stack;

    let mut test_stack = Stack::new();
    for i in 0..5 {
        test_stack.push(i);
    }
    assert_eq!(*test_stack.peek().unwrap(), 4);

    test_stack.pop();
    assert_eq!(*test_stack.peek().unwrap(), 3);
}

#[test]
fn test_check_par() {
    use crate::use_case::check_par;
    assert_eq!(check_par("()((123456))()"), true);
    assert_eq!(check_par("(123((123456))(231)32)321)1231(1231)"), false);
}

#[test]
fn test_check_par2() {
    use crate::use_case::check_par;
    assert_eq!(check_par("{()([][(123456))]()}"), true);
    assert_eq!(
        check_par("(123((12345[[]]6{}){)(2}31)32)32][1)1231(1231)"),
        false
    );
}

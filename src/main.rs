fn main() {
    let mut a:i32 = -10;
    let mut b:i32 = -20;
    println!("Before Swap: a is {}",a);
    println!("Before Swap: b is {}",b);
    swap_numbers(&mut a,&mut b);
    println!("After Swap: a is {}",a);
    println!("After Swap: b is {}",b);
}

fn swap_numbers(a: &mut i32,b: &mut i32) {
    *a = *a+*b;
    *b = *a-*b;
    *a = *a-*b;
    
}

#[test] // Test for normal swap case
fn test_swap1() {
    let mut a = 10;
    let mut b = 20;
    swap_numbers(&mut a,&mut b);
    assert_eq!(a,20);
    assert_eq!(b,10);
}

#[test] //Test for same values
fn test_swap2() {
    let a = 10;
    let b = 10;
    assert_eq!(a,10);
    assert_eq!(b,10);
}

#[test] //Test for -ve values 
fn test_swap3() {
    let mut a = -10;
    let mut b = -20;
    swap_numbers(&mut a, &mut b);
    assert_eq!(a,-20);
    assert_eq!(b,-10);
}

#[test] // Test for '0' value
fn test_swap4() {
    let mut a = 10;
    let mut b = 0;
    swap_numbers(&mut a, &mut b);
    assert_eq!(a,0);
    assert_eq!(b,10);
}

#[test] // Test for one -ve value
fn test_swap5() {
    let mut a = -10;
    let mut b = 20;
    swap_numbers(&mut a, &mut b);
    assert_eq!(a,20);
    assert_eq!(b,-10);
}
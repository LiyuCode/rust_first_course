//functions as a variable

fn apply(value: i32, f: fn(i32) -> i32) -> i32{
    f(value)
}

fn square(value: i32) -> i32{
    value * value
}

fn cube(value: i32) -> i32{
    value * value * value
}

fn double(value: i32) -> i32{
    value + value
}

fn apply_and_return(value: i32, f: fn(i32) -> i32) -> (i32,i32){
    (f(value),value)
}

fn addsub(x:isize,y:isize) ->(isize,isize){
    (x+y,x-y)
}


fn main() {
    println!("Apply square:{}",apply(2, square));
    println!("Apply cube:{}",apply(2, cube));
    println!("Apply double:{}",apply(2, double));
    let (a,s)=apply_and_return(2,double);
    println!("Apply double:{} and return:{}",a,s);
}

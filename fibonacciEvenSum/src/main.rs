fn main() {
    let mut first = 1;
    let mut second = 2;
    let mut temp = 0;
    let mut sum = 0;
    while second <= 4000000 {
        if second % 2 == 0 {
            sum += second;
        }
        temp = second;
        second = first + second;
        first = temp;
    }
    print!("{}", sum);
}

/*
fn fibonacci(sum: i64){

}
*/

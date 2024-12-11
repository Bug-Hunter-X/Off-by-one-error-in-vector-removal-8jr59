fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut i = 0;
    while i < numbers.len() {
        if numbers[i] % 2 == 0 {
            numbers.remove(i);
        } else {
            i += 1;
        }
    }
    println!("Numbers: {:?}", numbers);
}
//Alternative solution using iterators:
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers = numbers.into_iter().filter(|&x| x % 2 != 0).collect::<Vec<_>>();
    println!("Numbers: {:?}", numbers);
} 
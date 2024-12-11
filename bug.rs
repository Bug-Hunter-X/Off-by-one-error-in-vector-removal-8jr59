fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let mut index = 0;

    while index < numbers.len() {
        if numbers[index] % 2 == 0 {
            numbers.remove(index);
        } else {
            index += 1;
        }
    }

    println!("Numbers: {:?}", numbers);
}
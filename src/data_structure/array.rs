pub fn arrays() {
    let numbers: [i32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("The first number is: {}", numbers[0]);
    println!("The second number is: {}", numbers[1]);

    let length = numbers.len();
    println!("The length of the array is: {}", length);

    println!("All numbers in the array:");
    for &number in &numbers {
        println!("{}", number);
    }
}

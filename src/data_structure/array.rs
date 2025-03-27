pub fn arrays() {
    let ordered_number: [i32; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let unordered_number: [i32; 10] = [100, 61, 25, 34, 53, 27, 91, 88, 76, 11];

    println!("The first number is: {}", ordered_number[0]);
    println!("The second number is: {}", ordered_number[1]);

    let length = ordered_number.len();
    println!("The length of the array is: {}", length);

    println!("All ordered_number in the array:");
    for &number in &ordered_number {
        print!("{}\t", number);
    }

    println!();
    
    println!("The first number is: {}", unordered_number[0]);
    println!("The second number is: {}", unordered_number[1]);

    let length = unordered_number.len();
    println!("The length of the array is: {}", length);

    println!("All unordered_number in the array:");
    for &number in &unordered_number {
        print!("{}\t", number);
    }
    println!();
}

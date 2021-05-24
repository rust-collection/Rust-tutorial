fn main() {
    // 数组
    // Comma-separated list inside brackets
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    // Print first weekday
    println!("First weekday is {}", weekdays[0]); 
  
    // Initialize an array of 512 elements where every element is a zero
    let byte_buffer = [0_u8; 512];
    // Print the 100th element in the buffer
    println!("Buffer element 100 is {}", byte_buffer[99]);

    // 向量
    let three_numbers = vec![1,2,3];
    println!("Initial vector: {:?}", three_numbers);

    let ten_zeros = vec![0;10];
    println!("Ten zeros: {:?}", ten_zeros);

    let mut new_v = Vec::new();
    new_v.push(5);
    new_v.push(1);
    new_v.push(3);
    new_v.push(2);
    println!("vector: {:?}", new_v);
    let x = new_v.pop();
    println!("vector: {:?}", new_v);
    println!("{:?}", x);

    // 哈希表
    use std::collections::HashMap;
    
    let mut book_reviews: HashMap<String, String> = HashMap::new();
    
    // Add book reviews
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );
    if !book_reviews.contains_key("Les Misérables") {
        println!("{} reviews found. No reviews found for Les Misérables.", book_reviews.len());
    }
    let sherlock = "The Adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), false);
    
    // Verify review was removed
    if !book_reviews.contains_key("The Adventures of Sherlock Holmes") {
        println!("{} reviews found. No reviews found for The Adventures of Sherlock Holmes.", book_reviews.len());
    }
}

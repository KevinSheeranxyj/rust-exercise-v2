fn main() {
    let loop_with_array: [i32; 5] = [1,222,34,4,44];

    for item in loop_with_array.iter() {
        println!("For loop with arrays: {}", item);
    }

    let loop_with_range = 0..4;

    for item in loop_with_range.into_iter() {
        println!("For loop with range: {}", item);
    }


    let loop_with_vector = vec![1,2,33,4,4];

    for item in loop_with_vector {
        println!("For loop with vector: {}", item);
    }

}

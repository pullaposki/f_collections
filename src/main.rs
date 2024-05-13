fn main() {
    let array1 = ["One", "Two"]; 
    let array2 = ["Four", "Five", "Six"]; 
    println!("Array1: {:?} and Array2: {:?}", array1, array2);

    let array3 = ["a";10 ];
    println!("Array3: {:?}", array3);

    let array_of_ten = [1,2,3,4,5,6,7,8,9,10];

    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!("Three to Five: {:?}", three_to_five);
    println!("Start at Two: {:?}", start_at_two);
    println!("End at Five: {:?}", end_at_five);
    println!("Everything: {:?}", everything);

    // Slices work with Strings:

    let s = String::from("Hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice2 = &s[1..];

    println!("First slice: {} and second slice {}", slice, slice2);

    // Vectors are to arrays what Strings are to &str

    let name1 = String::from("John");
    let name2 = String::from("Doe");

    let mut my_vec = Vec::new(); // Vectors need to have a type associated with them. 

    my_vec.push(name1);
    my_vec.push(name2);

    println!("{:?}", my_vec);

    let vec_of_ten = vec![1,2,3,4,5,6,7,8,9,10];
    let three_to_five = &vec_of_ten[2..5];
    println!("Three to Five (vector): {:?}", three_to_five);

    // Tuples are collections that can house multiple types.
    // Empty function is a tuple.

    let random_tuple = ("Here is a name",8,vec!['a'],'b',[8,9,10], 7.7);
    println!("Inside the tuple are: 1. {} 2. {} 3. {:?} 4. {} 5. {:?} 6. {}", random_tuple.0, random_tuple.1, random_tuple.2, random_tuple.3, random_tuple.4, random_tuple.5);

}

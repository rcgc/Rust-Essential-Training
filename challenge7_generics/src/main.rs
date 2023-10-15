/*
    generics are a zero cost abstraction

    Box<T> to store values that cannot be known at compile time

    The Box data type consists of a pointer 
    on the stack that points to a chunk of 
    memory on the heap that has been allocated 
    for the data. When the box goes out of scope,
    it will automatically drop and deallocate
    the memory it was using on the heap.

    This challenge adds two numbers stored within
    Box<T> objects
*/
fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T>{
    Box::new(*a + *b)
}

fn main() {
    let one = Box::new(1);
    let two = Box::new(2);

    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(3.14159);
    let e = Box::new(2.71828);
    assert_eq!(*sum_boxes(pi, e), 5.85987);

    println!("Tests passed");
}

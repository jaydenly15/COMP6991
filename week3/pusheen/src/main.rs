fn main() {
    // the program doesnt compile because rust enforces that there could only be immutable reference to some data at any time
    // this is to prevent data races from happening, whereby two or more pointers access the data with at least one pointer
    // attempting to change the data
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let a = &mut vec;
    a.push(11);

    let b = &mut vec;
    b.push(12);

    // the modified code now compiles properly because the immutable reference a lasts until a.push(11) and no longer exists after that
    // we can now assign an immutable reference of vec to b and modify it accordingly.

}

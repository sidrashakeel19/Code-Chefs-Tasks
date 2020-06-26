// Let's try to work on a famous fun programming problem based on the popular 
// 99 bottles of the beer song. 

// Song: 
// 99 bottles of beer on the wall, 99 bottles of beer.
// Take one down and pass it around, 98 bottles of beer on the wall.

// 98 bottles of beer on the wall, 98 bottles of beer.
// Take one down and pass it around, 97 bottles of beer on the wall.
// ……...
// 3 bottles of beer on the wall, 3 bottles of beer.
// Take one down and pass it around, 2 bottles of beer on the wall.

// 2 bottles of beer on the wall, 2 bottles of beer.
// Take one down and pass it around, 1 bottle of beer on the wall.

// 1 bottle of beer on the wall, 1 bottle of beer.
// Take it down and pass it around, no more bottles of beer on the wall.

// No more bottles of beer on the wall, no more bottles of beer.
// Go to the store and buy some more, 99 bottles of beer on the wall.


fn main() {
    let mut count = 1;
    for i in (0..100).rev() {
        if i != 0 {
            println!("\n{} bottles of beer on the wall, {} bottles of beer.",i,i);
            println!("Take one down and pass it around, {} bottles of beer on the wall.",i-1);
            count = count + 1;
        }
        else if i == 0 {
            println!("\nNo more bottles of beer on the wall, no more bottles of beer.");
            println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
        }
    }
}
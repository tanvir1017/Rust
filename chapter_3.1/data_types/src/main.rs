fn main() {
    

    let a;

    a = 10;
    
    // a = 20; //  will give error because the a is not mutable and you might be think of a = 10 what about that. In there the a = 10 is assigned that's why no error will given

    let a: i8 = 127 ; // Integer data type. Who will not accept any fraction or (.) in number

    let str = "STRING";  // string type.

    let floating_point  = 2.3; // Floating point number. It should be a number with (. -> dot);

    let boolean_type = false; // boolean type

    // Unsigned and Insigned
    let unsigned_number = 250u8; // suffixing  type by adding with value

    let insigne_number = -128i8; // suffixing type by subtracting


    // We can add underscore(der Unterstrich) to visualize and for improve better readability. Like

    let readAble_number = -128_i8; // suffixing type by subtracting

    let readAble_number = 248_u8; // suffixing type by subtracting

    // for best upcases
    let one_cr = 1_00_00_000_u32; // Now its more readable as 1cr
    println!("{}", one_cr);


    // Integer Overflow -> Panic!

    let integer_overflow_example = match  integer_overflow().checked_add(0){
        Some(num) => num,
        None => {
            println!("cannot read number");
            return;
        }
    } ;
    //println!("🚀 ~ integer_overflow_example:{}", integer_overflow_example);


    // TODO -> Floating number. There are two floating number in RUST
    // -  f32 -> Take less decimal numbers than f64
    // -  f64 -> Take more decimal numbers than f64

    let f64 = 2.32323435345245234523521343623456;  // take 16 digit after decimal point
    let f32:f32 = 2.32323435345245234523521343623456; // take only 7 digit after decimal point
    println!("This is my f64: {:?}", f64);
    println!("This is my f32: {:?}", f32);

    



    let x:f32 = 5.0 / 2.0;
    let x:f32 = 5_f32 / 2_f32;
    println!("This is my x: {:?}", x);


    let c:&str = "jadkfasdfz";
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    println!("{:?} {:?} {:?}", c, z, heart_eyed_cat); 


    // ________________________TUPLE_______________________________________
    let tanvir: (&str, i32, bool, f64) = ("Tanvir Hossain", 21, true, 3.5);
    let (x, y, z, u) =  tanvir;

    println!("{x} {y} {z} {u}");
    // We can also use this syntax
    println!("by indexing we can also print tuple {}", tanvir.0);
    println!("by indexing we can also print tuple {}", tanvir.1);
    println!("by indexing we can also print tuple {}", tanvir.2);
    
    
    // ________________________TUPLE_______________________________________
    
    // ________________________UNIT_______________________________________
    let unit = ();
    println!("{:?}", unit); // an empty tuple called unite -> ()
    // ________________________UNIT_______________________________________
    
    
    // ________________________ARRAY_______________________________________
    let arr = [5, 30, 32];
    println!("array's element by index number {}", arr[0]);
    // ------------------Syntax Sugar------------------
    let arr: [i32; 5] = [10; 5]; // just like 5 10 in one array [10, 10, 10, 10, 10] 

    println!("array's element by index number {}", arr[4]);

    // ________________________ARRAY_______________________________________
    
    


}

/*

// How bits work in RUST :i32 :u8 -> what does it mean

The formula is =>
from -(2n - 1) to ((2n-1) - 1)
[!Link](https://doc.rust-lang.org/book/ch03-02-data-types.html)

by the formula => :i8 means -> -(28-1) to ((28-1) -1)
By the way the n is a power which represent the data type bits

so after calculating -128 to 128 is the value that means we can store value from -128 to 128. and if we go for 129 ti will give us an error

an addition :u8 follow this formula. Because of :u8 doesn't take any negative value
it can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255. 2*2*2*2*2*2*2*2 = 256 -1 => value < 256 that means  from 0 - 255


++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

i32 -> 32-bit signed integer. It can hold values from -2,147,483,648 to 2,147,483,647.

u8 -> 8-bit unsigned integer. It can hold values from 0 to 255.

*/

fn integer_overflow() -> u8 {
    255
}
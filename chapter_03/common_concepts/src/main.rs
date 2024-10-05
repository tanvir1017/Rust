
// const value should not be change able and it always in upper case and demand a type while declaring variable
const ONE_HOUR: u32 = 60 * 60;
const THREE_HOURS_IN_SECONDS:u32 = ONE_HOUR * PI; // more convenient and understandable
const PI:u32 = 3;
fn main() {
    // by adding `mut` keywords we can make a variable mutable. Although variable by default immutable
    let mut age: i32 = 25;

    println!("Ich bin {age} jahre alt gewessen");

    
    age = 34;

    println!("Jetzt bin ich {age} jahre alt gewessen");


    
    println!("PI values {} {}", PI, THREE_HOURS_IN_SECONDS);
    shadowing();
}
 // shadowing

 fn shadowing(){

    // shadowing means that declaring variable more than one with the same name. Like we'll do below

    
    
    /* 
    ! How it work?
    
    The fir st variable called apples value will be stored in `10`, after that when program comes to second line 
    the program will calculate first the two of there values like `10(apples[1 line]) + 20` then it will store it into new variable called apples. YES IT'S NEW VARIABLE. 
    !! previous one will be removed after read by program. THIS IS HOW RUST MANAGE MEMORY EFFICIENTLY
    */
     
    
    let apples: u32 = 10;

    let apples: u32 = apples + 20;

    println!("apples value right now {apples}");

    // Let's us allow to bring another example from RUST BOOK to know better way of shadowing

    let x: i32=  5;

    let x: i32 = x + 1; // this var shadowed the line 47 variable x, and assign value 6 into new variable called x  


    // creating inner scope 
    {
        let x: i32 = x * 2; // This one is shadowing the variable x in line number 49. while it can't reach 47 as it shadowed before. So for now 12 value assign into variable x but in 54 line inner scope. While the inner scope is done with task it will removed from the memory.
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // after x from inner scope removed from memory this print able x will reach to the x variable fo 49 and print value 6. Although it can't access or reach to inner scope. 😊. This is shading in   RUST
 }
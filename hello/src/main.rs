mod random_info;
use random_info::*;

// it can be allow, warn, deny
#[allow(unused_variables)] // so that I dont have warnings about unused vars
#[allow(unused_assignments)] // so that I dont have warnings about unused assignments

// #[allow(non_snake_case)] // so that I dont have warnings if names are not in snake case

struct DoughsData {
    some_bool: bool,
    some_float: f64,
    some_int: i32,
    random: RandomInfo, // Struct importing from another file - Its an example of composition
}

fn main() {
    // Rust donot support inheritance but composition
    // println!("Hello, World!"); // next print
    // print!("asd asd");

    ///////////////////////////////////////// DATA TYPES
    ////////////////////// Boolean
    // // naming convention is snake case by default
    // // rust variables are by_default immutable
    // let some_data: bool = true;
    // // above code can also be written without type
    // // let some_data = true;

    // // for creating a mutable variable
    // let mut some_data_mut = true;
    // // by using mut, we can change its value
    // some_data_mut = false;

    ////////////////////// Integer
    // let some_data_int: i8 = 10; // i stands for integer and 8 stands for 8 bits of memory i.e; from -128 to 127
    // println!("Min i8 is {}", std::i8::MIN);
    // println!("Max i8 is {}", std::i8::MAX);
    // println!("Max i8 is {}", "asd");

    // let dougs_test = some_data_int + 120; // default initialization is i8 and 130 value greater than 127 - panic error occur
    // println!("{}", dougs_test);

    ////////////////////// unsigned integers
    // let some_data_u: u8 = 10; // u8 stands for unsigned 8 bits of int and ranges from 0 to 255
    // let some_otherdata_u = 255 + some_data_u; // ERROR!!! Too big
    // let some_otherdata_u1 = 1 - some_data_u; // ERROR!!! Too small

    // some_data_u = 50; // throw error, as 'some_data_u' is immutable

    // let some_isize: isize = 20; // depends on whether your computer is 32 bit or 64 bit
    // let some_usize: usize = 10; // depends on whether your computer is 32 bit or 64 bit

    ////////////////////// float
    // let some_data_float: f32 = 10.; // always put dot somewhere in your number

    ////////////////////// char
    // let some_data_char: char = 'a'; // values must be in single quotes - More than just ascii , can be chinese language or emojy

    ////////////////////// Strings
    //// double quotes represent string and single quote represent char
    //// default string type is String Slice
    // let example_str: &str = "example"; // '&str' is read as 'String slice' - its a reference to heap memory - immutable
    // let example_string: String = String::from("example"); // it is present in heap memory - mutable

    //// Convert a string slice to string
    // let string_from_str: String = example_str.to_string(); // conversion on string slice to string
    // let string_from_str2: String = "Some hardcoded string".to_string();

    // let string_from_hardcoded = String::from("Some Hardcoded"); // no need of assigning type
    // let string_from_str_var = String::from(example_str); // create string from string slice - no need of type

    //// Convert a string to string slice
    // // adding "&" symbol, is a cheap way to convert string into &str
    // let str_from_string: &str = &example_string; // create string slice from string

    //// Combine two strings
    // let test = "first" + "last"; // compile error - '+' symbol cannot be used in between two string slices
    // let combine_string_literals = ["first", "last"].concat();
    // let combine_with_format_macro = format!("{} {}", "first", "last");
    // println!("{}", combine_with_format_macro);

    // let string_plus_str = example_string + example_str;

    //// mutable strings
    // let mut mut_string = String::new(); // no need to add type "String".
    // mut_string.push_str(example_str); // push_str for strings slices
    // mut_string.push_str("Some hardcoded string");
    // mut_string.push('m'); // push for characters (char)

    //// addition (concatenation)
    // let a = String::from("a");
    // let b = String::from("b");
    // let c = String::from("c");
    // let combined = a + &b; // for adding strings 1st one should be of type "string" and others of type "&str"
    // let combined_multiple = c + &b + &mut_string; // for adding strings 1st one should be of type "string" and others of type "&str"
    // println!("{}", combined_multiple);

    //// substring
    // let str_from_substring: &str = &example_str[0..4]; // e.g. in math notation = [0,4), substring from index 0 to 4 excluding 4, 0 or 4 can be eleminated
    // let str_from_substring: &str = &example_str[0..=4]; // substring from index 0 to 4 including 4
    // let str_from_substring: &str = &example_str[0..8]; // Error - because elements are till index 6 and we are calling index 7 too
    // println!("{}", str_from_substring);

    // //
    // let char_by_index = &example_str.chars().nth(1);
    // match char_by_index {
    //     Some(c) => println!("Found a char {}", c),
    //     None => {}
    // }

    // if let Some(c) = example_str.chars().nth(2) {
    //     println!("Found a char {}", c)
    // }

    ///////////////////////////////////////// Functions and Procedures
    //// Both are similar but functions return values and procedures don't
    ////// functions
    // let returned_data = some_function(10.2, 5);
    // println!("returned data is {}", returned_data);

    ////// procedures
    //// procedure with numbers as params
    // some_procedure(12.2, 5);

    //// procedure with string slice as params
    // some_str_procedure("test");

    // let string_slice_var: &str = "testing";
    // some_str_procedure(string_slice_var);

    // let string_var: String = String::from("I am a real string");
    // some_str_procedure(&string_var); // cannot pass string as param because param type is &str, thats why adding & symbol in param

    //// procedure with string as params
    // let string_var1: String = String::from("I am a real string1");
    // some_string_procedure(string_var1);
    // some_string_procedure(string_var1); // error - cannot use this multiple time. If we wanna use it multiple times, use following code.

    //
    // let string_var2: String = String::from("I am a real string2");
    // some_string_procedure_multiple(&string_var2);
    // some_string_procedure_multiple(&string_var2);

    ///////////////////////////////////////// Conditional Statements
    //// if conditions
    // let some_bool = true;
    // if some_bool == true {
    //     println!("Hit If branch")
    // }
    // if some_bool {
    //     println!("Hit If branch")
    // }
    // if !(some_bool == true) {
    //     println!("Hit If branch-")
    // }
    // if !some_bool {
    //     println!("Hit If branch-")
    // }

    //// if - else if - else conditions
    // let some_bool1 = true;
    // let some_int = 30;
    // let some_int1 = 50;
    // if some_bool1 == false || (some_int > 50 && some_int1 == 200) {
    //     println!("Hit If branch")
    // } else if some_int == 30 {
    //     println!("Hit Else If branch")
    // } else {
    //     println!("Hit else branch")
    // }

    //// inline if else conditions
    // let some_int = 30;
    // let some_int1 = 50;
    // let var_from_inline = if some_int == 9 { 300 } else { 400 };
    // let var_from_inline1 = if some_int == 9 {
    //     300 // returning 300
    // } else if some_int1 == -3 {
    //     println!("Testing...");
    //     0 // returning 0
    // } else {
    //     400 // returning 400
    // };

    //// match statement (like switch case)
    // let some_bool2 = true;
    // match some_bool2 {
    //     true => {
    //         println!("Hit true branch");
    //     }
    //     false => {
    //         println!("Hit false branch");
    //     }
    // }

    // let some_int2 = 10;
    // match some_int2 {
    //     0 => println!("Hit 0 branch"),
    //     1..=100 => println!("Between 1 and 100 branch"),
    //     101..=200 => {
    //         println!("Between 101 and 200 branch");
    //         println!("printing some more content");
    //     }
    //     _ => println!("Else Branch"),
    // }

    //// inline match conditions
    // let some_bool3 = true;
    // let some_int3 = 10;
    // let var_from_match = match some_bool3 {
    //     true => 10,
    //     false => 20,
    // };

    // let var_from_match1 = match some_int3 {
    //     0 => 10,
    //     1 | 2 => 20,
    //     _ => 30,
    // };
    ///////////////////////////////////////// Tuples
    ////Group data together
    // let some_tuple = (2, 3.4, "a", "b".to_string(), 'c', (1.1, 2));
    // println!("My data is {} {}", some_tuple.0, some_tuple.1);
    // println!("My full tuple is {:?}", some_tuple);
    // println!(
    //     "My nested tuple is {} {}",
    //     (some_tuple.5).0,
    //     some_tuple.5 .1
    // );
    //// Tuple with function
    // let some_color = get_some_rgb(); // returning a tuple
    // println!("Green is {}", some_color.1);

    // let (my_red, my_green, my_blue) = some_color;
    // println!("r {} g {} b {}", my_red, my_green, my_blue);

    //rgba or argb
    // let some_other_color: (u8, u8, u8, u8) = (0, 100, 150, 255); // rgba

    //// Empty tuple
    // a procedure also returns empty tuple

    // let empty_tuple = ();

    // match some_color.2 {
    //     0..=200 => println!("Blah!!!"),
    //     _ => (), // returning empty typle
    // }

    ///////////////////////////////////////// Structs (somehow matches with object)
    // let dougs_var = DoughsData {
    //     some_bool: true,
    //     some_float: 10.2,
    //     some_int: 5,
    //     random: RandomInfo {
    //         some_bool: true,
    //         some_int: 100,
    //     },
    // };
    // dougs_var.some_int = 4; // error because variable is immutable

    let mut dougs_var_mut = DoughsData {
        some_bool: true,
        some_float: 10.2,
        some_int: 5,
        // random: RandomInfo {
        //     some_bool: false,
        //     some_int: 50,
        // },
        random: RandomInfo::new(true), // calling method of RandomInfo - double colon is used to access function because we are not using &self in params
    };
    dougs_var_mut.some_int = 4; // variable is immutable

    let dougs_var_2 = DoughsData {
        some_int: 200,
        ..dougs_var_mut
    };

    //// Struct importing from another file
    let random_info_var = RandomInfo {
        some_bool: true,
        some_int: 10,
    };

    let is_this_smaller = random_info_var.is_smaller(9); // getting is_smaller function with dot operator because we are using &self in params
}

//
#[allow(dead_code)] // so that I dont have warnings about unused function

// function - f32 after '->' is return type
// put underscore before param name, if we are not using it in the function block
fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("I'm in some function");

    if param_a < 100. {
        let return_var = 10.1 * param_a + param_b as f32; // converting param_b into f32 because only same types can be added
        return return_var;
    } else {
        -1.
    }
    //
    //// return values
    // 1.2 // No semicolon means this is what's retured by the function
    //     // return 1.2; // we can also return this way
    //     // 10 as f32 // we can return integer value into f32 this way
    //     // 10f32 // we can return integer value into f32 this way also
}

//
#[allow(dead_code)] // so that I dont have warnings about unused function

// procedure - no return type
// put underscore before param name, if we are not using it in the function block
fn some_procedure(param_a: f32, param_b: i128) {
    println!("I'm in some procedure with a {}, b {}", param_a, param_b);
}

// fn some_str_procedure(param: &str) {
//     println!("I'm in some_str_procedure with param '{}'", param);
// }

// fn some_string_procedure(param: String) {
//     println!("I'm in some_string_procedure with param '{}'", param);
// }

// fn some_string_procedure_multiple(param: &String) {
//     println!(
//         "I'm in some_string_procedure_multiple with param '{}'",
//         param
//     );
// }

fn get_some_rgb() -> (u8, u8, u8) {
    // some logic...
    (200, 100, 50) // returning value is a tuple
}

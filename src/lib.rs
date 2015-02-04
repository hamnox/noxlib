mod algs;

//use std::io; //import io::functions()
//use std::cmp::Ordering;
//use std::{io, other_modules} // cool import style
//
//struct Thingy {
//     name1: type,
//     name2: type,
//     ...
//     // structs are like named tuples.. or ordered dictionaries
//          // faster than ordered dictionaries though
//     // can use Thingy.name1 to get a value
//}
//
//struct TupleStruct(type, type, ..);
//     // these are super useful for specify Units of a value
//
//enum Enumeration {
//     PossibleValue1,
//     PossibleValue2,
//     // Can set it up like a tuple struct with
//     // PossibleValue(type1, type2, ..)
//}
//
//fn main() {
//     let variable: type = value; //more specific declare and bind
//     let mut actual_variable = value; // reassignable
//     let array = [initial_values; size] // quick array setting
//     array[index_from_zero] //grab array value
//     let mut v: Vec<i32> = vec![1,2,3];
//          v.push(add_to_vector);
//          v.slice(start,end); //view into the vector.. or String
//
//     if expression { return } else { return };
//          //ifs are expressions, though they can contain statements. wow.
//
//     let tuple: (type, type, ..) = (val, val, ..);
//          // two tuples won't compile == comparisons
//          // if not the same types
//
//     let struct_example = Thingy { name1: value1, name2: value2 .. };
//     let tuplestruct_example = TupleStruct(value, value, ..);
//
//     //using an enum
//     let v: Enumeration = Enumeration::PossibleValue1(value);
//     // let v = Enumeration::PossibleValue1;
//     // let v = Enumeration::PossibleValue(123);
//     match v {
//          val1 => do something,
//          val2 => do something,
//          ..
//     } //match is as expressiony as if-else!
//
//     //Rust's for loop is a for-in loop
//     for var in array.iter() {
//          do stuff
//     }
//     for var in range(0,10) { //10 non-inclusive
//          do stuff
//     }
//
//     
//     let mut done = false;
//     while !done {
//          do stuff
//          if condition { done = true; }
//     }
//     // no need to while true, because Rust has:
//
//     loop{
//          if condition { break; }
//          else { continue; }
//     }
//
//     let input = io::stdin()
//                    .read_line()
//                    .ok()
//                    .expect("Failed to read line");
//     //voodoo magic here. also whitespace magic.
//
//}
//
//fn function(argument: type) -> returntype {
//// do not end return expressions in ;
//     return expression //can return stuff early!
//}
//use std::cmp::Ordering;
//fn cmp(a: u32, b: u32) -> Ordering {
//     if a < b { Ordering::Less }
//     else if a > b { Ordering::Greater }
//     else { Ordering::Equal }
//}
//
//#[cfg(test)] disable a module unless in test mode
//
//#[test]
//fn function() {
//     assert!(true);
//     assert_eq!("this","this");
//}
//[test]
//fn it_works() {
//}

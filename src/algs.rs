pub mod sorts{
    //use core::num;
    //pub fn quicksort<T>(&[T])
    //    where T: num::NumCast { 
    //        //for now only accepting numbers
    //        num::cast(
    //    }
    // TODO: impl for all comparables 
    pub fn quicksort(array: &mut [i64]) {
        let length = array.len();

        if length < 2 { }
        else {
            swap(array, (length - 1)/2, length - 1);
            let sort_char = length - 1;

            // go through the index. If a number < index, swap
            // it to the place after the last swapped index.
            let mut store_index = 0us;
            for index in 0..(length - 1) {
                if array[index] < array[sort_char] {
                    swap(array, index, store_index);
                    store_index += 1; 
                }
            }
            // put the pivoted character right after the last swapped index
            swap(array, sort_char, store_index);
            if store_index > 1 {
                quicksort(&mut array[0..(store_index - 1)]);
            }
            if store_index < length - 2 {
                quicksort(&mut array[(store_index + 1)..length]);
            }
        }
    }

    fn swap(array: &mut [i64], i1: usize, i2: usize) {
        let hold = array[i1];
        array[i1] = array[i2];
        array[i2] = hold;
    }

}

#[test]
fn len_check() {
    let testarray = &mut [3i64, 3, 3, 2, 2, 0, 1, 45, -1];
    assert_eq!(testarray.len(), 9);
}
#[test]
fn match_answer() {
    let testarray = &mut [2i64,1];
    sorts::quicksort(testarray);
    assert_eq!([1i64,2], *testarray);
}
#[test]
fn quicktest() {
    let testarray = &mut [3i64, 1, 2, 0];
    sorts::quicksort(testarray);
    assert_eq!([0i64,1,2,3], *testarray);
}
#[test]
fn multiplestest() {
    let testarray = &mut [2i64, 2, 0, 1, 4,
                             4, 5, 2, 3, 0];
    sorts::quicksort(testarray);
    assert_eq!(&mut [0i64, 0, 1, 2, 2,
                        2, 3, 4, 4, 5], testarray);
}

#[test]
fn ohyeahwhatifidothis() {
    let testarray = &mut [0i64, 1, 2, 3, 4,
                             5, 6, 7, 8, 9];
    sorts::quicksort(testarray);
    assert_eq!(&mut [0i64, 1, 2, 3, 4,
                        5, 6, 7, 8, 9], testarray);

}
// fn blanktest() {
//     let testarray = [3,5,2,7];
//     assert_eq!(testarray, sorts::quicksort(&testarray));
// }

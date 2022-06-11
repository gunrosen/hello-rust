/*
Check sub array
 */
pub(crate) fn example_1() {
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];
    let mut result: bool = false;

    let origin_length: usize = org_arr.len();
    let sub_length: usize = sub_arr.len();

    if origin_length < sub_length {
        result = false
    }

    let mut i = 0;
    while i < origin_length - sub_length {
        let candidate = &org_arr[i..i+sub_length];
        println!("{:?}", candidate);
        if candidate == sub_arr {
            result = true;
            break;
        }
        i = i + 1;
    }

    if result {
        println!("True");
    } else {
        println!("False");
    }

}
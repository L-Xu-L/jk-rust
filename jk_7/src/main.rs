// emapmle 1

// fn main() {
//     let data = vec![10, 42, 9, 8];
//     let v = 42;
//     if let Some(pos) = find_pos(data, v)    {
//         println!("Found {} at {}", v, pos);
//     }
// }


// fn find_pos(data: Vec<u32>, v: u32) -> Option<usize>{
//     for (pos, item) in data.iter().enumerate() {
//         if *item == v {
//             return Some(pos);
//         }
//     }

//     None
// }




// example 2

// fn main() {
//     let data = vec![1, 2, 3, 4];
//     let data1 = data;
//     println!("sum of data1: {}", sum(data1));
//     println!("data1: {:?}", data1);
//     println!("sum of data: {}", sum(data))
// }

// fn sum(data: Vec<u32>) -> u32 {
//     data.iter().fold(0, |acc, v| acc + v)
// }





// example 3
fn is_copy<T: Copy>() {}

fn types_impl_copy_trait() {
    is_copy::<bool>();
    is_copy::<char>();

    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    is_copy::<fn()>();

    is_copy::<*const String>();
    is_copy::<*mut String>();

    is_copy::<&[Vec<u8>]>();
    is_copy::<&String>();

    is_copy::<[u8; 4]>();
    is_copy::<(&str, &str)>();
}

fn types_not_impl_copy_trait() {
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();
    is_copy::<String>();

    is_copy::<&mut String>();

    is_copy::<[Vec<u8>; 4]>();
    is_copy::<(String, u32)>();
}

fn main() {
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}
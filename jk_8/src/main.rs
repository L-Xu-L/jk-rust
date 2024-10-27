// example 1

// fn main() {
//     let data = vec![1, 2, 3, 4, 5];
//     let data1 = &data;

//     // 值的地址是什么？引用的地址又是什么？
//     println!("addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
//             &data, data1, &&data, &data1
//     );
//     println!("sum of data1: {}", sum(data1));

//     // 堆上数据的地址是什么？
//     println!(
//         "addr of items: [{:p}， {:p}， {:p}，{:p}]",
//         &data[0], &data[1], &data[2], &data[3]
//     );
// }

// fn sum(data: &Vec<u32>) -> u32{
//     // 值得地址会改变么？引用的地址会改变么？
//     println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
//     data.iter().fold(0, |acc, val| acc + val)
// }


// example 2

// fn main() {
//     let r = local_ref();
//     println!("r: {:p}", r);
// }

// fn local_ref<'a>() -> &'a i32 {
//     let a = 42;
//     &a
// }



// example 3

// fn main() {
//     let mut data: Vec<&u32> = Vec::new();
//     let v = 41;
//     data.push(&v);
//     println!("data: {:?}", data);
// }



// example 4

// fn main() {
//     let mut data: Vec<&u32> = Vec::new();
//     push_local_ref(&mut data);
//     println!("data: {:?}", data);
// }

// fn push_local_ref(data: &mut Vec<&u32>) {
//     let v = 42;
//     data.push(&v);
// }




// example 5

// fn main() {
//     let mut data = vec![1, 2, 3];

//     for item in data.iter_mut() {
//         data.push(*item + 1);
//     }
// }


// example 6

fn main() {
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data[0]);

    for i in 4..100 {
        data.push(i);
    }

    println!("data[0]: {:p}", &data[0]);
    println!("boxed {:p}", &data1);
}
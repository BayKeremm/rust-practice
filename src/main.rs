mod structs;
mod algos;

use algos::mergesort::merge_sort;
use structs::node::Node;


fn print_tree(input: Vec<i32>){
    // first find the levels needed
    let n = input.len() as u32;
    let l = (n as f64).log2();
    let full_levels = l as u32;
    //let leafs = n - (2 as u32).pow(full_levels) + 1;
    //println!("full levels: {:?}", full_levels);
    //println!("leafs: {:?}", leafs);
    let space = " ";
    print!("{}", space.repeat(((2 as u32).pow(full_levels+1)-1) as usize));
    if let Some(root) = input.get(0){
        println!(" {:?}", root);
    }

    for i in 1 .. full_levels {
        let start_idx = (2 as u32).pow(i) - 1;
        let end_idx =  start_idx * 2;
        // numbers
        print!("{}", space.repeat(((2 as u32).pow(full_levels-i+1)-1)  as usize));
        for j in start_idx .. end_idx+1 {
            if let Some(value) = input.get(j as usize){
                print!("{:?}",value);
            }
            print!("{}", space.repeat((((2 as u32).pow(full_levels-i+1)*2)-2)  as usize));
        }
        println!();

    }

    // leafs
    let start_idx = (2 as u32).pow(full_levels) - 1;
    print!("{}", space);
    for i in start_idx .. n {
            if let Some(val) = input.get(i as usize){
                print!("{:?}",val);
            }
            print!("{}", space.repeat(3));
    }
    println!();

}

fn main() {
    let mut small_data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut medium_data = vec![
        45, 23, 67, 12, 89, 90, 56, 34, 23, 12, 78, 56, 45, 23, 12, 89, 67, 34, 23, 45, 78, 12, 90, 56, 
        34, 23, 12, 78, 56, 45, 23, 12, 89, 67, 34, 23, 45, 78, 12, 90, 56, 34, 23, 12, 78, 56, 45, 23
    ];
    let mut large_data = vec![
        45, 23, 67, 12, 89, 90, 56, 34, 23, 12, 78, 56, 45, 23, 12, 89, 67, 34, 23, 45, 78, 12, 90, 56, 
        34, 23, 12, 78, 56, 45, 23, 12, 89, 67, 34, 23, 45, 78, 12, 90, 56, 34, 23, 12, 78, 56, 45, 23, 
        12, 89, 67, 34, 23, 45, 78, 12, 90, 56, 34, 23, 12, 78, 56, 45, 23, 12, 89, 67, 34, 23, 45, 78, 
        12, 90, 56, 34, 23, 12, 78, 56, 45, 23, 12, 89, 67, 34, 23, 45, 78, 12, 90, 56, 34, 23, 12, 78, 
        56, 45, 23, 12, 89, 67, 34, 23, 45, 78, 12, 90, 56, 34, 23, 12, 78, 56, 45, 23, 12, 89, 67, 34, 
        23, 45, 78, 12, 90, 56, 34, 23, 12, 78, 56, 45, 23, 12, 89, 67, 34, 23, 45, 78, 12, 90, 56, 34
    ];


    println!("{:?}", small_data);
    //let res = bubble_sort(&mut large_data);
    let res = merge_sort(&mut small_data);
    println!("{:?}", res);
    //println!("{:?}", small_data);
    //heap_sort(&mut small_data);
    //println!("{:?}", small_data);

    let left_node = Node::new('a', 0.75, None, None);
    let right_node = Node::new('b', 0.10, None, None);
    let parent_node = Node::new('p', 0.15, Some(Box::new(left_node)), Some(Box::new(right_node)));

    println!("{:?}", parent_node);

}

fn merge(left : &mut Vec<i32>, right : &mut Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len()+right.len());

    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    let mut left_item = left_iter.next();
    let mut right_item = right_iter.next();

    while left_item.is_some() && right_item.is_some() {
        if left_item <= right_item {
            result.push(*left_item.unwrap());
            left_item = left_iter.next();
        }else{
            result.push(*right_item.unwrap());
            right_item = right_iter.next();
        }
    }
    while let Some(item) = left_item {
        result.push(*item);
        left_item = left_iter.next();
    }
    
    while let Some(item) = right_item {
        result.push(*item);
        right_item = right_iter.next();
    }
    return result;
}

pub fn merge_sort(input : &mut Vec<i32>) -> Vec<i32>{
    let l = input.len();
    if l <= 1 {
        return input.clone();
    }

    let middle = l / 2 ; 
    let mut left_half = merge_sort(&mut input[0..middle].to_vec());
    let mut right_half = merge_sort(&mut input[middle..].to_vec());

    merge(&mut left_half, &mut right_half)
}
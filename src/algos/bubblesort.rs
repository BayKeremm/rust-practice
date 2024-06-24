pub fn bubble_sort(input : &mut Vec<i32>) -> &mut Vec<i32>{
    let l = input.len();
    let mut done = false;
    let mut i = 0;
    while !done {
        for j in i..l {
            if input.get(i) > input.get(j) {
                input.swap(i, j)
            }
        }
        if i == l{
            done = true;
        }else{
            i+=1;
        }

    }
    return input;
}
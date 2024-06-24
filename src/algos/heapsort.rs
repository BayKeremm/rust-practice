fn heapify(input : &mut Vec<i32>, n:usize, i:usize){
    let mut largest = i;
    let left = 2*i + 1;
    let right = 2*i+2;

    if left < n && input[left] > input[largest]{
        largest = left;
    }

    if right < n && input[right] > input[largest]{
        largest = right;
    }

    if largest != i {
        input.swap(i, largest);
        heapify(input, n, largest)
    }
}

pub fn heap_sort(input : &mut Vec<i32>) {
    let n = input.len();

    for i in (0..n/2).rev(){
        heapify(input, n, i);
    }

    for i in (1..n).rev(){
        input.swap(0, i);
        heapify(input, i, 0);
    }

}
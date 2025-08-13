fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    qsort(arr, 0, arr.len() - 1);
}

fn qsort(arr: &mut [i32], first: usize, last: usize) {
    let split = partition(arr, first, last);
    
    if first < split {
        qsort(arr, first, split);
    }

    if last > split + 1 {
        qsort(arr, split + 1, last);
    }
}

fn partition(arr: &mut [i32], first: usize, last: usize) -> usize {
    let pivot = arr[(first + last) / 2];
    let mut i = first.wrapping_sub(1); //handle underflow
    let mut j = last + 1;

    loop {
        loop {
            i = i.wrapping_add(1);
            if arr[i] >= pivot {
                break;
            }
        }

        loop {
            j = j.wrapping_sub(1);
            if arr[j] <= pivot {
                break;
            }
        }

        if i < j {
            arr.swap(i, j);
        } else {
            return j;
        }
    }
}

fn main() {
    let mut arr1 = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", arr1);
    quicksort(&mut arr1);
    println!("Sorted array: {:?}", arr1);
}

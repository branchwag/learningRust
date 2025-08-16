fn partition(arr: &mut [u8], first: usize, last:usize) -> usize {
    let pivot = arr[(first + last) / 2];
    let mut i = first;
    let mut j = last;

    i = i.wrapping_sub(1);
    j = j.wrapping_add(1);

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

fn q_sort(arr: &mut [u8], first: usize, last: usize) {
    if first >= last {
        return;
    }

    let split = partition(arr, first, last);

    if first < split {
        q_sort(arr, first, split);
    }

    if last > split + 1 {
        q_sort(arr, split + 1, last);
    }
}

fn quicksort(arr: &mut [u8]) {
    if arr.len() <= 1 {
        return;
    }
    q_sort(arr, 0, arr.len() - 1);
}

fn main() {

    let mut arr: [u8; 5] = [5, 8, 2, 6, 1];

    println!("Get ready to quicksort...");

    println!("Here is the original array:");
    println!("{:?}", arr);
    quicksort(&mut arr);
    println!("Here is our array after sorting:");
    println!("{:?}", arr);
}

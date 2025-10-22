/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
fn sort_helper<T: PartialOrd + Clone>(arr: &mut [T], start: usize, end: usize) {
    if start + 1 >= end {
        return;
    }
    let pivot = arr[start].clone();
    let mut left = start + 1;
    let mut right = end - 1;
    
    while left <= right {
        while left <= right && arr[left] <= pivot {
            left += 1;
        }
        while left <= right && arr[right] > pivot {
            right -= 1;
        }
        if left < right {
            arr.swap(left, right);
        }
    }

    arr.swap(start, right);

    sort_helper(arr, start, right);
    sort_helper(arr, right + 1, end);
}

fn sort<T: PartialOrd + Clone>(arr: &mut [T]){
    sort_helper(arr, 0, arr.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        let mut swapped = false;

        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                // Swap arr[j] and arr[j+1]
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;

                swapped = true;
            }
        }

        // 如果一轮遍历没有发生交换，说明数组已经有序，可以提前结束排序
        if !swapped {
            break;
        }
    }
}

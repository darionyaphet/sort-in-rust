// Perform bubble sort on the given mutable integer slice.
pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();  // Get the length of the array

    // Iterate through the array
    for i in 0..n {
        // Flag to track if any swaps were made in this pass
        let mut swapped = false;

        // Iterate through the unsorted part of the array
        for j in 0..n - i - 1 {
            // Compare adjacent elements and swap them if they are in the wrong order
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;

                // Set swapped to true since a swap occurred
                swapped = true;
            }
        }

        // If no elements were swapped in this pass, the array is sorted, so break
        if !swapped {
            break;
        }
    }
}

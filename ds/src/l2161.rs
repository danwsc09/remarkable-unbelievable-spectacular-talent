pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let mut res: Vec<i32> = nums.clone();
    let mut less_count = 0;
    let mut more_count = 0;
    for num in &nums {
        if *num < pivot {
            less_count += 1;
        } else if *num > pivot {
            more_count += 1;
        }
    }

    let mut low = 0;
    let mut same = less_count;
    let mut high = nums.len() - more_count;
    for i in 0..nums.len() {
        if nums[i] < pivot {
            res[low] = nums[i];
            low += 1;
        } else if nums[i] > pivot {
            res[high] = nums[i];
            high += 1;
        } else {
            res[same] = pivot;
            same += 1;
        }
    }
    return res
}
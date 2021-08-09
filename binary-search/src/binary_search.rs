pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mid = nums.len() / 2;
    let left = &nums[0..mid];
    let right = &nums[mid..];


    if mid == 0 && nums[mid] != target {
        return -1;
    }

    return

        if nums[mid] == target {
            return nums[mid];
        } else if nums[mid] > target {
            binary_search(left.to_vec(), target)
        } else {
            binary_search(right.to_vec(), target)
        };
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_binary_search_case_found() {
        assert_eq!(binary_search(vec! {2, 5, 7, 30, 67, 98, 100}, 30), 30);
    }

    #[test]
    fn test_binary_search_case_not_found() {
        assert_eq!(binary_search(vec! {2, 5, 7, 30, 67, 98, 100}, 999), -1);
    }
}
fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut left_count = 0;
    let mut right_count = 0;
    let mut temp = Vec::new();

    while left_count < left.len() && right_count < right.len() {
        if left[left_count] >= right[right_count] {
            temp.push(right[right_count]);
            right_count += 1;
            if right_count == right.len() {
                break;
            }
        }

        if left[left_count] <= right[right_count] {
            temp.push(left[left_count]);
            left_count += 1;
        }
    }

    if left_count == left.len() {
        for x in right_count..right.len() {
            temp.push(right[x])
        }
    }

    if right_count == right.len() {
        for x in left_count..left.len() {
            temp.push(left[x])
        }
    }

    return temp;
}

fn merge_sort(vec: &Vec<i32>) -> Vec<i32> {
    match vec.len() {
        i if i < 1 => vec.to_vec(),
        i if i > 1 => {
            let size = vec.len() / 2;
            let left = merge_sort(&vec[0..size].to_vec());
            let right = merge_sort(&vec[size..].to_vec());
            let merged = merge(&left, &right);
            merged
        }
        _ => vec.to_vec(),
    }
}

fn main() {
    println!("Sort numbers ascending");
    let mut numbers = [4, 65, 2, -31, 0, 99, 2, 83, 782].to_vec();
    println!("Before: {:?}", numbers);
    let test = merge_sort(&mut numbers);
    assert!(test, [-31, 0, 2, 2, 4, 83, 99, 782].to_vec());
    println!("After:  {:?}\n", test);
}
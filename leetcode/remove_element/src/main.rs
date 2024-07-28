pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }

    let mut j = len - 1;
    let mut i = 0;
    while i < j {
        if nums[i] == val {
            nums[i] = nums[j];
            nums[j] = val;
            j -= 1;
        } else {
            i += 1;
        }
    }

    println!("i = {}", i);
    println!("j = {}", j);
    if i == 0 {
        *nums = vec![];
        return 0;
    }

    // *nums = nums[..i].to_vec();
    nums.truncate(i);

    (i + 1) as i32
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let mut val = 3;

    nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    val = 2;

    nums = vec![4, 5];
    val = 5;

    nums = vec![3];
    val = 3;

    nums = vec![3, 3];
    val = 3;
    let len = remove_element(&mut nums, val);

    println!("{:?}", nums);
    println!("{:?}", len);
    println!("Hello, world!");
}

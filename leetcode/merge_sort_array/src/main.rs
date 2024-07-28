pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &Vec<i32>, n: i32) {
    let mut m = m as usize; // 已初始化的 nums1 的长度
    let mut n = n as usize; // nums2 的长度
    let mut tail = m + n - 1; // 合并后数组的最后一个索引

    // 从后往前合并，以防止覆盖 nums1 中的未处理元素
    while m > 0 && n > 0 {
        if nums1[m - 1] > nums2[n - 1] {
            nums1[tail] = nums1[m - 1];
            m -= 1;
        } else {
            nums1[tail] = nums2[n - 1];
            n -= 1;
        }
        tail -= 1;
    }

    // 如果 nums2 中还有剩余元素，复制到 nums1 的起始位置
    while n > 0 {
        nums1[tail] = nums2[n - 1];
        tail -= 1;
        n -= 1;
    }
}




pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {

    let mm = m  as usize;
    let mut n = n as usize;
    let mut m = m as usize;
    print!("nums2[n]:  {} ", nums2[n -1]);

    for i in (0 .. mm) {
        println!("nums1 is {}", nums1[i as usize]);
        println!("n = {}, i = {}", n, i);
        

        if nums2[n - 1] >= nums1[m - 1] {
            nums1[n + m - 1] = nums2[n -1];
            n = n-1;
        } else {
            nums1[n + m - 1] = nums1[m - 1];
            m = m-1;
        }
        println!("nums2 is {}", nums1[i as usize]);
        println!("n = {}, i = {}", n, i);

        if n - 1  < 0 {
            break;
        }
    }


    for i in (0 .. n) {
        nums1[i] = nums2[i];
    }

}


fn main() {
    let mut arr1 = vec![1,2,7,0,0,0,0];
    let mut arr2 = vec![1,3,4,9];
    let m = arr1.len() as i32;
    let n = arr2.len() as i32;

    merge(&mut arr1, m - n , &mut arr2, n);
    println!("arr1 is {:?}", arr1);

    println!("Hello, world!");
}

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut no:i32=0;
        for i in nums{
            no=no^i;
            println!("{no}")
        }
        return no
    }
}

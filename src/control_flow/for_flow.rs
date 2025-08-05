

#[cfg(test)]
mod exercies {
    #[test]
    pub fn for_vev() {
        let nums = [1,2,3,4,5];
        for (i,&num) in nums.iter().enumerate() {
            print!("index:{i}, value:{num}\n");
        }
    }
}
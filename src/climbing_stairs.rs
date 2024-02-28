use std::collections::HashMap;

fn stairs(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(&result) = cache.get(&n) {
        return result;
    }
    let result = stairs(n - 1, cache) + stairs(n - 2, cache);

    cache.insert(n, result);
    result
}

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut fib_cache: HashMap<i32, i32> = HashMap::new();
        fib_cache.insert(0, 1);
        fib_cache.insert(1, 1);
        stairs(n, &mut fib_cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stairs() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}

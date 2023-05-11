fn subconjunto_suma(nums: &[i32], target: i32) -> Option<Vec<i32>> {
    let n = nums.len();
    let target = target as usize;

    let mut dp = vec![vec![false; target + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..=n {
        for j in (1..=target).rev() {
            let num = nums[i - 1] as usize;

            if j >= num {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - num];
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    if dp[n][target] {
        let mut subset = Vec::new();
        let mut i = n;
        let mut j = target;

        while i > 0 && j > 0 {
            if dp[i - 1][j] {
                i -= 1;
            } else {
                subset.push(nums[i - 1]);
                j -= nums[i - 1] as usize;
                i -= 1;
            }
        }

        Some(subset)
    } else {
        None
    }
}

fn main() {
    let nums = vec![3, 1, 5, 9, 12];
    let target = 8;

    if let Some(subset) = subconjunto_suma(&nums, target) {
        println!("Subconjunto que suma {}: {:?}", target, subset);
    } else {
        println!("No existe un subconjunto que suma {}", target);
    }
}

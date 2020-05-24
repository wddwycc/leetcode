pub fn rob(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }

    let mut max_profit_on_rob_today = vec![];
    let mut max_profit_on_no_rob_roday = vec![];
    for i in 0..len {
        match i {
            0 => {
                max_profit_on_rob_today.push(nums[0]);
                max_profit_on_no_rob_roday.push(0);
            }
            1 => {
                max_profit_on_rob_today.push(nums[1]);
                max_profit_on_no_rob_roday.push(nums[0]);
            }
            _ => {
                max_profit_on_rob_today.push(max_profit_on_no_rob_roday[i - 1] + nums[i]);
                max_profit_on_no_rob_roday.push(std::cmp::max(
                    max_profit_on_rob_today[i - 1],
                    max_profit_on_no_rob_roday[i - 1],
                ));
            }
        }
    }
    std::cmp::max(
        *max_profit_on_rob_today.last().unwrap(),
        *max_profit_on_no_rob_roday.last().unwrap(),
    )
}

fn main() {
    rob(vec![1, 2, 3, 1]);
}

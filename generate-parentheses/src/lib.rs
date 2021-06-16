pub struct Solution;
impl Solution {
    pub fn generate_parenthesis(mut n: i32) -> Vec<String> {
        let mut ans = vec![];
        Self::dfs(&mut "".to_owned(), &mut 0, &mut n, &mut ans);
        ans
    }

    fn dfs(str_acc: &mut String, l_acc: &mut i32, l_remaining: &mut i32, ans: &mut Vec<String>) {
        if *l_acc == 0 && *l_remaining == 0 {
            ans.push(str_acc.clone());
            return;
        }
        if *l_remaining > 0 {
            str_acc.push('(');
            *l_remaining -= 1;
            *l_acc += 1;
            Self::dfs(str_acc, l_acc, l_remaining, ans);
            str_acc.pop();
            *l_remaining += 1;
            *l_acc -= 1;
        }
        if *l_acc > 0 {
            str_acc.push(')');
            *l_acc -= 1;
            Self::dfs(str_acc, l_acc, l_remaining, ans);
            str_acc.pop();
            *l_acc += 1;
        }
    }
}

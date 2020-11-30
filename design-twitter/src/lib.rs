use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(PartialEq, Eq, Debug)]
struct Tweet {
    id: i32,
    ord: usize,
}
impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ord.cmp(&other.ord)
    }
}

#[derive(Default)]
pub struct Twitter {
    follows: HashMap<i32, HashSet<i32>>,
    user_tweets: HashMap<i32, Vec<Tweet>>,
    ord: usize,
}

impl Twitter {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /** Compose a new tweet. */
    pub fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.user_tweets
            .entry(user_id)
            .or_insert(vec![])
            .push(Tweet {
                id: tweet_id,
                ord: self.ord,
            });
        self.ord += 1;
    }

    /** Retrieve the 10 most recent tweet ids in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user herself. Tweets must be ordered from most recent to least recent. */
    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut follows = match self.follows.get(&user_id) {
            Some(a) => a.iter().collect::<Vec<&i32>>(),
            None => vec![],
        };
        follows.push(&user_id);
        let mut heap = BinaryHeap::new();
        follows
            .into_iter()
            .filter_map(|a| self.user_tweets.get(a))
            // TODO: take left 10
            .flat_map(|a| a)
            .for_each(|t| heap.push(t));

        let mut res = vec![];
        for _ in 0..10 {
            if let Some(t) = heap.pop() {
                res.push(t.id)
            } else {
                break;
            }
        }
        res
    }

    /** Follower follows a followee. If the operation is invalid, it should be a no-op. */
    pub fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.follows
            .entry(follower_id)
            .or_insert(HashSet::new())
            .insert(followee_id);
    }

    /** Follower unfollows a followee. If the operation is invalid, it should be a no-op. */
    pub fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id == followee_id {
            return;
        }
        self.follows
            .entry(follower_id)
            .or_insert(HashSet::new())
            .remove(&followee_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
        twitter.post_tweet(1, 3);
        assert_eq!(twitter.get_news_feed(1), vec![3, 5]);
    }
}

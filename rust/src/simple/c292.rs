struct Solution;

impl Solution {
  /// 你和你的朋友，两个人一起玩 Nim 游戏：
  ///
  /// 桌子上有一堆石头。
  ///
  /// 你们轮流进行自己的回合，你作为先手。
  ///
  /// 每一回合，轮到的人拿掉 1 - 3 块石头。
  ///
  /// 拿掉最后一块石头的人就是获胜者。
  ///
  /// 假设你们每一步都是最优解。请编写一个函数，来判断你是否可以在给定石头数量为 n 的情况下赢得游戏。如果可以赢，返回 true；否则，返回 false 。
  ///
  /// 提示：
  /// ```
  ///   1 <= n <= 2^31 - 1
  /// ```
  pub fn can_win_nim(n: i32) -> bool {
    // 巴什博弈：只有一堆n个物品，两个人轮流从这堆物品中取物，规定每次至少取一个，最多取m个。最后取光者得胜。
    // n%（m+1）== 0 则后手胜利
    n & 3 != 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn can_win_nim() {
    assert_eq!(true, Solution::can_win_nim(5));
  }
}

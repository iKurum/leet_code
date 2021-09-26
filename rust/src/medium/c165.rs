#![allow(dead_code)]

struct Solution;

impl Solution {
  /// 给你两个版本号 version1 和 version2 ，请你比较它们。
  ///
  /// 版本号由一个或多个修订号组成，各修订号由一个 '.' 连接。每个修订号由 **多位数字** 组成，可能包含 **前导零** 。每个版本号至少包含一个字符。修订号从左到右编号，下标从 0 开始，最左边的修订号下标为 0 ，下一个修订号下标为 1 ，以此类推。例如，2.5.33 和 0.1 都是有效的版本号。
  ///
  ///比较版本号时，请按从左到右的顺序依次比较它们的修订号。比较修订号时，只需比较 **忽略任何前导零后的整数值** 。也就是说，修订号 1 和修订号 001 **相等** 。如果版本号没有指定某个下标处的修订号，则该修订号视为 0 。例如，版本 1.0 小于版本 1.1 ，因为它们下标为 0 的修订号相同，而下标为 1 的修订号分别为 0 和 1 ，0 < 1 。
  ///
  /// 返回规则如下：
  ///
  /// - 如果 version1 > version2 返回 1，
  /// - 如果 version1 < version2 返回 -1，
  /// - 除此之外返回 0。
  ///
  /// 提示：
  /// ```
  ///   1 <= version1.length, version2.length <= 500
  ///   version1 和 version2 仅包含数字和 '.'
  ///   version1 和 version2 都是 有效版本号
  ///   version1 和 version2 的所有修订号都可以存储在 32 位整数 中
  /// ```
  fn compare_version(version1: String, version2: String) -> i32 {
    let a1 = version1.split(".").collect::<Vec<&str>>();
    let a2 = version2.split(".").collect::<Vec<&str>>();

    let mut r = 0;
    while r < a1.len() || r < a2.len() {
      let a = if r < a1.len() {
        a1[r].parse::<i32>().unwrap()
      } else {
        0
      };

      let b = if r < a2.len() {
        a2[r].parse::<i32>().unwrap()
      } else {
        0
      };

      if a > b {
        return 1;
      }

      if a < b {
        return -1;
      }

      r += 1;
    }

    0
  }
}

pub fn test(version1: String, version2: String) -> i32 {
  Solution::compare_version(version1, version2)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn compare_version() {
    assert_eq!(
      1,
      Solution::compare_version("1.0.01".to_string(), "1".to_string())
    )
  }
}

package medium

// 给你两个整数 a 和 b ，不使用 运算符 + 和 - ​​​​​​​，计算并返回两整数之和。
//
// 提示：
//
// -1000 <= a, b <= 1000
func C371(a int, b int) int {
	for b != 0 {
		carry := (a & b) << 1
		a ^= b
		b = int(carry)
	}
	return a
}

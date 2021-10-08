package medium

// 所有 DNA 都由一系列缩写为 'A'，'C'，'G' 和 'T' 的核苷酸组成，例如："ACGAATTCCG"。在研究 DNA 时，识别 DNA 中的重复序列有时会对研究非常有帮助。
//
// 编写一个函数来找出所有目标子串，目标子串的长度为 10，且在 DNA 字符串 s 中出现次数超过一次。
//
// 提示：
//
// 0 <= s.length <= 10^5
//
// s[i] 为 'A'、'C'、'G' 或 'T'
func C187(s string) (ans []string) {
	const l = 10
	var bin = map[byte]int{'A': 0, 'C': 1, 'G': 2, 'T': 3}

	n := len(s)
	if n <= l {
		return
	}
	x := 0
	for _, ch := range s[:l-1] {
		x = x<<2 | bin[byte(ch)]
	}
	cnt := map[int]int{}
	for i := 0; i <= n-l; i++ {
		x = (x<<2 | bin[s[i+l-1]]) & (1<<(l*2) - 1)
		cnt[x]++
		if cnt[x] == 2 {
			ans = append(ans, s[i:i+l])
		}
	}
	return ans
}

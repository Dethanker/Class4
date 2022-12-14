use std::u32;

fn sum(numbers: &[u32]) -> Option<u32> {
    let mut result = 0;
    for &number in numbers {
        // 如果当前的结果已经超出了 u32 的最大值，则返回 None。
        if result > u32::MAX - number {
            return None;
        }
        result += number;
    }
    Some(result)
}

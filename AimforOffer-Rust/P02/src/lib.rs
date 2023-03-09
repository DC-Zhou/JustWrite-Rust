///
/// Question:
/// input 2 string,which is integer in binary,
/// return the sum of them in string
///
/// 问题描述：
/// 输入两个表示二进制的字符串，计算它们的和并且以字符串的形式输出
///
/// Examples：
/// input: "01"  "00"
/// output: "01"
///
pub fn add(a:&str, b:&str) -> String {

    let mut i = a.len() - 1;
    let mut j = b.len() - 1;
    let mut carry = 0;

    while i >= 0 || j >= 0 {
        if i >= 0 {

        }

    }



}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add("0", "1"), "1");
        assert_eq!(add("1", "1"), "10");
    }
}

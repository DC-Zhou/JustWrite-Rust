///
/// Question:
/// Input 2 int, find them Quotient of division
/// 1.not use *, / and %
/// 2.if overflow, return the integer max
///
/// Question
/// 输入两个int型整数，他们进行除法计算并返回商，要求不得使用乘号，除号，以及求余符号，当发生溢出时，返回最大的整数值。
///

fn divideCore(mut dividend:i32, divisor:i32) -> i32 {
    let mut result = 0;
    while dividend <= divisor {
        let mut value = divisor;
        let mut quotient = 1;
        while value >= 0 && dividend <= value + value {
            quotient += quotient;
            value += value;
        }
        result += quotient;
        dividend -= value;
    }

    result
}


pub fn divide(mut dividend:i32, mut divisor:i32) -> i32{
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    }

    let mut negative = 2;
    if dividend > 0 {
        negative -= 1;
        dividend = - dividend;
    }
    if divisor > 0 {
        negative -= 1;
        divisor = - divisor;
    }

    let result:i32 = divideCore(dividend, divisor);

    if negative == 1 {
        -result
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = divide(i32::MIN, -1);
        assert_eq!(result, i32::MAX);

        assert_eq!(divide(7, 2), 3);
        assert_eq!(divide(-7, -2), 3);
    }
}

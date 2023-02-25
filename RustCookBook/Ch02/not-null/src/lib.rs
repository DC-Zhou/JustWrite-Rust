// There is no null


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn option_unwrap(){
        // Options to unwrap Options
        assert_eq!(Some(10).unwrap(), 10);
        assert_eq!(None.unwrap_or(10), 10);
        assert_eq!(None.unwrap_or_else(|| 5 * 2), 10);

        Option::<i32>::None.unwrap();
        Option::<i32>::None.expect("Better sat something when panicking");
    }
}

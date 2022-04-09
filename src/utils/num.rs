pub fn add(u: usize, i: i8) -> usize {
    if i.is_negative() {
        u - i.wrapping_abs() as u32 as usize
    } else {
        u + i as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_numbers() {
        assert_eq!(0, add(0, 0));
        assert_eq!(1, add(2, -1));
        assert_eq!(2, add(1, 1));
        assert_eq!(3, add(6, -3));
    }
}

/// Converts a uppercase ascii char to its lowercase counterpart
/// 
/// A -> 0b1000001
/// Z -> 0b1011010
/// So every char in the range A-Z in binary form only varies in the 5 LSBs
/// 
/// To convert from upper to lower, we need to add 32
/// 32 -> 0b0100000
/// 
/// And since the 6th bit is always 0 for uppercase letters, we can just set it to one with a binary OR
pub fn to_lower(c: char) -> Option<char> {
    if !c.is_ascii_uppercase() {
        return None;
    }

    char::from_u32((c as u32) | 0x20)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_to_lower() {
        assert_eq!(Some('a'), to_lower('A'));
        assert_eq!(Some('z'), to_lower('Z'));
        assert_eq!(None, to_lower('6'));
    }
}

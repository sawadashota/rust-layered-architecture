use nanoid::nanoid;

const HEX: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
];

const ALPHABET: [char; 36] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'u', 'z'
];

pub fn hex(length: usize) -> String {
    nanoid!(length, &HEX)
}

pub fn alphabet(length: usize) -> String {
    nanoid!(length, &ALPHABET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        let result = hex(10);
        assert_eq!(result.len(), 10);
        assert!(!result.contains("z"));
    }

    #[test]
    fn test_alphabet() {
        let result = alphabet(10);
        assert_eq!(result.len(), 10);
        assert!(!result.contains("-"));
    }
}

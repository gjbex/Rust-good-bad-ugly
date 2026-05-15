pub const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];
pub const ERROR_TOKENS: [char; 12] = ['B', 'D', 'E', 'F', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O'];

pub fn is_valid_nucleotide(value: char) -> bool {
    VALID_NUCLEOTIDES.contains(&value)
}

pub fn is_error_token(value: char) -> bool {
    ERROR_TOKENS.contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_valid_nucleotides() {
        for nucleotide in VALID_NUCLEOTIDES {
            assert!(is_valid_nucleotide(nucleotide));
        }

        for nucleotide in ERROR_TOKENS {
            assert!(!is_valid_nucleotide(nucleotide));
        }
    }

    #[test]
    fn error_tokens_do_not_include_valid_nucleotides() {
        for token in ERROR_TOKENS {
            assert!(is_error_token(token));
            assert!(!is_valid_nucleotide(token));
        }
    }
}

use std::fmt::{Display, Formatter, write};
use std::str::FromStr;
use anyhow::{Result, Error};
#[derive(Eq, PartialEq, Debug, Default)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ChunkTypeErrorKind {
    TooManyChars,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseChunkTypeError {
    kind: ChunkTypeErrorKind,
}




impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ParseChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().all(|c| c.is_ascii_alphabetic()) {
            Ok(ChunkType{chunk_type: value})
        } else {
            Err(ParseChunkTypeError { kind: ChunkTypeErrorKind::TooManyChars })
        }
    }
}

impl Display for ParseChunkTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseChunkTypeError!")
    }
}

impl std::error::Error for ParseChunkTypeError {

}

impl FromStr for ChunkType {
    type Err = ParseChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 && s.bytes().into_iter().all(|c| c.is_ascii_alphabetic()) {
            Ok(ChunkType{chunk_type: <[u8; 4]>::try_from(s.as_bytes()).unwrap() })
        } else {
            Err(ParseChunkTypeError { kind: ChunkTypeErrorKind::TooManyChars })
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.chunk_type.to_vec()).unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }

    pub fn is_valid(&self) -> bool {
        self.chunk_type.iter().all(|c| c.is_ascii_alphabetic()) && self.chunk_type[2].is_ascii_uppercase()
    }

    pub fn is_critical(&self) -> bool {
        self.chunk_type[0].is_ascii_uppercase()
    }

    pub fn is_public(&self) -> bool {
        self.chunk_type[1].is_ascii_uppercase()
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.chunk_type[2].is_ascii_uppercase()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.chunk_type[3].is_ascii_lowercase()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
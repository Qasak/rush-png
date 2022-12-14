use std::fmt::{Display, Formatter, write};
use std::io::Read;
use crate::chunk_type::ChunkType;
use anyhow::Result;
use crc::{Crc, Algorithm, CRC_32_ISO_HDLC};
use crate::png::PngError;

pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Eq, PartialEq, Debug)]
pub struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug)]
pub struct ChunkError;

impl std::fmt::Display for ChunkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk Error")
    }
}


impl std::error::Error for ChunkError {}




impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;
    // TODO: Optimisation code here
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let length = u32::from_be_bytes(value[0..4].try_into().unwrap());
        let chunk_type: &[u8; 4] = <&[u8; 4]>::try_from(&value[4..8]).unwrap();
        println!("chunk_type: {:?}", String::from_utf8(chunk_type.to_vec()));
        let data = &value[8..(8 + length as usize)];
        let crc = u32::from_be_bytes(value[(8 + length as usize)..(12 + length as usize)].try_into().unwrap());
        let mix = chunk_type
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<_>>();
        let real_crc = CASTAGNOLI.checksum(mix.as_slice());
        if crc == real_crc {
            Ok(Chunk { length,
                chunk_type: {ChunkType::try_from(*chunk_type).unwrap()},
                data: data.to_vec(),
                crc
            })
        } else {
            Err(ChunkError)
        }

    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "chunk_type: {}\ndata: {}\nlength:{}\ncrc:{}",
               self.chunk_type,
               String::from_utf8(self.data.clone()).unwrap(),
               self.length,
               self.crc
        )
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mix = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<_>>();
        let length = data.len() as u32;
        let crc = CASTAGNOLI.checksum(mix.as_slice());
        Chunk {
            chunk_type,
            data,
            length,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let s = String::from_utf8(self.data.clone())?;
        Ok(s)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let all= self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect();
        all
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref())?
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt")?;
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string()?;
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref())?;

        let chunk_string = chunk.data_as_string()?;
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref())?;

        let _chunk_string = format!("{}", chunk);
    }
}
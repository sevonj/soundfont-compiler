use riff::{ChunkContents, ChunkId};

use super::GenAmountType;

#[derive(Debug, Clone)]
pub struct PresetGenList {
    pub contents: Vec<PresetGen>,
}

impl From<PresetGenList> for ChunkContents {
    fn from(value: PresetGenList) -> Self {
        println!("Packing pgen: {value:?}");

        let mut contents = vec![];
        for gen in value.contents {
            contents.append(&mut gen.as_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"pgen" }, contents)
    }
}

impl PresetGenList {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct PresetGen {
    pub sf_gen_oper: u16, // SFGenerator 16-bit enum value
    pub gen_amount: GenAmountType,
}

impl PresetGen {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.append(&mut self.sf_gen_oper.to_le_bytes().to_vec());
        bytes.append(&mut self.gen_amount.as_bytes());

        assert_eq!(bytes.len(), 4);

        bytes
    }

    pub fn terminal() -> Self {
        Self {
            sf_gen_oper: 0,
            gen_amount: GenAmountType::Signed(0),
        }
    }
}

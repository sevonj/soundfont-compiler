use riff::{ChunkContents, ChunkId};

use super::GenAmountType;

#[derive(Debug, Clone)]
pub struct PresetGenList {
    pub contents: Vec<PresetGen>,
}

impl Default for PresetGenList {
    fn default() -> Self {
        Self::new()
    }
}

impl PresetGenList {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }

    pub fn to_riff(&self) -> ChunkContents {
        let mut contents = vec![];
        for gen in &self.contents {
            contents.append(&mut gen.to_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"pgen" }, contents)
    }
}

#[derive(Debug, Clone)]
pub struct PresetGen {
    pub sf_gen_oper: u16, // SFGenerator 16-bit enum value
    pub gen_amount: GenAmountType,
}

impl PresetGen {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.append(&mut self.sf_gen_oper.to_le_bytes().to_vec());
        bytes.append(&mut self.gen_amount.to_bytes());

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

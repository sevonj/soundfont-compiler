use riff::{ChunkContents, ChunkId};

use super::GenAmountType;

#[derive(Debug, Clone)]
pub struct InstGenList {
    pub contents: Vec<InstrumentGen>,
}

impl Default for InstGenList {
    fn default() -> Self {
        Self::new()
    }
}

impl InstGenList {
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

        ChunkContents::Data(ChunkId { value: *b"igen" }, contents)
    }
}

#[derive(Debug, Clone)]
pub struct InstrumentGen {
    pub sf_gen_oper: u16, // SFInstrumentGen 16-bit enum value
    pub gen_amount: GenAmountType,
}

impl InstrumentGen {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.append(&mut self.sf_gen_oper.to_le_bytes().to_vec());
        bytes.append(&mut self.gen_amount.to_bytes());

        assert_eq!(bytes.len(), 4);

        bytes
    }

    /// Null valued terminal record
    pub fn terminal() -> Self {
        Self {
            sf_gen_oper: 0,
            gen_amount: GenAmountType::Signed(0),
        }
    }
}

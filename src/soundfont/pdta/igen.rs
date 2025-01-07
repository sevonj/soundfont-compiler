use riff::{ChunkContents, ChunkId};

use super::GenAmountType;

#[derive(Debug, Clone)]
pub struct InstGenList {
    pub contents: Vec<InstrumentGen>,
}

impl From<InstGenList> for ChunkContents {
    fn from(value: InstGenList) -> Self {
        println!("Packing igen: {value:?}");

        let mut contents = vec![];
        for gen in value.contents {
            contents.append(&mut gen.as_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 4, 0);

        ChunkContents::Data(ChunkId { value: *b"igen" }, contents)
    }
}

impl InstGenList {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct InstrumentGen {
    pub sf_gen_oper: u16, // SFInstrumentGen 16-bit enum value
    pub gen_amount: GenAmountType,
}

impl InstrumentGen {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.append(&mut self.sf_gen_oper.to_le_bytes().to_vec());
        bytes.append(&mut self.gen_amount.as_bytes());

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

use riff::{ChunkContents, ChunkId};

pub enum SampleError {
    SampleTooShort,
    LoopTooShort,
    LoopNotEnoughLead,
    LoopNotEnoughTail,
    TerminalNotNull,
}
impl std::fmt::Display for SampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::SampleTooShort => {
                write!(f, "Sample data must be at least 48 data points long.")
            }
            SampleError::LoopTooShort => {
                write!(f, "The loop must be at least 32 data points long.")
            }
            SampleError::LoopNotEnoughLead => {
                write!(f, "There must be at least 8 data points before startloop")
            }
            SampleError::LoopNotEnoughTail => {
                write!(f, "There must be at least 8 data points after endloop")
            }
            SampleError::TerminalNotNull => {
                write!(f, "Terminal sample must be null.")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SampleList {
    pub contents: Vec<SampleHeader>,
}

impl From<SampleList> for ChunkContents {
    fn from(value: SampleList) -> Self {
        println!("Packing shdr: {value:?}");

        let mut contents = vec![];
        for header in value.contents {
            let _ = header.validate();
            contents.append(&mut header.as_bytes());
        }

        assert_ne!(contents.len(), 0);
        assert_eq!(contents.len() % 46, 0);

        ChunkContents::Data(ChunkId { value: *b"shdr" }, contents)
    }
}

impl SampleList {
    pub fn new() -> Self {
        Self { contents: vec![] }
    }
}

#[derive(Debug, Clone)]
pub struct SampleHeader {
    pub name: String,
    pub start: u32,
    pub end: u32,
    pub startloop: u32,
    pub endloop: u32,
    pub sample_rate: u32,
    pub original_pitch: u8,
    pub pitch_correction: i8,
    pub sample_link: u16,
    pub sample_type: u16, //SFSampleLink enum
}

impl SampleHeader {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        let mut name_bytes = self.name.as_bytes().to_vec();
        assert!(name_bytes.len() <= 20);
        name_bytes.resize(20, 0);

        bytes.append(&mut name_bytes);
        bytes.append(&mut self.start.to_le_bytes().to_vec());
        bytes.append(&mut self.end.to_le_bytes().to_vec());
        bytes.append(&mut self.startloop.to_le_bytes().to_vec());
        bytes.append(&mut self.endloop.to_le_bytes().to_vec());
        bytes.append(&mut self.sample_rate.to_le_bytes().to_vec());
        bytes.append(&mut self.original_pitch.to_le_bytes().to_vec());
        bytes.append(&mut self.pitch_correction.to_le_bytes().to_vec());
        bytes.append(&mut self.sample_link.to_le_bytes().to_vec());
        bytes.append(&mut self.sample_type.to_le_bytes().to_vec());

        assert_eq!(bytes.len(), 46);

        bytes
    }

    pub fn validate(&self) -> Result<(), SampleError> {
        // Terminal entry. "End of Samples",
        if self.name == "EOS" {
            if self.start != 0
                || self.end != 0
                || self.startloop != 0
                || self.endloop != 0
                || self.sample_rate != 0
                || self.original_pitch != 0
                || self.pitch_correction != 0
                || self.sample_link != 0
                || self.sample_type != 0
            {
                return Err(SampleError::TerminalNotNull);
            }
            return Ok(());
        }

        // Sample must be at least 48 data points long.
        if self.end - self.start < 48 {
            return Err(SampleError::SampleTooShort);
        }

        // The loop must be at least 32 data points long.
        if self.endloop - self.startloop < 32 {
            return Err(SampleError::LoopTooShort);
        }

        // There must be at least 8 data points before startloop
        if self.startloop - self.start < 8 {
            return Err(SampleError::LoopNotEnoughLead);
        }

        // There must be at least 8 data points after endloop
        if self.end - self.endloop < 8 {
            return Err(SampleError::LoopNotEnoughTail);
        }

        Ok(())
    }

    pub fn terminal() -> Self {
        Self {
            name: "EOS".into(),
            start: 0,
            end: 0,
            startloop: 0,
            endloop: 0,
            sample_rate: 0,
            original_pitch: 0,
            pitch_correction: 0,
            sample_link: 0,
            sample_type: 0,
        }
    }
}

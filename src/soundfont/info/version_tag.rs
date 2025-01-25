#[derive(Debug, Default, Clone, Copy)]
pub struct VersionTag {
    pub major: u16,
    pub minor: u16,
}

impl VersionTag {
    pub fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [self.major.to_le_bytes(), self.minor.to_le_bytes()].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() {
        let tag = VersionTag::default();
        assert_eq!(tag.to_bytes().len(), 4);
    }
}

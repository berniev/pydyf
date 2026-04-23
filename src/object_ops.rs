pub struct ObjectOps {
    last_object_number: ObjectNumber,
}

impl ObjectOps {
    pub fn new() -> Self {
        Self {
            // 0 is in xref table as 'free'. is gen# 65535, else 0 for new
            last_object_number: ObjectNumber::new(0),
        }
    }

    pub fn last_object_number(&self) -> ObjectNumber {
        self.last_object_number
    }

    pub fn next_object_number(&mut self) -> ObjectNumber {
        self.last_object_number.object_number += 1;

        self.last_object_number
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ObjectNumber {
    object_number: u64,
}

impl ObjectNumber {
    pub fn new(value: u64) -> Self {
        Self {
            object_number: value,
        }
    }

    pub fn value(self) -> u64 {
        self.object_number
    }

    pub fn to_string(&self) -> String {
        self.object_number.to_string()
    }
}

impl PartialEq for ObjectNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for ObjectNumber {}

impl PartialOrd for ObjectNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObjectNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

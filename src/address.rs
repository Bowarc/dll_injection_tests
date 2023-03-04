#[allow(dead_code)]
type Addr = Address;

#[derive(Clone, Copy)]
pub struct Address(usize);

impl std::ops::Deref for Address {
    type Target = usize;
    fn deref(&self) -> &usize {
        &self.0
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<usize> for Address {
    fn from(a: usize) -> Self {
        Self(a)
    }
}

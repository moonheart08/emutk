/// A trait that allows an object to be converted to and from arbitrary endianness bytes.
pub trait ByteRepr: Sized {
    /// The length of the value in bytes.
    const BYTE_LEN: usize;

    /// Convert little endian bytes into Self.
    /// Returns None if the conversion fails.
    fn try_from_le_bytes(b: &[u8]) -> Option<Self>;

    /// Convert little endian bytes into Self.
    /// Panics if the conversion fails!
    fn from_le_bytes(b: &[u8]) -> Self;

    /// Copy Self into a little endian byte buffer.
    /// Panics if `dest` is too small to contain Self.
    fn copy_into_le_bytes(self, dest: &mut [u8]);

    /// Convert Self into a little endian byte buffer.
    fn into_le_bytes(self) -> Vec<u8>;

    /// Convert big endian bytes into Self.
    /// Returns None if the conversion fails.
    fn try_from_be_bytes(b: &[u8]) -> Option<Self>;

    /// Convert little endian bytes into Self.
    /// Panics if the conversion fails!
    fn from_be_bytes(b: &[u8]) -> Self;

    /// Copy Self into a big endian byte buffer.
    /// Panics if `dest` is too small to contain Self.
    fn copy_into_be_bytes(self, dest: &mut [u8]);

    /// Convert Self into a big endian byte buffer.
    fn into_be_bytes(self) -> Vec<u8>;
}


// This is not implemented for usize and isize for sanity reasons.


impl ByteRepr for bool {
    const BYTE_LEN: usize = 1;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v != 0)
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v != 0)
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        b[0] != 0
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        b[0] != 0
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        dest[0] = self as u8;
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        dest[0] = self as u8;
    }

    fn into_be_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }

    fn into_le_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }
}


impl ByteRepr for u8 {
    const BYTE_LEN: usize = 1;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v)
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v)
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        b[0]
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        b[0]
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        dest[0] = self;
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        dest[0] = self;
    }

    fn into_be_bytes(self) -> Vec<u8> {
        vec![self]
    }

    fn into_le_bytes(self) -> Vec<u8> {
        vec![self]
    }
}

impl ByteRepr for i8 {
    const BYTE_LEN: usize = 1;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v as i8)
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v as i8)
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        b[0] as i8
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        b[0] as i8
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        dest[0] = self as u8;
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        dest[0] = self as u8;
    }

    fn into_be_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }

    fn into_le_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }
}

impl ByteRepr for u16 {
    const BYTE_LEN: usize = 2;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 1 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 1 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 2] = [0; 2];
        boilerplate.copy_from_slice(b);
        u16::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 2] = [0; 2];
        boilerplate.copy_from_slice(b);
        u16::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..2].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..2].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for i16 {
    const BYTE_LEN: usize = 2;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 1 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 1 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 2] = [0; 2];
        boilerplate.copy_from_slice(b);
        i16::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 2] = [0; 2];
        boilerplate.copy_from_slice(b);
        i16::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..2].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..2].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for u32 {
    const BYTE_LEN: usize = 4;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        u32::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        u32::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for i32 {
    const BYTE_LEN: usize = 4;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        i32::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        i32::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for f32 {
    const BYTE_LEN: usize = 4;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        f32::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        f32::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for u64 {
    const BYTE_LEN: usize = 8;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        u64::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        u64::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for i64 {
    const BYTE_LEN: usize = 8;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        i64::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        i64::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for f64 {
    const BYTE_LEN: usize = 8;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        f64::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        f64::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for u128 {
    const BYTE_LEN: usize = 16;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 15 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 15 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 16] = [0; 16];
        boilerplate.copy_from_slice(b);
        u128::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 16] = [0; 16];
        boilerplate.copy_from_slice(b);
        u128::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for i128 {
    const BYTE_LEN: usize = 16;

    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 15 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 15 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 16] = [0; 16];
        boilerplate.copy_from_slice(b);
        i128::from_le_bytes(boilerplate)
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 16] = [0; 16];
        boilerplate.copy_from_slice(b);
        i128::from_be_bytes(boilerplate)
    }

    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}


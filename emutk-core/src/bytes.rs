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

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v != 0)
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v != 0)
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        b[0] != 0
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        b[0] != 0
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        dest[0] = self as u8;
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        dest[0] = self as u8;
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }
}


impl ByteRepr for u8 {
    const BYTE_LEN: usize = 1;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v)
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v)
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        b[0]
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        b[0]
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        dest[0] = self;
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        dest[0] = self;
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        vec![self]
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        vec![self]
    }
}

impl ByteRepr for i8 {
    const BYTE_LEN: usize = 1;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v as i8)
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if let Some(v) = b.get(0) {
            Some(*v as i8)
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        b[0] as i8
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        b[0] as i8
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        dest[0] = self as u8;
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        dest[0] = self as u8;
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        vec![self as u8]
    }
}

impl ByteRepr for u16 {
    const BYTE_LEN: usize = 2;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 1 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 1 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 2] = [0; 2];
        boilerplate.copy_from_slice(b);
        u16::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 2] = [0; 2];
        boilerplate.copy_from_slice(b);
        u16::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..2].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..2].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for i16 {
    const BYTE_LEN: usize = 2;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 1 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 1 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 2] = [0; 2];
        boilerplate.copy_from_slice(b);
        i16::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 2] = [0; 2];
        boilerplate.copy_from_slice(b);
        i16::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..2].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..2].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for u32 {
    const BYTE_LEN: usize = 4;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        u32::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        u32::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for i32 {
    const BYTE_LEN: usize = 4;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        i32::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        i32::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for f32 {
    const BYTE_LEN: usize = 4;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 3 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        f32::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 4] = [0; 4];
        boilerplate.copy_from_slice(b);
        f32::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..4].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for u64 {
    const BYTE_LEN: usize = 8;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        u64::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        u64::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for i64 {
    const BYTE_LEN: usize = 8;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        i64::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        i64::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for f64 {
    const BYTE_LEN: usize = 8;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 7 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        f64::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 8] = [0; 8];
        boilerplate.copy_from_slice(b);
        f64::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..8].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for u128 {
    const BYTE_LEN: usize = 16;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 15 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 15 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 16] = [0; 16];
        boilerplate.copy_from_slice(b);
        u128::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 16] = [0; 16];
        boilerplate.copy_from_slice(b);
        u128::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..16].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..16].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl ByteRepr for i128 {
    const BYTE_LEN: usize = 16;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 15 {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() > 15 {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 16] = [0; 16];
        boilerplate.copy_from_slice(b);
        i128::from_le_bytes(boilerplate)
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        let mut boilerplate: [u8; 16] = [0; 16];
        boilerplate.copy_from_slice(b);
        i128::from_be_bytes(boilerplate)
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_le_bytes();
        dest[..16].copy_from_slice(&bytes);
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        let bytes = self.to_be_bytes();
        dest[..16].copy_from_slice(&bytes);
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

/// Used to generate a ByteRepr implementation for arrays
macro_rules! gen_byterepr_impls_array {
    ($($arrlen:literal)+) => {
        $(
            impl<T: ByteRepr + Default + Copy> ByteRepr for [T; $arrlen] {
                const BYTE_LEN: usize = T::BYTE_LEN * $arrlen;
            
                #[inline]
                fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
                    if b.len() < Self::BYTE_LEN {
                        None
                    } else {
                        Some(Self::from_le_bytes(b))
                    }
                }
            
                #[inline]
                fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
                    if b.len() < Self::BYTE_LEN {
                        None
                    } else {
                        Some(Self::from_be_bytes(b))
                    }
                }
            
                #[inline]
                fn from_le_bytes(b: &[u8]) -> Self {
                    let mut out: [T; $arrlen] = Default::default();
            
                    for i in 0..(out.len()) {
                        let val_start = T::BYTE_LEN * i;
                        let val_end = val_start + T::BYTE_LEN;
                        out[i] = T::from_le_bytes(&b[val_start..=val_end]);
                    }
            
                    out
                }
            
                #[inline]
                fn from_be_bytes(b: &[u8]) -> Self {
                    let mut out: [T; $arrlen] = Default::default();
            
                    for i in 0..(out.len()) {
                        let val_start = T::BYTE_LEN * i;
                        let val_end = val_start + T::BYTE_LEN;
                        out[i] = T::from_be_bytes(&b[val_start..=val_end]);
                    }
            
                    out
                }
            
                #[inline]
                fn copy_into_le_bytes(self, dest: &mut [u8]) {
                    assert!(dest.len() >= Self::BYTE_LEN);
            
                    for i in 0..$arrlen {
                        let val_start = T::BYTE_LEN * i;
                        let val_end = val_start + T::BYTE_LEN;
                        self[i].copy_into_le_bytes(&mut dest[val_start..=val_end]);
                    }
                }
            
                #[inline]
                fn copy_into_be_bytes(self, dest: &mut [u8]) {
                    assert!(dest.len() >= Self::BYTE_LEN);
                    
                    for i in 0..$arrlen {
                        let val_start = T::BYTE_LEN * i;
                        let val_end = val_start + T::BYTE_LEN;
                        self[i].copy_into_be_bytes(&mut dest[val_start..=val_end]);
                    }
                }
            
                #[inline]
                fn into_be_bytes(self) -> Vec<u8> {
                    let mut out_vec = vec![Default::default(); Self::BYTE_LEN];
                    self.copy_into_be_bytes(&mut out_vec);
                    out_vec
                }
            
                #[inline]
                fn into_le_bytes(self) -> Vec<u8> {
                    let mut out_vec = vec![Default::default(); Self::BYTE_LEN];
                    self.copy_into_le_bytes(&mut out_vec);
                    out_vec
                }
            }
        )+
    };
}

#[allow(unused_variables)]
gen_byterepr_impls_array!(
    1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
);

macro_rules! gen_byterepr_impls_tuple {
    ($(
        $tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            // ByteRepr mandates Sized
            impl<$($T:ByteRepr + Default),+> ByteRepr for ($($T,)+) {
                const BYTE_LEN: usize = $($T::BYTE_LEN +)+ 0;

                #[inline]
                fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
                    if b.len() < Self::BYTE_LEN {
                        None
                    } else {
                        Some(Self::from_le_bytes(b))
                    }
                }
            
                #[inline]
                fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
                    if b.len() < Self::BYTE_LEN {
                        None
                    } else {
                        Some(Self::from_be_bytes(b))
                    }
                }

                #[allow(unused_assignments)]
                #[inline]
                fn from_le_bytes(b: &[u8]) -> Self {
                    let mut pos = 0;
                    let mut tup: ($($T,)+)  = ($($T::default(),)+);
                    $(
                        tup.$idx = $T::from_le_bytes(&b[pos..(pos + $T::BYTE_LEN)]);
                        pos += $T::BYTE_LEN;
                    )+

                    tup
                }

                #[allow(unused_assignments)]
                #[inline]
                fn from_be_bytes(b: &[u8]) -> Self {
                    let mut pos = 0;
                    let mut tup: ($($T,)+)  = ($($T::default(),)+);
                    $(
                        tup.$idx = $T::from_be_bytes(&b[pos..(pos + $T::BYTE_LEN)]);
                        pos += $T::BYTE_LEN;
                    )+

                    tup
                }

                #[allow(unused_assignments)]
                #[inline]
                fn copy_into_le_bytes(self, dest: &mut [u8]) {
                    let mut pos = 0;
                    $(
                        self.$idx.copy_into_le_bytes(&mut dest[pos..(pos + $T::BYTE_LEN)]);
                        pos += $T::BYTE_LEN;
                    )+
                }

                #[allow(unused_assignments)]
                #[inline]
                fn copy_into_be_bytes(self, dest: &mut [u8]) {
                    let mut pos = 0;
                    $(
                        self.$idx.copy_into_be_bytes(&mut dest[pos..(pos + $T::BYTE_LEN)]);
                        pos += $T::BYTE_LEN;
                    )+
                }

                #[allow(unused_assignments)]
                #[inline]
                fn into_be_bytes(self) -> Vec<u8> {
                    let mut dest = vec![0;Self::BYTE_LEN];

                    let mut pos = 0;
                    $(
                        self.$idx.copy_into_be_bytes(&mut dest[pos..(pos + $T::BYTE_LEN)]);
                        pos += $T::BYTE_LEN;
                    )+

                    dest
                }

                #[allow(unused_assignments)]
                #[inline]
                fn into_le_bytes(self) -> Vec<u8> {
                    let mut dest = vec![0;Self::BYTE_LEN];

                    let mut pos = 0;
                    $(
                        self.$idx.copy_into_le_bytes(&mut dest[pos..(pos + $T::BYTE_LEN)]);
                        pos += $T::BYTE_LEN;
                    )+

                    dest
                }
            }
        )+
    }
}



gen_byterepr_impls_tuple!
{
    Tuple1 {
        (0) -> A
    }

    Tuple2 {
        (0) -> A
        (1) -> B
    }

    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }

    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
    }

    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
    }

    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
        (5) -> F
    }

    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
        (5) -> F
        (6) -> G
    }

    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }

    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }

    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }

    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }

    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }

    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D 
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
        (12) -> M
    }
}

//TODO: make array and tuple implementations even more generic.
/*
impl<T: ByteRepr> ByteRepr for [T; 4] {
    const BYTE_LEN: usize = T::BYTE_LEN * 4;

    #[inline]
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() < Self::BYTE_LEN {
            None
        } else {
            Some(Self::from_le_bytes(b))
        }
    }

    #[inline]
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() < Self::BYTE_LEN {
            None
        } else {
            Some(Self::from_be_bytes(b))
        }
    }

    #[inline]
    fn from_le_bytes(b: &[u8]) -> Self {
        let mut out: [MaybeUninit<T>; 4] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        for i in 0..(out.len()) {
            let val_start = T::BYTE_LEN * i;
            let val_end = val_start + T::BYTE_LEN;
            out[i] = MaybeUninit::new(T::from_le_bytes(&b[val_start..=val_end]));
        }

        unsafe { transmute::<_, [T; 4]>(out) }
    }

    #[inline]
    fn from_be_bytes(b: &[u8]) -> Self {
        unimplemented!();
    }

    #[inline]
    fn copy_into_le_bytes(self, dest: &mut [u8]) {
        unimplemented!();
    }

    #[inline]
    fn copy_into_be_bytes(self, dest: &mut [u8]) {
        unimplemented!();
    }

    #[inline]
    fn into_be_bytes(self) -> Vec<u8> {
        unimplemented!();
    }

    #[inline]
    fn into_le_bytes(self) -> Vec<u8> {
        unimplemented!();
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_tuple_len() {
        assert_eq!(<(i32, i32, i32)>::BYTE_LEN, 12);
    }

    #[test]
    fn convert_tuple_to_bytes() {
        let tup: (i32, i32, i32) = (0, 1, 2);
        let expected = [0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0];

        assert_eq!(&tup.into_le_bytes(), &expected);
    }

    #[test]
    fn check_array_len() {
        assert_eq!(<[i32; 4]>::BYTE_LEN, 16);
    }
}
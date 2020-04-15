use std::num::*;
use std::mem::{
    MaybeUninit,
    transmute,
    transmute_copy,
};
use bytemuck::Pod;

/// A trait that allows an object to be converted to and from arbitrary endianness bytes.
pub trait ByteRepr: Sized + Pod + Clone {
    /// The length of the value in bytes.
    const BYTE_LEN: usize;

    /// Convert little endian bytes into Self.
    /// Returns None if the conversion fails.
    fn try_from_le_bytes(b: &[u8]) -> Option<Self> {
        if b.len() >= Self::BYTE_LEN {
            Some(<Self as ByteRepr>::from_le_bytes(b))
        } else {
            None
        }
    }

    /// Convert little endian bytes into Self.
    /// Panics if the conversion fails!
    fn from_le_bytes(b: &[u8]) -> Self;

    /// Copy Self into a little endian byte buffer.
    /// Panics if `dest` is too small to contain Self.
    fn copy_to_le_bytes(self, dest: &mut [u8]);

    /// Convert Self into a little endian byte buffer.
    fn into_le_bytes(self) -> Vec<u8>;

    /// Convert big endian bytes into Self.
    /// Returns None if the conversion fails.
    fn try_from_be_bytes(b: &[u8]) -> Option<Self> {
        if b.len() >= Self::BYTE_LEN {
            Some(<Self as ByteRepr>::from_be_bytes(b))
        } else {
            None
        }
    }

    /// Convert little endian bytes into Self.
    /// Panics if the conversion fails!
    fn from_be_bytes(b: &[u8]) -> Self;

    /// Copy Self into a big endian byte buffer.
    /// Panics if `dest` is too small to contain Self.
    fn copy_to_be_bytes(self, dest: &mut [u8]);

    /// Convert Self into a big endian byte buffer.
    fn into_be_bytes(self) -> Vec<u8>;

    /// Convert big endian bytes into Self.
    /// Returns None if the conversion fails.
    fn try_from_ne_bytes(b: &[u8]) -> Option<Self> {
        if b.len() >= Self::BYTE_LEN {
            Some(<Self as ByteRepr>::from_ne_bytes(b))
        } else {
            None
        }
    }

    /// Convert little endian bytes into Self.
    /// Panics if the conversion fails!
    fn from_ne_bytes(b: &[u8]) -> Self {
        if cfg!(endianness = "little") {
            <Self as ByteRepr>::from_le_bytes(b)
        } else {
            <Self as ByteRepr>::from_be_bytes(b)
        }
    }

    /// Copy Self into a big endian byte buffer.
    /// Panics if `dest` is too small to contain Self.
    fn copy_to_ne_bytes(self, dest: &mut [u8]) {
        if cfg!(endianness = "little") {
            <Self as ByteRepr>::copy_to_le_bytes(self, dest)
        } else {
            <Self as ByteRepr>::copy_to_be_bytes(self, dest)
        }
    }

    /// Convert Self into a big endian byte buffer.
    fn into_ne_bytes(self) -> Vec<u8> {
        if cfg!(endianness = "little") {
            <Self as ByteRepr>::into_le_bytes(self)
        } else {
            <Self as ByteRepr>::into_be_bytes(self)
        }
    }
}

macro_rules! gen_byterepr_impls_nums {
    ($($T:ty)+) => {
        $(
        impl ByteRepr for $T {
            const BYTE_LEN: usize = std::mem::size_of::<$T>();
        
            #[inline]
            fn from_le_bytes(b: &[u8]) -> Self {
                let mut boilerplate: [u8; Self::BYTE_LEN] = [0; Self::BYTE_LEN];
                boilerplate.copy_from_slice(&b[0..Self::BYTE_LEN]);
                Self::from_le_bytes(boilerplate)
            }
            
            #[inline]
            fn from_be_bytes(b: &[u8]) -> Self {
                let mut boilerplate: [u8; Self::BYTE_LEN] = [0; Self::BYTE_LEN];
                boilerplate.copy_from_slice(&b[0..Self::BYTE_LEN]);
                Self::from_be_bytes(boilerplate)
            }
        
            #[inline]
            fn copy_to_le_bytes(self, dest: &mut [u8]) {
                let bytes = self.to_le_bytes();
                dest[..Self::BYTE_LEN].copy_from_slice(&bytes[0..Self::BYTE_LEN]);
            }
        
            #[inline]
            fn copy_to_be_bytes(self, dest: &mut [u8]) {
                let bytes = self.to_be_bytes();
                dest[..Self::BYTE_LEN].copy_from_slice(&bytes[0..Self::BYTE_LEN]);
            }
        
            fn into_be_bytes(self) -> Vec<u8> {
                self.to_be_bytes().to_vec()
            }
        
            fn into_le_bytes(self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
        )+
    }
}

gen_byterepr_impls_nums!(u8 i8 u16 i16 u32 i32 f32 u64 i64 f64 u128 i128);

macro_rules! gen_byterepr_impls_nonzero_nums {
    ($($T:ty, $J:ty)+) => {
        $(
        impl ByteRepr for Option<$T> {
            const BYTE_LEN: usize = std::mem::size_of::<Option<$T>>();
        
            #[inline]
            fn from_le_bytes(b: &[u8]) -> Self {
                let val = <$J as ByteRepr>::from_le_bytes(b);
                <$T>::new(val)
            }
        
            #[inline]
            fn from_be_bytes(b: &[u8]) -> Self {
                let val = <$J as ByteRepr>::from_be_bytes(b);
                <$T>::new(val)
            }
        
            #[inline]
            fn copy_to_le_bytes(self, dest: &mut [u8]) {
                match self {
                    Some(v) => <$J as ByteRepr>::copy_to_le_bytes(v.get(), dest),
                    None => <$J as ByteRepr>::copy_to_le_bytes(0, dest),
                }
            }
        
            #[inline]
            fn copy_to_be_bytes(self, dest: &mut [u8]) {
                match self {
                    Some(v) => <$J as ByteRepr>::copy_to_be_bytes(v.get(), dest),
                    None => <$J as ByteRepr>::copy_to_be_bytes(0, dest),
                }
            }
        
            fn into_be_bytes(self) -> Vec<u8> {
                let val = match self {
                    Some(v) => v.get(),
                    None => 0 as $J,
                };
                val.to_be_bytes().to_vec()
            }
        
            fn into_le_bytes(self) -> Vec<u8> {
                let val = match self {
                    Some(v) => v.get(),
                    None => 0 as $J,
                };
                val.to_le_bytes().to_vec()
            }
        }
        )+
    }
}

gen_byterepr_impls_nonzero_nums!(NonZeroI8, i8 NonZeroU8, u8 NonZeroI16, i16 NonZeroU16, u16);

impl<T: ByteRepr> ByteRepr for Wrapping<T> {
    const BYTE_LEN: usize = T::BYTE_LEN;

    fn from_le_bytes(b: &[u8]) -> Self {
        Wrapping(<T as ByteRepr>::from_le_bytes(b))
    }

    fn from_be_bytes(b: &[u8]) -> Self {
        Wrapping(<T as ByteRepr>::from_be_bytes(b))
    }

    fn copy_to_le_bytes(self, dest: &mut [u8]) {
        self.0.copy_to_le_bytes(dest);
    }

    fn copy_to_be_bytes(self, dest: &mut [u8]) {
        self.0.copy_to_be_bytes(dest);
    }

    fn into_be_bytes(self) -> Vec<u8> {
        self.0.into_be_bytes()
    }

    fn into_le_bytes(self) -> Vec<u8> {
        self.0.into_le_bytes()
    }
}

macro_rules! gen_byterepr_impls_array {
    ($($arrlen:literal)+) => {
        $(
            impl<T: ByteRepr + Copy> ByteRepr for [T; $arrlen] {
                const BYTE_LEN: usize = T::BYTE_LEN * $arrlen;
            
                fn from_le_bytes(b: &[u8]) -> Self {
                    let mut out: [MaybeUninit<T>; $arrlen] = unsafe {
                        MaybeUninit::uninit().assume_init()
                    };

                    for i in 0..(out.len()) {
                        let val_start = T::BYTE_LEN * i;
                        let val_end = val_start + T::BYTE_LEN;
                        out[i] = MaybeUninit::new(T::from_le_bytes(&b[val_start..val_end]));
                    }
            
                    unsafe { transmute_copy::<_, [T; $arrlen]>(&out) }
                }
            
                fn from_be_bytes(b: &[u8]) -> Self {
                    let mut out: [MaybeUninit<T>; $arrlen] = unsafe {
                        MaybeUninit::uninit().assume_init()
                    };
            
                    for i in 0..(out.len()) {
                        let val_start = T::BYTE_LEN * i;
                        let val_end = val_start + T::BYTE_LEN;
                        out[i] = MaybeUninit::new(T::from_be_bytes(&b[val_start..val_end]));
                    }
            
                    unsafe { transmute_copy::<_, [T; $arrlen]>(&out) }
                }
            
                fn copy_to_le_bytes(self, dest: &mut [u8]) {
                    assert!(dest.len() >= Self::BYTE_LEN);
            
                    for i in 0..$arrlen {
                        let val_start = T::BYTE_LEN * i;
                        let val_end = val_start + T::BYTE_LEN;
                        self[i].copy_to_le_bytes(&mut dest[val_start..val_end]);
                    }
                }
            
                fn copy_to_be_bytes(self, dest: &mut [u8]) {
                    assert!(dest.len() >= Self::BYTE_LEN);
                    
                    for i in 0..$arrlen {
                        let val_start = T::BYTE_LEN * i;
                        let val_end = val_start + T::BYTE_LEN;
                        self[i].copy_to_be_bytes(&mut dest[val_start..val_end]);
                    }
                }
            
                fn into_be_bytes(self) -> Vec<u8> {
                    let mut out_vec = vec![Default::default(); Self::BYTE_LEN];
                    self.copy_to_be_bytes(&mut out_vec);
                    out_vec
                }
            
                fn into_le_bytes(self) -> Vec<u8> {
                    let mut out_vec = vec![Default::default(); Self::BYTE_LEN];
                    self.copy_to_le_bytes(&mut out_vec);
                    out_vec
                }
            }
        )+
    };
}


#[allow(unused_variables)]
gen_byterepr_impls_array!(
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 48 64 96 128 256 512 1024 2048 4096
);
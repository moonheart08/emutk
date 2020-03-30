use bytemuck::{
    Pod,
    bytes_of_mut,
};

/// Forces the input to be little endian, and converts it if it is not.
pub fn pod_is_le<T: Pod>(mut data: T) -> T {
    if cfg!(target_endian = "little") {
        data
    } else {
        let by = bytes_of_mut(&mut data);
        by.reverse();
        data
    }
}

/// Forces the input to be big endian, and converts it if it is not.
pub fn pod_is_be<T: Pod>(mut data: T) -> T {
    if cfg!(target_endian = "big") {
        data
    } else {
        let by = bytes_of_mut(&mut data);
        by.reverse();
        data
    }
}
use nix::sys::ioctl::ioctl_num_type;
use nix::{ioctl_write_buf, request_code_write};

pub struct S3ObjectVersionBuffer {
    pub data: [u8; 32],
}

impl<T: AsRef<str>> From<T> for S3ObjectVersionBuffer {
    fn from(value: T) -> Self {
        let mut buffer = S3ObjectVersionBuffer { data: [0; 32] };
        let bytes = value.as_ref().as_bytes();
        let len = bytes.len().min(buffer.data.len());
        buffer.data[..len].copy_from_slice(&bytes[..len]);
        buffer
    }
}

const MOUNT_S3_IOC_MAGIC: u8 = b'm';

/// IOCTL to set the S3 object version Iid for an inode. Once set,
/// this version id will be used for all S3 operations on the inode.
pub const MOUNT_S3_IOC_TYPE_SET_VERSION: ioctl_num_type =
    request_code_write!(MOUNT_S3_IOC_MAGIC, 0, size_of::<S3ObjectVersionBuffer>());
ioctl_write_buf!(
    ioctl_mount_s3_set_inode_version,
    MOUNT_S3_IOC_MAGIC,
    0,
    S3ObjectVersionBuffer
);

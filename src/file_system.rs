use mockall::automock;
use std::io;
use std::io::Write;

#[automock]
pub trait FileSystem{
    fn write(&self, data: &[u8]) -> io::Result<usize>;
}

pub struct RealFs;
impl FileSystem for RealFs{
    fn write(&self, data: &[u8]) -> io::Result<usize> {
        io::stdout().write(data)
    }   
}
#[derive(Debug)]
pub enum DriverError {
    NotInitialized,
    AlreadyShutDown,
    BufferOverflow(String),
    OutOfBounds(String),
}

pub struct BufferSimulator {
    buffer: Vec<u8>,
}
impl BufferSimulator {
    //constructore that creates a new buffersimulator witha given buffer size
    pub fn new(size: usize) -> Self {
        BufferSimulator {
            buffer: vec![0; size],
        }
    }
}

pub trait ReadOperations {
    async fn read(&self, offset:usize, length:usize) -> Result<&[u8], DriverError>;
}
impl ReadOperations for BufferSimulator {
    async fn read(&self, offset:usize, length:usize) -> Result<&[u8], DriverError> {
        if offset + length > self.buffer.len() {
            return Err(DriverError::OutOfBounds("out of bounds mate, you are trying to access data outside the buffer range".to_string()));
        }

        Ok(&self.buffer[offset..offset + length])
    }
}

pub trait WriteOperations {
    async fn write(&mut self, offset:usize, data:&[u8]) -> Result<(), DriverError>;
}
impl WriteOperations for BufferSimulator{
    async fn write(&mut self, offset:usize, data:&[u8]) -> Result<(), DriverError> {
        if offset >= self.buffer.len() {
            return Err(DriverError::OutOfBounds("out-of-bounds".to_string()));
        }

        if offset + data.len() > self.buffer.len() {
            return Err(DriverError::BufferOverflow("Buffer Overflow bruv".to_string()));
        }

        self.buffer[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }
}

pub struct DriverProcesses<R:ReadOperations, W:WriteOperations> {
    pub initialized: bool,
    pub shut_down: bool,
    pub read_process: R,
    pub write_process: W,
}
impl<R:ReadOperations, W:WriteOperations> DriverProcesses<R,W> {
    pub fn new(read_process:R , write_process:W ) -> Self {
        DriverProcesses {
            initialized: false,
            shut_down: false,
            read_process,
            write_process,
        }
    }

    pub async fn init(&mut self) -> Result<(), DriverError>{
        self.initialized = true;
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), DriverError>{
        if self.shut_down{
            return Err(DriverError::AlreadyShutDown)
        }

        self.shut_down = true;
        Ok(())
    }

    pub async fn read(&self, offset:usize, length:usize) -> Result<&[u8], DriverError> {
        if !self.initialized{ return Err(DriverError::NotInitialized); }
        if self.shut_down{ return Err(DriverError::AlreadyShutDown); }

        Ok(self.read_process.read(offset, length).await?)
    }

    pub async fn write(&mut self, offset:usize, data:&[u8]) -> Result<(), DriverError> {
        if !self.initialized{ return Err(DriverError::NotInitialized); }
        if self.shut_down{ return Err(DriverError::AlreadyShutDown); } 

        self.write_process.write(offset, data).await?;
        Ok(())
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //before writing tests, sort out what the noop does otherwise tests won't really be for
    //anything
}
*/
















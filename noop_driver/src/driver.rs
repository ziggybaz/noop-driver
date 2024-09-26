use std::error::Error;
use std::pin::Pin;
use std::future::Future;

#[derive(Debug)]
enum DriverError {
    NotInitialized,
    AlreadyShutDown,
}

struct Data {
    
}

pub trait ReadOperations {
    async fn read<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Data, DriverError>> + Send + 'a>>;
}

pub trait WriteOperations {
    async fn write<'a>(&'a self, data:Data) -> Pin<Box<dyn Future<Output = Result<(), DriverError>> + Send + 'a>>;
}

pub struct DriverRead;

impl ReadOperations for DriverRead {
    async fn read<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Data, DriverError>> + Send + 'a>>{
        Box::pin(async {Ok(Data {})})
    }
}

pub struct DriverWrite;

impl WriteOperations for DriverWrite {
    async fn write<'a>(&'a self,_data:Data) -> Pin<Box<dyn Future<Output = Result<(), DriverError>> + Send + 'a>> {
        Box::Pin(async { Ok(() ) })
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

    pub async fn read(&self) -> Result<Data, DriverError> {
        if !self.initialized{ return Err(DriverError::NotInitialized); }
        if self.shut_down{ return Err(DriverError::AlreadyShutDown); }

        self.read_process.read().await
    }

    pub async fn write(&self, data:Data) -> Result<(), DriverError> {
        if !self.initialized{ return Err(DriverError::NotInitialized); }
        if self.shut_down{ return Err(DriverError::AlreadyShutDown); } 

        self.write_process.write(data).await
    }
}




















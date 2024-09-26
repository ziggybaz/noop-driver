use std::error::Error;

#[derive(Debug)]
enum DriverError {
    NotInitialized,
    AlreadyShutDown,
}

struct Data {
    
}

pub trait ReadOperations {
    async fn read(&self) -> Result<Data, DriverError>;
}

pub trait WriteOperations {
    async fn write(&self, data:Data) -> Result<(), DriverError>;
}

pub struct DriverRead;

impl ReadOperations for DriverRead {
    async fn read(&self) -> Result<Data, DriverError>{
    }
}

pub struct DriverWrite;

impl WriteOperations for DriverWrite {
    async fn write(&self, _data:Data) -> Result<(), DriverError> {
        Ok(())
    }
}

pub struct DriverProcesses {
    pub initialized: bool,
    pub shut_down: bool,
    pub read_process: Box<dyn ReadOperations>,
    pub write_process: Box<dyn WriteOperations>,
}

impl DriverProcesses {
    fn new(read_process: Box<dyn ReadOperations>, write_process: Box<dyn WriteOperations>) -> Self {
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

    pub async fn write(&self, _data:Data) -> Result<(), DriverError> {
        if !self.initialized{ return Err(DriverError::NotInitialized); }
        if self.shut_down{ return Err(DriverError::AlreadyShutDown); } 

        self.write_process.write(data).await
    }
}




















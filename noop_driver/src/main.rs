#![cfg(all(target_os = "linux", target_arch = "x86_64", feature = "x86_64_linux"))]

#[cfg(not(all(target_os = "linux", target_arch = "x86_64", feature = "x86_64_linux")))]
compile_error!("this useless driver only runs linux on x86_64 archs.");

mod driver;

use crate::driver::{BufferSimulator, DriverProcesses, ReadOperations, WriteOperations};
use std::process;

#[tokio::main]
async fn main() {
    let read_process = BufferSimulator::new(1024);
    let write_process = BufferSimulator::new(1024);

    let mut driver = DriverProcesses::new(read_process, write_process);

    if let Err(e) = driver.init().await {
        eprintln!("Failed to initialize the driver bruv:\n{:?}", e);
        graceful_shutdown(&mut driver).await;
        process::exit(1);
    }

    let _data = driver.read(0, 10).await.expect("Unable to read from buffer, sorting it out");

    //driver.write(0, data).await.expect("Unable to write to buffer mse.");
    
    if let Err(e) = driver.shutdown().await {
        eprintln!("Error shutting down the driver, kindly wait for the system to terminate gracefully bruv:\n{:?}", e);
        process::exit(1);
    }

    println!("Driver has succesfully shutdown");
    process::exit(0);
}

async fn graceful_shutdown<R, W>(driver: &mut DriverProcesses<R, W>)
    where
    R:ReadOperations,
    W:WriteOperations,
{
    if !driver.shut_down {
        if let Err(_e) = driver.shutdown().await {
            eprintln!("Bruv, there was an error during shutdown, I'm trying to handle it, stay calm");
        } else {
            println!("System sorted, driver has succesfully shutdown");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_graceful_shutdown() {
        let read_process = BufferSimulator::new(1024);
        let write_process = BufferSimulator::new(1024);

        let mut driver = DriverProcesses::new(read_process, write_process);

        assert!(!driver.shut_down);

        graceful_shutdown(&mut driver).await;

        assert!(driver.shut_down);
    }
}


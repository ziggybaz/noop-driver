mod driver;

use crate::driver::{DriverRead, DriverWrite, DriverProcesses, ReadOperations, WriteOperations};
use std::process;

#[tokio::main]
async fn main() {
    let read_process = DriverRead;
    let write_process = DriverWrite;

    let mut driver = DriverProcesses::new(read_process, write_process);

    if let Err(e) = driver.init().await {
        eprintln!("Failed to initialize the driver bruv:\n{:?}", e);
        graceful_shutdown(driver).await;
        process::exit(1);
    }

    let data = driver.read().await.unwrap();
    driver.write(data).await.unwrap();
    
    if let Err(e) = driver.shutdown().await {
        eprintln!("Error shutting down the driver, kindly wait for the system to terminate gracefully bruv:\n{:?}", e);
        process::exit(1);
    }

    println!("Driver has succesfully shutdown");
    process::exit(0);
}

async fn graceful_shutdown<R, W>(mut driver: DriverProcesses<R, W>)
    where
    R:ReadOperations,
    W:WriteOperations,
{
    if !driver.shut_down {
        if let Err(_e) = driver.shutdown().await {
            eprintln!("Bruv, there was an error during shutdown, we are trying to handle it, stay calm");
        } else {
            println!("System sorted out, driver succesfully shutdown");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graceful_shutdown() {
    }
}



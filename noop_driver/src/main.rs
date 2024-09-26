mod driver;


use crate::driver::{DriverRead, DriverWrite, DriverProcesses};

#[tokio::main]
async fn main() {
    let read_process = DriverRead;
    let write_process = DriverWrite;

    let mut driver = DriverProcesses::new(read_process, write_process);

    driver.init().await.unwrap();

    let data = driver.read().await.unwrap();
    driver.write(data).await.unwrap();
    
    match driver.shutdown().await {
        Ok(_) => println!("Driver successfully shutdown"),
        Err(e) => eprintln!("Error shutting down the Driver: {:?}", e),
    }
}

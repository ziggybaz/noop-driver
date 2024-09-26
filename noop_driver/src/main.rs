#[tokio::main]
async fn main() {
    let read_process = Box::new(ReadOperations);
    let write_process = Box::new(WriteOperations);

    let mut driver = DriverProcess::new(read_process, write_process);

    driver.init().await.unwrap();

    let data = driver.read().await.unwrap();
    driver.write(data).await.unwrap();
    
    match driver.shutdown().await {
        Ok(_) => println!("Driver successfully shutdown"),
        Err(e) => eprintln!("Error shutting down the Driver: {:?}", e),
    }
}

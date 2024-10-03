if your host system is an x86_64 arch linux then you can run
`cargo build` or `cargo build --features x86_64_linux`

otherwise on other platforms it won't compile as the driver is platform specific. 

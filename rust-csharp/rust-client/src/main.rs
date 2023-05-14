fn main() {
    let result = call_dynamic().unwrap();
    println!("C# said that 1 + 2 = {result}");
}

fn call_dynamic() -> Result<i32, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("/home/rainer/github/rust-samples/rust-csharp/csharp-library/out/csharp-library.so")?;
        let func: libloading::Symbol<unsafe extern fn(i32, i32) -> i32> = lib.get(b"add")?;
        Ok(func(1, 2))
    }
}
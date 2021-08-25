from :https://codeburst.io/how-to-use-rust-to-extend-python-360174ee5819

As I’m building it on my MacBook, I’m going to have to set some linker arguments — they’re not needed on Linux or Windows. On macOS though, I just need to add the following file to the project: .cargo/config
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
Now, all we need to do is to run the following command:
cargo build --release
The resulting binary is in target/release/libmylib.dylib (it would be .so on Linux or .dll on Windows). In order for Python to see it we're going to rename the file to mylib.so:
cp target/release/libmylib.dylib ./mylib.so
Then we can give it a test:
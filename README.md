# Hello Rust

Here you can find a simple Rust CLI application that will get a Github pull request URL and will tell if it is merged or not:

```
cargo run -- -u https://github.com/Restfulness/Restfulness-flutter-app/pull/35  https://github.com/Restfulness/Restfulness-flutter-app/pull/36
   Compiling hello-rust v0.1.0 (/Users/farid.ahmadian/lab/hello-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 9.78s
     Running `target/debug/hello-rust -u 'https://github.com/Restfulness/Restfulness-flutter-app/pull/35' 'https://github.com/Restfulness/Restfulness-flutter-app/pull/36'`
https://api.github.com/repos/Restfulness/Restfulness-flutter-app/pulls/35/merge
Merged
https://api.github.com/repos/Restfulness/Restfulness-flutter-app/pulls/36/merge
Not Merged
```

We created this app during a recording of a short tutorial of Rust lang in Farsi. You can see those video clips inside of one playlist in my youtube channel:
https://www.youtube.com/playlist?list=PL2DfDaq51k1lu9QUQ3VBo0VsblPZcnBlH

Or in the `Rust in Farsi` channel:
https://www.youtube.com/channel/UCIXThmX_uAp2cJTR-hTjdcA

 

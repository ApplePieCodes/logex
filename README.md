# logex

### A simple logger for rust applications

#### How to use

Just use the command
```rust
logex::log("Log Text Here", logex::LogType::Info);
```

Info Can be replaced with Warning, Error, or FatalError to produce different ouputs.
Please note that using LogType::FatalError will cause a panic. This Is intentional. If you don't like this, use LogType::Error.
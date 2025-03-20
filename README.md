# logex

## A simple logger for rust applications

### How to use

Just use the command

```rust
logex::log_info("Log Text Here");
```

Info Can be replaced with warning, error, or fatal_error to produce different ouputs.
Please note that using fatal_error will cause a panic. This Is intentional. If you don't like this, use error.

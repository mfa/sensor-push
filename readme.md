## sensor push

### about

Push sensor data to a custom endpoint every minute.
The original version was in Python, but this one is hopefully more stable over system updates.

Currently only the nvml (Nvidia GPU) is implemented.

### run

```
cargo run -- --url http://example.org/push/ --id rtx2070
```

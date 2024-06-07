extern crate perfetto_rs;

use perfetto_rs::rust_initialize_tracing;
use perfetto_rs::rust_create_observer;
use perfetto_rs::rust_wait_for_tracing_start;
use perfetto_rs::rust_wait;
use perfetto_rs::rust_flush_tracing;
use perfetto_rs::rust_destroy_observer;

fn main() {
    unsafe {
        rust_initialize_tracing();

        let observer = rust_create_observer();
        rust_wait_for_tracing_start(observer);

        rust_wait();

        rust_flush_tracing();
        rust_destroy_observer(observer);
    }
}


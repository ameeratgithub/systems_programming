mod cpu_emulator;
// Uncomment following module if you want to visualize the heap allocations
// mod heap_visualizer;
mod kv_store;

fn main() {
    // heap_visualizer::visualize();
    kv_store::run();
}

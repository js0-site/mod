#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[volo::main]
async fn main() -> volo_boot::Result {
  js0_volo::run().await
}

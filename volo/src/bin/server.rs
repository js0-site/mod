#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[volo::main]
async fn main() -> anyhow::Result<()> {
  js0_volo::run().await
}

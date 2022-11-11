fn main() {
  std::env::set_var("DATABASE_URL", "postgresql://postgres:Boomers4life!123@localhost:5432/Rust");
  api::main();
}
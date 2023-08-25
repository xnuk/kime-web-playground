#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod engine;
mod glue_kime;
mod helper;
mod web_input;
mod web_keycode;

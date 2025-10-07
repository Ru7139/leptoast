mod app_iteself;
mod simple_on_page_item;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(app_iteself::app::App)
}

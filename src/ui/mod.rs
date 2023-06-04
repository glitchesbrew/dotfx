use cursive::views::TextView;

pub fn run() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let conf_result = crate::core::load_config();
    if conf_result.is_ok() {

    }

    siv.add_layer(TextView::new("hello"));

    siv.run()
}

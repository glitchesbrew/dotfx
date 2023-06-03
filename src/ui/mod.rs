use cursive::views::TextView;

pub fn run() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(TextView::new("hello"));

    siv.run()
}

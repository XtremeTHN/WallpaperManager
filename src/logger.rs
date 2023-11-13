use std::io::Write;

pub fn setup_logger() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .write_style(env_logger::WriteStyle::Always)
        .format(|buf, record| {
            let mut style = buf.style();
            let mut file_style = style.clone();
            style.set_color(env_logger::fmt::Color::Rgb(166, 227, 161)).set_bold(true);
            file_style.set_color(env_logger::fmt::Color::Rgb(203, 166, 247)).set_bold(true);
            writeln!(
                buf,
                "[{}] [{}:{}] [{}] {}: {}",
                buf.timestamp(),
                file_style.value(record.file().unwrap_or("unknown")),
                file_style.value(record.line().unwrap_or(0)),
                style.value(record.target()),
                record.level(),
                record.args()
            )
        })
        .init();
}
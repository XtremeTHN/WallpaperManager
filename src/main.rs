use adw::{self, Application, ApplicationWindow, prelude::BoxExt, prelude::*};
use gtk4::{Grid, Image, Box as GtkBox, Label, HeaderBar, glib::{self, clone}, self};
// use std::io::Write;

use log::{info, error};

mod wallpaper_browser;
mod constants;
mod logger;

use logger::setup_logger;


fn activated(app: &Application) {
    info!("Building ui...");
    let header = HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new("Wallpaper Manager", "Designed for wayland compositors"))
        .build();

    let root = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let widgets = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .margin_bottom(20)
        .margin_end(20)
        .margin_start(20)
        .margin_top(20)
        .build();

    let wallpaper_list = Grid::new();


    root.append(&header);
    widgets.append(&Label::builder()
        .label(r#"<span size="x-large" weight="bold">Available wallpapers:</span>"#)
        .use_markup(true)
        .build()
    );
    let spin = gtk4::Spinner::new();
    root.append(&widgets);

    let window = ApplicationWindow::builder()
        .default_width(600)
        .default_height(800)
        
        .application(app)
        .content(&root)
        .build();


    window.connect_show(move |_| {
        glib::spawn_future_local(clone!(@strong spin, @strong wallpaper_list => async move {
            spin.start();
            let mut browser = wallpaper_browser::WallpaperBrowser::new();
            let res = browser.browse(constants::directories(constants::user_home()));
            let wallpapers: Vec<GtkBox> = res.iter().map(|index| {
                let ibox = GtkBox::builder()
                    .orientation(gtk4::Orientation::Vertical)
                    .build();
                ibox.append(&Image::builder()
                    .file(index)
                    .build()
                );
                ibox.append(&Label::new(Some(index)));
                ibox
            }).collect();

            println!("{:?}", wallpapers);
        }));
    });
}

fn main() {
    setup_logger();

    if let Err(err) = adw::init() {
        error!("Error while initializing adwaita: {}", err);
        panic!();
    };
    let app = Application::builder()
        .application_id("org.github.WallpaperManager")
        .build();

    app.connect_activate(activated);
    app.run();
}

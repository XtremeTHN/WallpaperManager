use std::path::PathBuf;

use adw::{self, prelude::BoxExt, prelude::*, Application, ApplicationWindow};
use gtk4::{
    self, gio,
    glib::{self, clone},
    Box as GtkBox, Grid, HeaderBar, Image, Label,
};
// use std::io::Write;

use log::{error, info, warn};

mod constants;
mod logger;
mod wallpaper_browser;

use logger::setup_logger;

fn max_pages(values: i32) -> i32 {
    values / 9
}

fn populate_grid(grid: &Grid, wallpapers: &mut Vec<GtkBox>) {
    grid.attach(&wallpapers[0], 0, 0, 30, 30);
    grid.attach(&wallpapers[1], 1, 0, 30, 30);
    grid.attach(&wallpapers[2], 2, 0, 30, 30);
    grid.attach(&wallpapers[3], 0, 1, 30, 30);
    grid.attach(&wallpapers[4], 1, 1, 30, 30);
    grid.attach(&wallpapers[5], 2, 1, 30, 30);
    grid.attach(&wallpapers[6], 0, 2, 30, 30);
    grid.attach(&wallpapers[7], 1, 2, 30, 30);
    grid.attach(&wallpapers[8], 2, 2, 30, 30);

    info!("Draining first 9 wallpapers...");
    wallpapers.drain(0..9);
}

fn activated(app: &Application) {
    info!("Building ui...");
    let header = HeaderBar::builder()
        .title_widget(&adw::WindowTitle::new(
            "Wallpaper Manager",
            "Designed for wayland compositors",
        ))
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
    widgets.append(
        &Label::builder()
            .label(r#"<span size="x-large" weight="bold">Available wallpapers:</span>"#)
            .use_markup(true)
            .build(),
    );
    let spin = gtk4::Spinner::new();
    widgets.append(&spin);
    widgets.append(&wallpaper_list);
    root.append(&widgets);

    let window = ApplicationWindow::builder()
        .default_width(600)
        .default_height(800)
        .application(app)
        .content(&root)
        .build();

    let widgets_clone = widgets.clone();
    window.connect_show(move |_| {
        glib::spawn_future_local(
            clone!(@weak spin, @strong widgets_clone, @strong wallpaper_list => async move {
                spin.start();
                info!("Spinner start");
                let mut browser = wallpaper_browser::WallpaperBrowser::new();
                let res = browser.browse(constants::directories(constants::user_home()));
                let mut wallpapers: Vec<GtkBox> = res.iter().map(|index| {
                    let ibox = GtkBox::builder()
                        .orientation(gtk4::Orientation::Vertical)
                        .build();

                    let texture = gtk4::gdk_pixbuf::Pixbuf::from_file(index);
                    let image = match texture {
                        Ok(texture) => Image::from_pixbuf(Some(&texture)),
                        Err(err) => {
                            error!("Error while loading image: {}", err);
                            warn!("Fallbacking to default image");
                            Image::new()
                        }
                    };
                    image.set_icon_size(gtk4::IconSize::Large);
                    image.set_pixel_size(200);
                    ibox.append(&image);

                    let basename = PathBuf::from(index).file_name().unwrap().to_string_lossy().to_string();
                    ibox.append(&Label::new(Some(&basename)));
                    ibox
                }).collect();
                spin.stop();
                widgets_clone.remove(&spin);
                wallpaper_list.set_row_spacing(12);
                wallpaper_list.set_column_spacing(12);
                wallpaper_list.set_row_homogeneous(true);
                wallpaper_list.set_column_homogeneous(true);
                wallpaper_list.set_margin_start(12);
                wallpaper_list.set_margin_end(12);
                wallpaper_list.set_margin_top(12);
                wallpaper_list.set_margin_bottom(12);

                info!("Populating grid...");
                populate_grid(&wallpaper_list, &mut wallpapers);
            }),
        );
    });
    window.present();
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

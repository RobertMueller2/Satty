use relm4::gtk::gio::{FileIcon, Icon};
use relm4::gtk::gio::{Notification, prelude::ApplicationExt};
use std::path::PathBuf;

use crate::configuration::APP_CONFIG;
use relm4::gtk::gdk_pixbuf::{InterpType, Pixbuf};
use relm4::gtk::prelude::Cast;
use relm4::gtk::{IconLookupFlags, IconTheme, TextDirection};
use satty_cli::command_line::NotificationIcon;

pub fn log_result(msg: &str, notify: bool) {
    eprintln!("{msg}");
    if notify && !APP_CONFIG.read().disable_notifications() {
        show_notification(msg, None);
    }
}

pub fn log_result_with_pixbuf(msg: &str, pixbuf: Pixbuf) {
    eprintln!("{msg}");

    if APP_CONFIG.read().disable_notifications() {
        return;
    }

    let notification_icon_kind = APP_CONFIG.read().notification_icon();

    let pixbuf = match notification_icon_kind {
        NotificationIcon::AppIcon => None,
        _ => {
            let src_w = pixbuf.width();
            let src_h = pixbuf.height();

            if src_w == 0 || src_h == 0 {
                None
            } else {
                let scale = f64::min(96.0 / src_w as f64, 96.0 / src_h as f64);

                let new_w = ((src_w as f64) * scale).round().max(1.0) as i32;
                let new_h = ((src_h as f64) * scale).round().max(1.0) as i32;

                pixbuf.scale_simple(new_w, new_h, InterpType::Bilinear)
            }
        }
    };

    let icon = match pixbuf {
        Some(p) if notification_icon_kind == NotificationIcon::ThumbnailIcon => {
            Some(p.upcast::<Icon>())
        }
        Some(p) if notification_icon_kind == NotificationIcon::ThumbnailFileIcon => {
            let mut path: PathBuf = std::env::temp_dir();
            path.push(format!(
                "satty-notification-thumbnail-{}.png",
                std::process::id()
            ));

            if p.savev(&path, "png", &[]).is_err() {
                None
            } else {
                let file = gio::File::for_path(&path);
                Some(FileIcon::new(&file).upcast::<Icon>())
            }
        }
        _ => None,
    };

    show_notification(msg, icon);
}

fn show_notification(msg: &str, icon: Option<Icon>) {
    // construct
    let notification = Notification::new("Satty");
    notification.set_body(Some(msg));

    if let Some(i) = icon {
        notification.set_icon(&i);
    } else {
        // lookup sattys icon
        let theme = IconTheme::default();
        if theme.has_icon("satty")
            && let Some(icon_file) = theme
                .lookup_icon(
                    "satty",
                    &[],
                    96,
                    1,
                    TextDirection::Ltr,
                    IconLookupFlags::empty(),
                )
                .file()
        {
            notification.set_icon(&FileIcon::new(&icon_file));
        }
    }

    // send notification
    relm4::main_application().send_notification(None, &notification);
}

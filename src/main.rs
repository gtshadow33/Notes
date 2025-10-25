use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button,
    FileChooserAction, FileChooserNative, Orientation,
    ResponseType, ScrolledWindow, TextView,
};
use std::fs;
use std::rc::Rc;

fn main() {
    let app = Application::builder()
        .application_id("com.example.blocnotas")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Bloc de Notas Simple")
            .default_width(600)
            .default_height(400)
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 5);
        let text_view = TextView::new();
        let text_buffer = text_view.buffer();
        let scrolled = ScrolledWindow::builder()
            .child(&text_view)
            .vexpand(true)
            .build();

        let open_button = Button::with_label("Abrir");
        let save_button = Button::with_label("Guardar");

        let hbox = GtkBox::new(Orientation::Horizontal, 5);
        hbox.append(&open_button);
        hbox.append(&save_button);

        vbox.append(&hbox);
        vbox.append(&scrolled);
        window.set_child(Some(&vbox));

        let text_buffer = Rc::new(text_buffer);

        // Botón "Abrir"
        {
            let text_buffer = text_buffer.clone();
            let window = window.clone();
            open_button.connect_clicked(move |_| {
                let dialog = Rc::new(FileChooserNative::new(
                    Some("Abrir archivo"),
                    Some(&window),
                    FileChooserAction::Open,
                    Some("Abrir"),
                    Some("Cancelar"),
                ));

                let tb = text_buffer.clone();
                dialog.connect_response(move |d, response| {
                    if response == ResponseType::Accept {
                        if let Some(file) = d.file() {
                            if let Some(path) = file.path() {
                                if let Ok(content) = fs::read_to_string(path) {
                                    tb.set_text(&content);
                                }
                            }
                        }
                    }
                });

                dialog.show();
            });
        }

        // Botón "Guardar"
        {
            let text_buffer = text_buffer.clone();
            let window = window.clone();
            save_button.connect_clicked(move |_| {
                let dialog = Rc::new(FileChooserNative::new(
                    Some("Guardar archivo"),
                    Some(&window),
                    FileChooserAction::Save,
                    Some("Guardar"),
                    Some("Cancelar"),
                ));

                let tb = text_buffer.clone();
                dialog.connect_response(move |d, response| {
                    if response == ResponseType::Accept {
                        if let Some(file) = d.file() {
                            if let Some(path) = file.path() {
                                let start = tb.start_iter();
                                let end = tb.end_iter();
                                let text = tb.text(&start, &end, false);
                                let _ = fs::write(path, text.as_str());
                            }
                        }
                    }
                   
                });

                dialog.show();
            });
        }

        window.present();
    });

    app.run();
}

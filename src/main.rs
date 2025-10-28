use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, FileChooserAction, FileChooserNative,
    Orientation, ResponseType, ScrolledWindow, TextView, Label, Dialog,
};
use std::cell::RefCell;
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
        let save_button_as = Button::with_label("Guardar como");
        let save_button = Button::with_label("Guardar");

        let hbox = GtkBox::new(Orientation::Horizontal, 5);
        hbox.append(&open_button);
        hbox.append(&save_button_as);
        hbox.append(&save_button);

        vbox.append(&hbox);
        vbox.append(&scrolled);
        window.set_child(Some(&vbox));

        // Variables compartidas
        let text_buffer = Rc::new(text_buffer);
        let current_path = Rc::new(RefCell::new(None));

        // --- Botón "Abrir" ---
        {
            let current_path = current_path.clone();
            let text_buffer = text_buffer.clone();
            let window = window.clone();

            open_button.connect_clicked(move |_| {
                let dialog = FileChooserNative::new(
                    Some("Abrir archivo"),
                    Some(&window),
                    FileChooserAction::Open,
                    Some("Abrir"),
                    Some("Cancelar"),
                );

                let tb = text_buffer.clone();
                let cp = current_path.clone();

                dialog.connect_response(move |d, response| {
                    if response == ResponseType::Accept {
                        if let Some(file) = d.file() {
                            if let Some(path) = file.path() {
                                *cp.borrow_mut() = Some(path.clone());
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

        // --- Botón "Guardar como" ---
        {
            let text_buffer = text_buffer.clone();
            let current_path = current_path.clone();
            let window = window.clone();

            save_button_as.connect_clicked(move |_| {
                let dialog = FileChooserNative::new(
                    Some("Guardar archivo"),
                    Some(&window),
                    FileChooserAction::Save,
                    Some("Guardar"),
                    Some("Cancelar"),
                );

                let tb = text_buffer.clone();
                let cp = current_path.clone();

                dialog.connect_response(move |d, response| {
                    if response == ResponseType::Accept {
                        if let Some(file) = d.file() {
                            if let Some(path) = file.path() {
                                *cp.borrow_mut() = Some(path.clone());
                                let start = tb.start_iter();
                                let end = tb.end_iter();
                                let text = tb.text(&start, &end, false);
                                if let Err(err) = fs::write(path, text.as_str()) {
                                    eprintln!("Error al guardar: {}", err);
                                }
                            }
                        }
                    }
                });

                dialog.show();
            });
        }

        // --- Botón "Guardar" ---
        {
            let current_path = current_path.clone();
            let text_buffer = text_buffer.clone();
            let window = window.clone();

            save_button.connect_clicked(move |_| {
                if let Some(ref path) = *current_path.borrow() {
                    // Guardar directamente
                    let start = text_buffer.start_iter();
                    let end = text_buffer.end_iter();
                    let text = text_buffer.text(&start, &end, false);
                    if let Err(err) = fs::write(path, text.as_str()) {
                        eprintln!("Error al guardar el archivo: {}", err);
                    }
                } else {
                    // Mostrar ventana emergente
                    let warring = Dialog::builder()
                        .transient_for(&window)
                        .modal(true)
                        .title("Sin archivo")
                        .build();

                    let content = warring.content_area();
                    let label = Label::new(Some(
                        "Usa 'Guardar como' para elegir un destino.",
                    ));
                    content.append(&label);

                    warring.add_button("Cerrar", ResponseType::Close);

                    warring.connect_response(|d, _resp| {
                        d.close();
                    });

                    dialog.present();
                }
            });
        }

        window.present();
    });

    app.run();
}

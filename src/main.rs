use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Button, Entry, Orientation};
use rand::Rng;
use std::iter;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Créer une nouvelle application
    let app = Application::builder().application_id(APP_ID).build();

    // Connecter le signal "activate" de l'application
    app.connect_activate(build_ui);

    // Exécuter l'application
    app.run()
}

fn build_ui(app: &Application) {
    // Créer un conteneur Box avec une orientation verticale
    let center = GtkBox::new(Orientation::Vertical, 5);

    let ecran = Entry::builder()
        .placeholder_text("Votre mot de passe")
        .editable(false)
        .build();
    let button = Button::builder()
        .label("Generation du mot de passe")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Ajouter ecran et button au conteneur center
    center.append(&ecran);
    center.append(&button);

    // Connecter le signal "clicked" du bouton
    button.connect_clicked(move |_| {
        let mut rng = rand::thread_rng();
        // Changer le label en "Hello World!" après le clic
        let chars: Vec<char> = ('A'..='Z').chain('a'..='z').chain('0'..='9').collect();

        // Générer une suite de 12 caractères
        let random_string: String = iter::repeat_with(|| {
            let index = rng.gen_range(0..chars.len());
            chars[index]
        })
        .take(12)
        .collect();

        // Mettre à jour le label avec la chaîne générée
        ecran.set_text(&random_string);
    });

    // Créer une fenêtre
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Générateur de mot de passe")
        .child(&center)
        .resizable(false) // Rendre la fenêtre non redimensionnable
        .default_width(400) // Largeur par défaut
        .default_height(200)
        .build();

    // Afficher la fenêtre
    window.present();
}

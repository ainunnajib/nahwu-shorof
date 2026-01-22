use std::sync::LazyLock;
use tera::Tera;

pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = match Tera::new("assets/views/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template parsing error: {}", e);
            std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html"]);
    tera
});

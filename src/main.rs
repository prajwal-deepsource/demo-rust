use actix_web_lab::web as web_lab;
use regex;

fn redir_fn(dest: String, other_dest: String) -> web_lab::Redirect {
    web_lab::redirect("Hello", dest)
}

fn redir_struct_method(dest: &str) -> web_lab::Redirect {
    web_lab::Redirect::new("Hello".into(), dest.into())
}

fn writer_check<W: std::io::Write>(w: &mut W) -> std::io::Result<()> {
    w.write(b"foo")?;
    Ok(())
}

fn random(path: String) {
    let x = 0 as *const u32;
    let y = 0 as *mut *const u32;
    let z = 0 as *mut u32;
    actix_files::NamedFile::open(path).unwrap();
    let a: f32 = 0.0;
    if a == f32::NAN {}
    if a != f32::NAN {}
}

struct A;
impl A {
    fn default() -> Self {
        Self
    }
}

fn main() {}

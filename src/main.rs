use actix_web_lab::web as web_lab;

fn main() {
    let a: web_lab::Redirect = get_redir("/yello".into());
    let b: web_lab::Redirect = get_direct_redir("/yello".into());
}

fn get_direct_redir(dest: &str) -> web_lab::Redirect {
    web_lab::Redirect = web_lab::redirect("/".into(), dest.into())
}

fn get_redir(dest: &str) -> web_lab::Redirect {
    web_lab::Redirect::new("/".into(), dest.into())
}

fn get_sanitised_redir(dest: &str) -> web_lab::Redirect {
    let san = sanitise(dest);
    web_lab::Redirect::new("/".into(), dest.into())
}

fn sanitise(s: &str) -> &str {
    s
}

use actix_web_lab::web as web_lab;

fn redir_fn(dest: String, other_dest: String) -> web_lab::Redirect {
    web_lab::redirect("Hello".into(), dest)
}

fn redir_struct_method(dest: &str) -> web_lab::Redirect {
    web_lab::Redirect::new("Hello".into(), dest.into())
}

fn redir_with_cast(dest: &str) -> web_lab::Redirect {
    web_lab::redirect("Hello".into(), dest as String)
}

fn no_match_on_field(destTuple: (String, usize)) -> web_lab::Redirect {
    // Should not raise on this call
    web_lab::redirect("Hello".into(), dest.0)
}

fn no_match_used_var(dest: String) -> Option<web_lab::Redirect> {
    if !valid(dest) {
        return None;
    };
    // Should not raise on this call
    Some(web_lab::redirect("Hello".into(), dest))
}

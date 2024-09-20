#[macro_export]
macro_rules! extend_url_params {
    ($url:ident, $($param:ident),*) => {
        $(
            if let Some($param) = $param {
                $url.push_str(&format!(concat!(stringify!($param), "={}&"), $param));
            }
        )*
    };
}

#[macro_export]
macro_rules! extend_form_text_fields {
    ($form:ident, $($field:ident),*) => {
        $(
            if let Some($field) = $field {
                $form = $form.text(stringify!($field), $field.to_string());
            }
        )*
    };
}

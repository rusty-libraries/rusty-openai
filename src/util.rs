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

#[macro_export]
macro_rules! setters {
    ($(
        $(#[$setter_attributes:meta])*
        $setter_ident:ident: $setter_type:ty,
    )*) => {
        $(
            $(#[$setter_attributes])*
            #[inline(always)]
            pub fn $setter_ident(mut self, $setter_ident: $setter_type) -> Self {
                self.$setter_ident = Some($setter_ident);
                self
            }
        )*
    };
}

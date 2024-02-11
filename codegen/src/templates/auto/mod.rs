// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

{% if enums %}
mod enums;
pub use super::enums::{
{%- for enum in enums %}
    {{enum.name}},
{%- endfor %}
};
{% endif %}

{% if flags %}
mod flags;
{% endif %}

{% if aliases %}
mod alias;
pub use super::alias::{
{%- for alias in aliases %}
    {{alias.name}},
{%- endfor %}
};
{% endif %}

{% if global_functions %}
pub(crate) mod functions;
{% endif %}

{% if constants %}
mod constants;
pub use super::constants::{
{%- for constant in constants %}
    {{constant.name}},
{%- endfor %}
};
{% endif %}

{% if traits %}
pub(crate) mod traits {

}
{% endif %}

{% if builders %}
pub(crate) mod builders {

}
{% endif %}
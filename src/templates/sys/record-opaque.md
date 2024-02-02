#[repr(C)]
pub struct {% if is_disguised %}_{{type}} {% else %}{{type}} {% endif %} {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
{% if is_disguised %}
pub type {{type}} = _{{type}};
{% else %}
impl ::std::fmt::Debug for {{type}} {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("{{type}} @ {self:p}"))
            .finish()
    }
}
{% endif %}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct {{type}} {
    {% for field in fields %}
    {{field}}
    {% endfor %}
}
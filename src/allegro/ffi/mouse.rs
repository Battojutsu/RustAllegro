use std::libc::*;
use ffi::display::ALLEGRO_DISPLAY;
use ffi::events::ALLEGRO_EVENT_SOURCE;
use rust_util::c_bool;

opaque!(ALLEGRO_MOUSE)

pub static ALLEGRO_MOUSE_MAX_EXTRA_AXES: u32 = 4;

pub struct ALLEGRO_MOUSE_STATE
{
    x: c_int,
    y: c_int,
    z: c_int,
    w: c_int,
    more_axes: [c_int, ..ALLEGRO_MOUSE_MAX_EXTRA_AXES],
    buttons: c_int,
    pressure: c_float,
    display: *mut ALLEGRO_DISPLAY,
}

extern "C"
{
	pub fn al_is_mouse_installed() -> c_bool;
	pub fn al_install_mouse() -> c_bool;
	pub fn al_uninstall_mouse();
	pub fn al_get_mouse_num_buttons() -> c_uint;
	pub fn al_get_mouse_num_axes() -> c_uint;
	pub fn al_set_mouse_xy(display: *mut ALLEGRO_DISPLAY, x: c_int, y: c_int) -> c_bool;
	pub fn al_set_mouse_z(z: c_int) -> c_bool;
	pub fn al_set_mouse_w(w: c_int) -> c_bool;
	pub fn al_set_mouse_axis(axis: c_int, value: c_int) -> c_bool;
	pub fn al_get_mouse_state(ret_state: *mut ALLEGRO_MOUSE_STATE);
	pub fn al_mouse_button_down(state: *ALLEGRO_MOUSE_STATE, button: c_int) -> c_bool;
	pub fn al_get_mouse_state_axis(state: *ALLEGRO_MOUSE_STATE, axis: c_int) -> c_int;
	pub fn al_get_mouse_cursor_position(ret_x: *mut c_int, ret_y: *mut c_int) -> c_bool;
	pub fn al_grab_mouse(display: *mut ALLEGRO_DISPLAY) -> c_bool;
	pub fn al_ungrab_mouse() -> c_bool;
	pub fn al_get_mouse_event_source() -> *mut ALLEGRO_EVENT_SOURCE;
}

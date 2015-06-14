// Copyright (c) 2014 by SiegeLord
//
// All rights reserved. Distributed under ZLib. For full terms see the file LICENSE.

#![crate_name="allegro_image"]
#![crate_type = "lib"]
#![allow(non_upper_case_globals)]

#![feature(libc)]

extern crate allegro;
#[macro_use]
extern crate allegro_util;
extern crate libc;

use std::cell::RefCell;
use std::marker::PhantomData;

use allegro::Core;
use ffi::allegro_image::*;

#[cfg(not(manual_link))]
mod link_name
{
	#[link(name = "allegro_image")]
	extern "C" {}
}

pub mod ffi
{
	pub use self::allegro_image::*;
	pub mod allegro_image
	{
		use libc::*;
		use allegro::c_bool;

		extern "C"
		{
			pub fn al_init_image_addon() -> c_bool;
			pub fn al_shutdown_image_addon();
			pub fn al_get_allegro_image_version() -> uint32_t;
		}
	}
}

static mut initialized: bool = false;
thread_local!(static spawned_on_this_thread: RefCell<bool> = RefCell::new(false));

pub struct ImageAddon
{
	no_send_marker: PhantomData<*mut u8>
}

impl ImageAddon
{
	pub fn init(core: &Core) -> Result<ImageAddon, String>
	{
		let mutex = core.get_core_mutex();
		let _guard = mutex.lock();
		unsafe
		{
			if initialized
			{
				if spawned_on_this_thread.with(|x| *x.borrow())
				{
					Err("The image addon has already been created in this task.".to_string())
				}
				else
				{
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(ImageAddon{ no_send_marker: PhantomData })
				}
			}
			else
			{
				if al_init_image_addon() != 0
				{
					initialized = true;
					spawned_on_this_thread.with(|x| *x.borrow_mut() = true);
					Ok(ImageAddon{ no_send_marker: PhantomData })
				}
				else
				{
					Err("Could not initialize the image addon.".to_string())
				}
			}
		}
	}

	pub fn get_version() -> i32
	{
		unsafe
		{
			al_get_allegro_image_version() as i32
		}
	}
}

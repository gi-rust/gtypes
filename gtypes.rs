// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013, 2014  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA


//! This crate provides definitions for types that are intrinsic in
//! GObject introspection, unlike all other types that have a definition in
//! a GIR file. All crates generated from GObject introspection data should
//! use these shared definitions, usable in FFI declarations as well as
//! in idiomatic Rust bindings.
//!
//! Some types are omitted because a suitable equivalent is readily available
//! in Rust:
//!
//! 1. Fixed-size integer types. These have straightforward built-in
//!    counterparts in Rust.
//! 2. Strings can have GI types `utf8` or `filename`. In the FFI all string
//!    values are represented as raw pointers to `gchar`. A safe
//!    representation of C strings in idiomatic Rust bindings
//!    needs some wrapper types which are not defined here.

#![allow(non_camel_case_types)]

use libc;

pub type gboolean       = libc::c_int;
pub type gchar          = libc::c_char;
pub type guchar         = libc::c_uchar;
pub type gint           = libc::c_int;
pub type guint          = libc::c_uint;
pub type glong          = libc::c_long;
pub type gulong         = libc::c_ulong;
pub type gsize          = libc::size_t;
pub type gssize         = libc::ssize_t;
pub type gfloat         = libc::c_float;
pub type gdouble        = libc::c_double;
pub type gpointer       = *mut   libc::c_void;
pub type gconstpointer  = *const libc::c_void;

pub const FALSE: gboolean = 0;
pub const TRUE : gboolean = 1;

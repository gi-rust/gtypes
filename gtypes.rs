// This file is part of Grust, GObject introspection bindings for Rust
//
// Copyright (C) 2013-2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
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
//! use these shared definitions.
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

extern crate libc;

// The fundamental types are defined in Python module giscanner.ast

pub mod primitive {

    //! Definitions for primitive fundamental types.
    //!
    //! Values of these types don't have associated semantics or state
    //! that would necessitate some kind of wrapper types in safe Rust
    //! bindings. Therefore, these types can be used in both
    //! FFI declarations and idiomatic Rust bindings, unlikely to require
    //! name disambiguation with any generated or binding types.
    //!
    //! The untyped pointer types are also defined in this module,
    //! because safe Rust bindings are unlikely to need to reuse their names.

    #![allow(non_camel_case_types)]

    use libc;

    pub type gboolean       = libc::c_int;
    pub type gchar          = libc::c_char;
    pub type guchar         = libc::c_uchar;
    pub type gshort         = libc::c_short;
    pub type gushort        = libc::c_ushort;
    pub type gint           = libc::c_int;
    pub type guint          = libc::c_uint;
    pub type glong          = libc::c_long;
    pub type gulong         = libc::c_ulong;
    pub type gsize          = libc::size_t;
    pub type gssize         = libc::ssize_t;
    pub type gintptr        = libc::intptr_t;
    pub type guintptr       = libc::uintptr_t;
    pub type gfloat         = libc::c_float;
    pub type gdouble        = libc::c_double;
    pub type gunichar       = u32;
    pub type gpointer       = *mut   libc::c_void;
    pub type gconstpointer  = *const libc::c_void;

    pub const FALSE: gboolean = 0;
    pub const TRUE : gboolean = 1;
}

// For -sys crates only dealing with FFI types, all public names defined
// in this crate should be conflict-free, so we make them available at the
// crate level handy for glob-importing.
// FIXME: Can't glob-import due to https://github.com/rust-lang/rust/issues/4865,
// which is fixed by Rust 1.4.0
pub use primitive::{gboolean, gchar, guchar, gshort, gushort, gint, guint};
pub use primitive::{glong, gulong, gsize, gssize, gintptr, guintptr};
pub use primitive::{gfloat, gdouble, gunichar, gpointer, gconstpointer};
pub use primitive::{FALSE, TRUE};

/// An integer type that designates GObject type identifiers.
///
/// `GType` is considered a fundamental type, regardless of being also
/// introspected both in GLib and GObject. Unlike the primitive types,
/// valid values of `GType` can only result from type registration,
/// so the safe bindings are likely to provide their own `GType` wrapper.
pub type GType = gsize;

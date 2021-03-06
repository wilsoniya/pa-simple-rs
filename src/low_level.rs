//! Provides low-level pulseaudio bindings and types.
//!
//! See also: http://freedesktop.org/software/pulseaudio/doxygen/simple_8h.html

// Allow zero-sized structs
#![allow(improper_ctypes)]

use libc::{c_int, c_char, size_t, c_void};

#[link(name = "pulse-simple")]
#[link(name = "pulse")]
extern {
    pub fn pa_simple_new(server: *mut c_char,
                         name: *mut c_char,
                         dir: c_int,
                         dev: *mut c_char,
                         stream_name: *mut c_char,
                         sample_spec: *mut pa_sample_spec,
                         channel_map: *mut u8,
                         attr: *mut u8,
                         error: *mut c_int) -> *mut pa_simple;
    pub fn pa_simple_free (pa: *mut pa_simple);
    pub fn pa_simple_write (pa: *mut pa_simple,
                            data: *const c_void,
                            bytes: size_t,
                            error: *mut c_int) -> c_int;
    pub fn pa_simple_drain (pa: *mut pa_simple,
                            error: *mut c_int);
    pub fn pa_simple_read(pa: *mut pa_simple,
                          data: *mut c_void,
                          num_bytes: size_t,
                          error: *mut c_int) -> c_int;
    pub fn pa_simple_get_latency (pa: *mut pa_simple,
                                  error: *mut c_int) -> u64;
    pub fn pa_strerror(error: c_int) -> *mut c_char;
    pub fn pa_simple_flush(pa: *mut pa_simple, error: *mut c_int) -> c_int;
}

/// The direction of information which will stream from the server.
///
/// See also:
/// [pa_stream_direction_t](http://freedesktop.org/software/pulseaudio/doxygen/def_8h.html#a637b1451881b8c0b0f98bafe115d7254)
/// and
/// [pa_stream_direction](http://freedesktop.org/software/pulseaudio/doxygen/def_8h.html#a7311932553b3f7962a092906576bc347).
pub enum StreamDirection {
//  /// Invalid direction.
//  NoDirection = 0,
//  /// Playback stream.
    Playback = 1,
    /// Record stream.
    Record = 2,
//  /// Sample upload stream.
//  Upload = 3,
}

/// Wire formats of individual audio samples.
///
/// See also:
/// [pa_sample_format](http://freedesktop.org/software/pulseaudio/doxygen/sample_8h.html#a3c622fc51f4fc6ebfdcc7b454ac9c05f)
pub enum SampleFormat {
    /// Unsigned 8 Bit PCM.
    U8,
    /// 8 Bit a-Law.
    ALAW,
    /// 8 Bit mu-Law.
    ULAW,
    /// Signed 16 Bit PCM, little endian (PC).
    S16LE,
//  /// Signed 16 Bit PCM, big endian.
//  S16BE,
    /// 32 Bit IEEE floating point, little endian (PC), range -1.0 to 1.0.
    FLOAT32LE,
//  /// 32 Bit IEEE floating point, big endian, range -1.0 to 1.0.
//  FLOAT32BE,
    /// Signed 32 Bit PCM, little endian (PC).
    S32LE,
//  /// Signed 32 Bit PCM, big endian.
//  S32BE,
//  /// Signed 24 Bit PCM packed, little endian (PC).
//  S24LE,
//  /// Signed 24 Bit PCM packed, big endian.
//  S24BE,
//  /// Signed 24 Bit PCM in LSB of 32 Bit words, little endian (PC).
//  S24_32LE,
//  /// Signed 24 Bit PCM in LSB of 32 Bit words, big endian.
//  S24_32BE,
//  /// Upper limit of valid sample types.
//  MAX,
//  /// An invalid value.
//  INVALID = -1,
}

// typedef struct pa_simple pa_simple
#[repr(C)]
pub struct pa_simple;

// see pulse/def.h
#[repr(C)]
pub struct pa_sample_spec {
    pub format: c_int,
    pub rate: u32,
    pub channels: u8
}

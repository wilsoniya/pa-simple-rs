//! Provides pulseaudio related stuff.

// Create a new connection to the server. More...
// pa_simple * 	pa_simple_new (
//      const char *server, const char *name, pa_stream_direction_t dir, const char *dev, const
//      char *stream_name, const pa_sample_spec *ss, const pa_channel_map *map, const
//      pa_buffer_attr *attr, int *error)
//
// Close and free the connection to the server. More...
// void 	pa_simple_free (pa_simple *s)
//
// Write some data to the server. More...
// int 	pa_simple_write (
//      pa_simple *s, const void *data, size_t bytes, int *error)
//
// Wait until all data already written is played by the daemon. More...
// int 	pa_simple_drain (pa_simple *s, int *error)
//
// Read some data from the server. More...
// int 	pa_simple_read (pa_simple *s, void *data, size_t bytes, int *error)
//
// Return the playback or record latency. More...
// pa_usec_t 	pa_simple_get_latency (pa_simple *s, int *error)
//
// Flush the playback or record buffer. More...
// int 	pa_simple_flush (pa_simple *s, int *error)

use std::ffi::{CString, CStr};
use std::marker::PhantomData;
use std::ptr;
use std::str::from_utf8;

use libc::{c_int, c_char, size_t, free, c_void};

#[link(name = "pulse-simple")]
#[link(name = "pulse")]
extern {
    fn pa_simple_new(server: *mut c_char,
                     name: *mut c_char,
                     dir: c_int,
                     dev: *mut c_char,
                     steam_name: *mut c_char,
                     sample_spec: *mut pa_sample_spec,
                     channel_map: *mut u8,
                     attr: *mut u8,
                     error: *mut c_int) -> *mut pa_simple;
    fn pa_simple_free (pa: *mut pa_simple);
    fn pa_simple_write (pa: *mut pa_simple,
                        data: *mut c_void,
                        bytes: size_t,
                        error: *mut c_int) -> c_int;
    fn pa_simple_drain (pa: *mut pa_simple,
                        error: *mut c_int);
    fn pa_simple_read(pa: *mut pa_simple,
                      data: *mut c_void,
                      num_bytes: size_t,
                      error: *mut c_int) -> c_int;
    fn pa_simple_get_latency (pa: *mut pa_simple,
                              error: *mut c_int) -> u64;
    fn pa_strerror(error: c_int) -> *mut c_char;
    fn pa_simple_flush(pa: *mut pa_simple, error: *mut c_int) -> c_int;
}

// see pa_sample_format
static PA_SAMPLE_S16LE: c_int = 3_i32;

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
    /// Sample upload stream.
    Upload = 3,
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
    /// Signed 16 Bit PCM, big endian.
    S16BE,
    /// 32 Bit IEEE floating point, little endian (PC), range -1.0 to 1.0.
    FLOAT32LE,
    /// 32 Bit IEEE floating point, big endian, range -1.0 to 1.0.
    FLOAT32BE,
    /// Signed 32 Bit PCM, little endian (PC).
    S32LE,
    /// Signed 32 Bit PCM, big endian.
    S32BE,
    /// Signed 24 Bit PCM packed, little endian (PC).
    S24LE,
    /// Signed 24 Bit PCM packed, big endian.
    S24BE,
    /// Signed 24 Bit PCM in LSB of 32 Bit words, little endian (PC).
    S24_32LE,
    /// Signed 24 Bit PCM in LSB of 32 Bit words, big endian.
    S24_32BE,
//  /// Upper limit of valid sample types.
//  MAX,
//  /// An invalid value.
//  INVALID = -1,
}

// typedef struct pa_simple pa_simple
#[repr(C)]
struct pa_simple;

// see pulse/def.h
#[repr(C)]
struct pa_sample_spec {
    format: c_int,
    rate: u32,
    channels: u8
}

/// pulseaudio manager type.
pub struct PulseAudio<T> {
    ptr: *mut pa_simple,
    sample_rate: usize,
    phantom: PhantomData<T>,
}

impl<T> PulseAudio<T> {
    /// Constructs a new `PulseAudio`.
    pub fn new(pa_name: &str, stream_name: &str,
               stream_direction: StreamDirection,
               sample_rate: usize) -> PulseAudio<T> {
        let mut err: c_int = 0;

        let mut s_spec = pa_sample_spec{
            format: SampleFormat::S16LE as c_int,
            rate: sample_rate as u32,
            channels: 1};

        unsafe {
            let pa = pa_simple_new(
                ptr::null_mut::<i8>() as *mut i8,
                CString::new(pa_name).unwrap().as_ptr() as *mut i8,
                stream_direction as c_int,
                ptr::null_mut::<i8>() as *mut i8,
                CString::new(stream_name).unwrap().as_ptr() as *mut i8,
                &mut s_spec,
                ptr::null_mut::<u8>() as *mut u8,
                ptr::null_mut::<u8>() as *mut u8,
                &mut err);
            PulseAudio::<T>::handle_error(err);

            PulseAudio { ptr: pa, sample_rate: sample_rate,
                phantom: PhantomData }
        }
    }

    unsafe fn handle_error(err_code: c_int) {
        if err_code != 0 {
            let err_msg = CStr::from_ptr(pa_strerror(err_code));
            let err_msg: &str = from_utf8(err_msg.to_bytes()).unwrap();
            panic!("err code {} from pulse: \"{}\"", err_code, err_msg);
        }
    }
}

impl<T> Drop for PulseAudio<T> {
    fn drop(&mut self) {
        unsafe {
            ptr::read(self.ptr);
            free(self.ptr as *mut c_void);
        }
    }
}

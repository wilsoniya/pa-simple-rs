//! Provides pulseaudio related stuff.
//!
//! See also: http://freedesktop.org/software/pulseaudio/doxygen/simple_8h.html


// Allow zero-sized structs
#![allow(improper_ctypes)]

use std::ffi::{CString, CStr};
use std::ptr;
use std::str::from_utf8;
use std::marker::PhantomData;

use libc::{c_int, c_char, size_t, c_void};

#[link(name = "pulse-simple")]
#[link(name = "pulse")]
extern {
    fn pa_simple_new(server: *mut c_char,
                     name: *mut c_char,
                     dir: c_int,
                     dev: *mut c_char,
                     stream_name: *mut c_char,
                     sample_spec: *mut pa_sample_spec,
                     channel_map: *mut u8,
                     attr: *mut u8,
                     error: *mut c_int) -> *mut pa_simple;
    fn pa_simple_free (pa: *mut pa_simple);
    fn pa_simple_write (pa: *mut pa_simple,
                        data: *const c_void,
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

/// The direction of information which will stream from the server.
///
/// See also:
/// [pa_stream_direction_t](http://freedesktop.org/software/pulseaudio/doxygen/def_8h.html#a637b1451881b8c0b0f98bafe115d7254)
/// and
/// [pa_stream_direction](http://freedesktop.org/software/pulseaudio/doxygen/def_8h.html#a7311932553b3f7962a092906576bc347).
enum StreamDirection {
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
enum SampleFormat {
    /// Unsigned 8 Bit PCM.
    U8,
//  /// 8 Bit a-Law.
//  ALAW,
//  /// 8 Bit mu-Law.
//  ULAW,
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
struct pa_simple;

// see pulse/def.h
#[repr(C)]
struct pa_sample_spec {
    format: c_int,
    rate: u32,
    channels: u8
}

unsafe fn handle_error(err_code: c_int) {
    if err_code != 0 {
        let err_msg = CStr::from_ptr(pa_strerror(err_code));
        let err_msg: &str = from_utf8(err_msg.to_bytes()).unwrap();
        panic!("err code {} from pulse: \"{}\"", err_code, err_msg);
    }
}

/// Manager type for constructing objects that can either read or write samples
/// to pulseaudio.
pub struct Builder {
//  server: *mut c_char,
    /// descriptive name for client
    name: String,
//  dir: c_int,
//  dev: *mut c_char,
    /// descriptive name for a stream (e.g., song title)
    stream_name: String,
    sample_spec: pa_sample_spec,
//  channel_map: *mut u8,
//  attr: *mut u8,
//  error: *mut c_int
}

impl Builder {
    /// Constructs a new Builder.
    pub fn new(name: String, stream_name: String) -> Builder {
        let sample_spec = pa_sample_spec {
            format: SampleFormat::S16LE as i32,
            rate: 44100,
            channels: 1
        };
        Builder {
            name: name,
            stream_name: stream_name,
            sample_spec: sample_spec,
        }
    }

    /// Sets the sample rate in Hz.
    pub fn rate(mut self, rate: u32) -> Builder {
        self.sample_spec.rate = rate;
        self
    }

    /// Sets the number of channels.
    pub fn channels(mut self, channels: u8) -> Builder {
        self.sample_spec.channels = channels;
        self
    }

    /// Builds a Reader.
    fn reader<T>(&mut self, field_size: u8) -> Reader<T> {
        let mut err: c_int = 0;

        unsafe {
            let pa = pa_simple_new(
                ptr::null_mut::<i8>() as *mut i8,
                CString::new(self.name.clone()).unwrap().as_ptr() as *mut i8,
                StreamDirection::Record as c_int,
                ptr::null_mut::<i8>() as *mut i8,
                CString::new(self.stream_name.clone()).unwrap().as_ptr() as *mut i8,
                &mut self.sample_spec,
                ptr::null_mut::<u8>() as *mut u8,
                ptr::null_mut::<u8>() as *mut u8,
                &mut err);
            handle_error(err);

            Reader { ptr: pa, field_size: field_size, phantom: PhantomData }
        }
    }

    /// Builds a Writer.
    fn writer<T>(&mut self, field_size: u8) -> Writer<T> {
        let mut err: c_int = 0;

        unsafe {
            let pa = pa_simple_new(
                ptr::null_mut::<i8>() as *mut i8,
                CString::new(self.name.clone()).unwrap().as_ptr() as *mut i8,
                StreamDirection::Playback as c_int,
                ptr::null_mut::<i8>() as *mut i8,
                CString::new(self.stream_name.clone()).unwrap().as_ptr() as *mut i8,
                &mut self.sample_spec,
                ptr::null_mut::<u8>() as *mut u8,
                ptr::null_mut::<u8>() as *mut u8,
                &mut err);
            handle_error(err);

            Writer { ptr: pa, field_size: field_size, phantom: PhantomData }
        }
    }

    /// Builds a Reader that returns 8 bit PCM
    pub fn reader_u8(&mut self) -> Reader<u8> {
        self.sample_spec.format = SampleFormat::U8 as i32;
        self.reader(1)
    }
    /// Builds a Reader that returns 16 bit signed PCM
    pub fn reader_i16(&mut self) -> Reader<i16> {
        self.sample_spec.format = SampleFormat::S16LE as i32;
        self.reader(2)
    }
    /// Builds a Reader that returns 32 bit signed PCM
    pub fn reader_i32(&mut self) -> Reader<i32> {
        self.sample_spec.format = SampleFormat::S32LE as i32;
        self.reader(4)
    }
    /// Builds a Reader that returns 32 bit floating point samples in the range
    /// `[-1.0, 1.0]`
    pub fn reader_f32(&mut self) -> Reader<f32> {
        self.sample_spec.format = SampleFormat::FLOAT32LE as i32;
        self.reader(4)
    }
    /// Builds a writer that returns 8 bit PCM
    pub fn writer_u8(&mut self) -> Writer<u8> {
        self.sample_spec.format = SampleFormat::U8 as i32;
        self.writer(1)
    }
    /// Builds a Writer that returns 16 bit signed PCM
    pub fn writer_i16(&mut self) -> Writer<i16> {
        self.sample_spec.format = SampleFormat::S16LE as i32;
        self.writer(2)
    }
    /// Builds a Writer that returns 32 bit signed PCM
    pub fn writer_i32(&mut self) -> Writer<i32> {
        self.sample_spec.format = SampleFormat::S32LE as i32;
        self.writer(4)
    }
    /// Builds a Writer that returns 32 bit floating point samples in the range
    /// `[-1.0, 1.0]`
    pub fn writer_f32(&mut self) -> Writer<f32> {
        self.sample_spec.format = SampleFormat::FLOAT32LE as i32;
        self.writer(4)
    }
}

/// Reader of audio samples from a pulseaudio source.
pub struct Reader<T> {
    ptr: *mut pa_simple,
    /// size of underlying sample type in bytes
    field_size: u8,
    phantom: PhantomData<T>,
}

impl<T> Reader<T> {
    /// Reads samples into buffer.
    pub fn read(&mut self, buf: &mut [T]) {
        let mut err: c_int = 0;
        unsafe {
            pa_simple_read(self.ptr, buf.as_mut_ptr() as *mut c_void,
                           self.field_size as size_t, &mut err);
            handle_error(err);
        }
    }

    /// Gets the record latency in μsecs.
    pub fn get_latency(&mut self) -> u64 {
        let mut err: c_int = 0;
        let ret;
        unsafe {
            ret = pa_simple_get_latency(self.ptr, &mut err);
            handle_error(err);
        }
        ret
    }

    /// Flushes the record buffer.
    pub fn flush(&mut self) -> i64 {
        let mut err: c_int = 0;
        let ret;
        unsafe {
            ret = pa_simple_flush(self.ptr, &mut err);
            handle_error(err);
        }
        ret as i64
    }
}

impl<T> Drop for Reader<T> {
    fn drop(&mut self) {
        unsafe {
            pa_simple_free(self.ptr);
        }
    }
}

/// Writer of audio samples to a pulseaudio sink.
pub struct Writer<T> {
    ptr: *mut pa_simple,
    /// size of underlying sample type in bytes
    field_size: u8,
    phantom: PhantomData<T>,
}

impl<T> Writer<T> {
    /// Writes samples from buffer to pulseaudio.
    pub fn write(&mut self, buf: &[T]) {
        let mut err: c_int = 0;
        unsafe {
            pa_simple_write(self.ptr, buf.as_ptr() as *const c_void,
                            self.field_size as size_t, &mut err);
            handle_error(err);
        }
    }

    /// Gets the playback latency in μsecs.
    pub fn get_latency(&mut self) -> u64 {
        let mut err: c_int = 0;
        let ret;
        unsafe {
            ret = pa_simple_get_latency(self.ptr, &mut err);
            handle_error(err);
        }
        ret
    }

    /// Wait until all data already written is played by the daemon.
    pub fn drain(&mut self) {
        let mut err: c_int = 0;
        unsafe {
            pa_simple_drain(self.ptr, &mut err);
            handle_error(err);
        }
    }

    /// Flushes the playback buffer.
    pub fn flush(&mut self) -> i64 {
        let mut err: c_int = 0;
        let ret;
        unsafe {
            ret = pa_simple_flush(self.ptr, &mut err);
            handle_error(err);
        }
        ret as i64
    }
}

impl<T> Drop for Writer<T> {
    fn drop(&mut self) {
        unsafe {
            pa_simple_free(self.ptr);
        }
    }
}

// Modul: streamer/ladspa_ffi — LADSPA C-FFI Definitionen
//
// Raw LADSPA C-Structs und Typen basierend auf ladspa.h
#![allow(non_camel_case_types, dead_code)]

use std::os::raw::{c_char, c_float, c_int, c_ulong, c_void};

/// LADSPA Port Types
pub const LADSPA_PORT_INPUT: c_int = 0x1;
pub const LADSPA_PORT_OUTPUT: c_int = 0x2;
pub const LADSPA_PORT_CONTROL: c_int = 0x4;
pub const LADSPA_PORT_AUDIO: c_int = 0x8;

/// LADSPA Plugin Handle (opaque pointer)
pub type LADSPA_Handle = *mut c_void;

/// LADSPA Port Descriptor
pub type LADSPA_PortDescriptor = c_int;

/// LADSPA Data Type
pub type LADSPA_Data = c_float;

/// LADSPA Port Range Hint Descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LADSPA_PortRangeHintDescriptor(pub c_int);

/// LADSPA Port Range Hint
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LADSPA_PortRangeHint {
    pub HintDescriptor: LADSPA_PortRangeHintDescriptor,
    pub LowerBound: LADSPA_Data,
    pub UpperBound: LADSPA_Data,
}

/// LADSPA Plugin Descriptor (Main Structure)
#[repr(C)]
pub struct LADSPA_Descriptor {
    /// Unique Plugin ID
    pub UniqueID: c_ulong,

    /// Plugin Label (identifier string)
    pub Label: *const c_char,

    /// Plugin Properties (bitfield)
    pub Properties: c_int,

    /// Plugin Name (human-readable)
    pub Name: *const c_char,

    /// Plugin Maker/Author
    pub Maker: *const c_char,

    /// Copyright/License
    pub Copyright: *const c_char,

    /// Number of Ports
    pub PortCount: c_ulong,

    /// Port Descriptors (array)
    pub PortDescriptors: *const LADSPA_PortDescriptor,

    /// Port Names (array of strings)
    pub PortNames: *const *const c_char,

    /// Port Range Hints (array)
    pub PortRangeHints: *const LADSPA_PortRangeHint,

    /// Implementation Data (opaque)
    pub ImplementationData: *mut c_void,

    /// Instantiate Function
    pub instantiate: Option<
        unsafe extern "C" fn(
            Descriptor: *const LADSPA_Descriptor,
            SampleRate: c_ulong,
        ) -> LADSPA_Handle,
    >,

    /// Connect Port Function
    pub connect_port: Option<
        unsafe extern "C" fn(
            Instance: LADSPA_Handle,
            Port: c_ulong,
            DataLocation: *mut LADSPA_Data,
        ),
    >,

    /// Activate Function (optional)
    pub activate: Option<unsafe extern "C" fn(Instance: LADSPA_Handle)>,

    /// Run Function (main processing)
    pub run: Option<unsafe extern "C" fn(Instance: LADSPA_Handle, SampleCount: c_ulong)>,

    /// Run Adding Function (optional)
    pub run_adding:
        Option<unsafe extern "C" fn(Instance: LADSPA_Handle, SampleCount: c_ulong)>,

    /// Set Run Adding Gain Function (optional)
    pub set_run_adding_gain: Option<unsafe extern "C" fn(Instance: LADSPA_Handle, Gain: LADSPA_Data)>,

    /// Deactivate Function (optional)
    pub deactivate: Option<unsafe extern "C" fn(Instance: LADSPA_Handle)>,

    /// Cleanup Function
    pub cleanup: Option<unsafe extern "C" fn(Instance: LADSPA_Handle)>,
}

/// LADSPA Descriptor Function Type (exported by plugins)
pub type LADSPA_Descriptor_Function =
    unsafe extern "C" fn(Index: c_ulong) -> *const LADSPA_Descriptor;

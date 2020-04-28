#![allow(dead_code, non_camel_case_types, clippy::identity_op)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::os::raw::{c_char, c_int, c_void, c_uint};

pub const XKB_MOD_NAME_SHIFT   : &[u8]  = b"Shift\0";
pub const XKB_MOD_NAME_CAPS    : &[u8]  = b"Lock\0";
pub const XKB_MOD_NAME_CTRL    : &[u8]  = b"Control\0";
pub const XKB_MOD_NAME_ALT     : &[u8]  = b"Mod1\0";
pub const XKB_MOD_NAME_NUM     : &[u8]  = b"Mod2\0";
pub const XKB_MOD_NAME_LOGO    : &[u8]  = b"Mod4\0";

pub const XKB_LED_NAME_CAPS    : &[u8]  = b"Caps Lock\0";
pub const XKB_LED_NAME_NUM     : &[u8]  = b"Num Lock\0";
pub const XKB_LED_NAME_SCROLL  : &[u8]  = b"Scroll Lock\0";

pub struct xkb_context;
pub struct xkb_keymap;
pub struct xkb_state;
pub struct xkb_compose_table;
pub struct xkb_compose_state;

pub type xkb_keycode_t = u32;
pub type xkb_keysym_t = u32;
pub type xkb_layout_index_t = u32;
pub type xkb_layout_mask_t = u32;
pub type xkb_level_index_t = u32;
pub type xkb_mod_index_t = u32;
pub type xkb_mod_mask_t = u32;
pub type xkb_led_index_t = u32;
pub type xkb_led_mask_t = u32;

pub const XKB_KEYCODE_INVALID :u32 = 0xffff_ffff;
pub const XKB_LAYOUT_INVALID  :u32 = 0xffff_ffff;
pub const XKB_LEVEL_INVALID   :u32 = 0xffff_ffff;
pub const XKB_MOD_INVALID     :u32 = 0xffff_ffff;
pub const XKB_LED_INVALID     :u32 = 0xffff_ffff;
pub const XKB_KEYCODE_MAX     :u32 = 0xffff_fffe;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xkb_rule_names {
    pub rules:   *const c_char,
    pub model:   *const c_char ,
    pub layout:  *const c_char,
    pub variant: *const c_char,
    pub options: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_keysym_flags {
    /** Do not apply any flags. */
    XKB_KEYSYM_NO_FLAGS = 0,
    /** Find keysym by case-insensitive search. */
    XKB_KEYSYM_CASE_INSENSITIVE = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_context_flags {
    /** Do not apply any context flags. */
    XKB_CONTEXT_NO_FLAGS = 0,
    /** Create this context with an empty include path. */
    XKB_CONTEXT_NO_DEFAULT_INCLUDES = 1 << 0,
    /**
     * Don't take RMLVO names from the environment.
     * @since 0.3.0
     */
    XKB_CONTEXT_NO_ENVIRONMENT_NAMES = 1 << 1
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_log_level {
    /** Log critical internal errors only. */
    XKB_LOG_LEVEL_CRITICAL = 10,
    /** Log all errors. */
    XKB_LOG_LEVEL_ERROR = 20,
    /** Log warnings and errors. */
    XKB_LOG_LEVEL_WARNING = 30,
    /** Log information, warnings, and errors. */
    XKB_LOG_LEVEL_INFO = 40,
    /** Log everything. */
    XKB_LOG_LEVEL_DEBUG = 50
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_keymap_compile_flags {
    /** Do not apply any flags. */
    XKB_KEYMAP_COMPILE_NO_FLAGS = 0,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_keymap_format {
    /** Cannot be used for creation */
    XKB_KEYMAP_USE_ORIGINAL_FORMAT = 0,
    /** The current/classic XKB text format, as generated by xkbcomp -xkb. */
    XKB_KEYMAP_FORMAT_TEXT_V1 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_key_direction {
    /** The key was released. */
    XKB_KEY_UP,
    /** The key was pressed. */
    XKB_KEY_DOWN
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_compose_compile_flags {
    XKB_COMPOSE_COMPILE_NO_FLAGS = 0
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_compose_format {
    XKB_COMPOSE_FORMAT_TEXT_V1 = 1
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_compose_state_flags {
    XKB_COMPOSE_STATE_NO_FLAGS = 0
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_compose_status {
    XKB_COMPOSE_NOTHING,
    XKB_COMPOSE_COMPOSING,
    XKB_COMPOSE_COMPOSED,
    XKB_COMPOSE_CANCELLED
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum xkb_compose_feed_result {
    XKB_COMPOSE_FEED_IGNORED,
    XKB_COMPOSE_FEED_ACCEPTED
}

bitflags::bitflags!(
    pub struct xkb_state_component: u32 {
        /** Depressed modifiers, i.e. a key is physically holding them. */
        const XKB_STATE_MODS_DEPRESSED = (1 << 0);
        /** Latched modifiers, i.e. will be unset after the next non-modifier
         *  key press. */
        const XKB_STATE_MODS_LATCHED = (1 << 1);
        /** Locked modifiers, i.e. will be unset after the key provoking the
         *  lock has been pressed again. */
        const XKB_STATE_MODS_LOCKED = (1 << 2);
        /** Effective modifiers, i.e. currently active and affect key
         *  processing (derived from the other state components).
         *  Use this unless you explictly care how the state came about. */
        const XKB_STATE_MODS_EFFECTIVE = (1 << 3);
        /** Depressed layout, i.e. a key is physically holding it. */
        const XKB_STATE_LAYOUT_DEPRESSED = (1 << 4);
        /** Latched layout, i.e. will be unset after the next non-modifier
         *  key press. */
        const XKB_STATE_LAYOUT_LATCHED = (1 << 5);
        /** Locked layout, i.e. will be unset after the key provoking the lock
         *  has been pressed again. */
        const XKB_STATE_LAYOUT_LOCKED = (1 << 6);
        /** Effective layout, i.e. currently active and affects key processing
         *  (derived from the other state components).
         *  Use this unless you explictly care how the state came about. */
        const XKB_STATE_LAYOUT_EFFECTIVE = (1 << 7);
        /** LEDs (derived from the other state components). */
        const XKB_STATE_LEDS = (1 << 8);
    }
);

dlopen_external_library!(XkbCommon,
functions:
    fn xkb_keysym_get_name(xkb_keysym_t, *mut c_char, usize) -> c_int,
    fn xkb_keysym_from_name(*const c_char, xkb_keysym_flags) -> xkb_keysym_t,
    fn xkb_keysym_to_utf8(xkb_keysym_t, *mut c_char, usize) -> c_int,
    fn xkb_keysym_to_utf32(xkb_keysym_t) -> u32,
    fn xkb_context_new(xkb_context_flags) -> *mut xkb_context,
    fn xkb_context_ref(*mut xkb_context) -> *mut xkb_context,
    fn xkb_context_unref(*mut xkb_context) -> (),
    fn xkb_context_set_user_data(*mut xkb_context, *mut c_void) -> (),
    fn xkb_context_get_user_data(*mut xkb_context) -> *mut c_void,
    fn xkb_context_include_path_append(*mut xkb_context, *const c_char) -> c_int,
    fn xkb_context_include_path_append_default(*mut xkb_context) -> c_int,
    fn xkb_context_include_path_reset_defaults(*mut xkb_context) -> c_int,
    fn xkb_context_include_path_clear(*mut xkb_context) -> (),
    fn xkb_context_num_include_paths(*mut xkb_context) -> c_uint,
    fn xkb_context_include_path_get(*mut xkb_context, c_uint) -> *const c_char,
    fn xkb_context_set_log_level(*mut xkb_context, xkb_log_level) -> (),
    fn xkb_context_get_log_level(*mut xkb_context) -> xkb_log_level,
    fn xkb_context_set_log_verbosity(*mut xkb_context, c_int) -> (),
    fn xkb_context_get_log_verbosity(*mut xkb_context) -> c_int,
    fn xkb_keymap_new_from_names(*mut xkb_context,
                                 *const xkb_rule_names,
                                 xkb_keymap_compile_flags
                                ) -> *mut xkb_keymap,
    fn xkb_keymap_new_from_string(*mut xkb_context,
                                  *const c_char,
                                  xkb_keymap_format,
                                  xkb_keymap_compile_flags
                                 ) -> *mut xkb_keymap,
    fn xkb_keymap_new_from_buffer(*mut xkb_context,
                                  *const c_char,
                                  usize,
                                  xkb_keymap_format,
                                  xkb_keymap_compile_flags
                                 ) -> *mut xkb_keymap,
    fn xkb_keymap_ref(*mut xkb_keymap) -> *mut xkb_keymap,
    fn xkb_keymap_unref(*mut xkb_keymap) -> (),
    fn xkb_keymap_get_as_string(*mut xkb_keymap, xkb_keymap_format) -> *const c_char,
    fn xkb_keymap_key_repeats(*mut xkb_keymap, xkb_keycode_t) -> c_int,

    fn xkb_state_new(*mut xkb_keymap) -> *mut xkb_state,
    fn xkb_state_ref(*mut xkb_state) -> *mut xkb_state,
    fn xkb_state_unref(*mut xkb_state) -> (),
    fn xkb_state_update_mask(*mut xkb_state,
                             xkb_mod_mask_t,
                             xkb_mod_mask_t,
                             xkb_mod_mask_t,
                             xkb_layout_index_t,
                             xkb_layout_index_t,
                             xkb_layout_index_t
                            ) -> xkb_state_component,
    fn xkb_state_update_key(*mut xkb_state,
                            xkb_keycode_t,
                            xkb_key_direction
                           ) -> xkb_state_component,
    fn xkb_state_key_get_syms(*mut xkb_state,
                              xkb_keycode_t,
                              *const *mut xkb_keysym_t
                             ) -> c_int,
    fn xkb_state_key_get_utf8(*mut xkb_state,
                              xkb_keycode_t,
                              *mut c_char,
                              usize
                             ) -> c_int,
    fn xkb_state_key_get_utf32(*mut xkb_state, xkb_keycode_t) -> u32,
    fn xkb_state_key_get_one_sym(*mut xkb_state, xkb_keycode_t) -> xkb_keysym_t,
    fn xkb_state_mod_name_is_active(*mut xkb_state, *const c_char, xkb_state_component) -> c_int,
    fn xkb_compose_table_new_from_locale(*mut xkb_context, *const c_char, xkb_compose_compile_flags) -> *mut xkb_compose_table,
    fn xkb_compose_table_unref(*mut xkb_compose_table) -> (),
    fn xkb_compose_state_new(*mut xkb_compose_table, xkb_compose_state_flags) -> *mut xkb_compose_state,
    fn xkb_compose_state_unref(*mut xkb_compose_state) -> (),
    fn xkb_compose_state_feed(*mut xkb_compose_state, xkb_keysym_t) -> xkb_compose_feed_result,
    fn xkb_compose_state_reset(*mut xkb_compose_state) -> (),
    fn xkb_compose_state_get_status(*mut xkb_compose_state) -> xkb_compose_status,
    fn xkb_compose_state_get_utf8(*mut xkb_compose_state, *mut c_char, usize) -> c_int,
    fn xkb_compose_state_get_one_sym(*mut xkb_compose_state) -> xkb_keysym_t,
);

lazy_static::lazy_static!(
    pub static ref XKBCOMMON_OPTION: Option<XkbCommon> = {
        XkbCommon::open("libxkbcommon.so.0")
            .or_else(|_| XkbCommon::open("libxkbcommon.so"))
            .ok()
    };
    pub static ref XKBCOMMON_HANDLE: &'static XkbCommon = {
        XKBCOMMON_OPTION.as_ref().expect("Library libxkbcommon.so could not be loaded.")
    };
);

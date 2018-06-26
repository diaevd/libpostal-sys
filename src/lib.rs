
// extern crate libc;

use ::std::os::raw::c_char;

//
// Address dictionaries
//
// Bit set, should be able to keep it at a short (uint16_t)
// #define LIBPOSTAL_ADDRESS_NONE 0
// #define LIBPOSTAL_ADDRESS_ANY (1 << 0)
// #define LIBPOSTAL_ADDRESS_NAME (1 << 1)
// #define LIBPOSTAL_ADDRESS_HOUSE_NUMBER (1 << 2)
// #define LIBPOSTAL_ADDRESS_STREET (1 << 3)
// #define LIBPOSTAL_ADDRESS_UNIT (1 << 4)
// #define LIBPOSTAL_ADDRESS_LEVEL (1 << 5)
// #define LIBPOSTAL_ADDRESS_STAIRCASE (1 << 6)
// #define LIBPOSTAL_ADDRESS_ENTRANCE (1 << 7)

// #define LIBPOSTAL_ADDRESS_CATEGORY (1 << 8)
// #define LIBPOSTAL_ADDRESS_NEAR (1 << 9)

// #define LIBPOSTAL_ADDRESS_TOPONYM (1 << 13)
// #define LIBPOSTAL_ADDRESS_POSTAL_CODE (1 << 14)
// #define LIBPOSTAL_ADDRESS_PO_BOX (1 << 15)
// #define LIBPOSTAL_ADDRESS_ALL ((1 << 16) - 1)

pub const LIBPOSTAL_ADDRESS_NONE: u16 = 0;
pub const LIBPOSTAL_ADDRESS_ANY: u16 = (1 << 0);
pub const LIBPOSTAL_ADDRESS_NAME: u16 = (1 << 1);
pub const LIBPOSTAL_ADDRESS_HOUSE_NUMBER: u16 = (1 << 2);
pub const LIBPOSTAL_ADDRESS_STREET: u16 = (1 << 3);
pub const LIBPOSTAL_ADDRESS_UNIT: u16 = (1 << 4);
pub const LIBPOSTAL_ADDRESS_LEVEL: u16 = (1 << 5);
pub const LIBPOSTAL_ADDRESS_STAIRCASE: u16 = (1 << 6);
pub const LIBPOSTAL_ADDRESS_ENTRANCE: u16 = (1 << 7);

pub const LIBPOSTAL_ADDRESS_CATEGORY: u16 = (1 << 8);
pub const LIBPOSTAL_ADDRESS_NEAR: u16 = (1 << 9);

pub const LIBPOSTAL_ADDRESS_TOPONYM: u16 = (1 << 13);
pub const LIBPOSTAL_ADDRESS_POSTAL_CODE: u16 = (1 << 14);
pub const LIBPOSTAL_ADDRESS_PO_BOX: u16 = (1 << 15);
pub const LIBPOSTAL_ADDRESS_ALL: u16 = 65535;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libpostal_normalize_options {
    pub languages: *mut *mut c_char,
    pub num_languages: usize,
    pub address_components: u16,
    pub latin_ascii: bool,
    pub transliterate: bool,
    pub strip_accents: bool,
    pub decompose: bool,
    pub lowercase: bool,
    pub trim_string: bool,
    pub drop_parentheticals: bool,
    pub replace_numeric_hyphens: bool,
    pub delete_numeric_hyphens: bool,
    pub split_alpha_from_numeric: bool,
    pub replace_word_hyphens: bool,
    pub delete_word_hyphens: bool,
    pub delete_final_periods: bool,
    pub delete_acronym_periods: bool,
    pub drop_english_possessives: bool,
    pub delete_apostrophes: bool,
    pub expand_numex: bool,
    pub roman_numerals: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libpostal_address_parser_options {
    pub language: *const c_char,
    pub country: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libpostal_address_parser_response {
    pub num_components: usize,
    pub components: *const *const c_char,
    pub labels: *const *const c_char,
}

#[link(name="postal", kind="dylib")]
extern "C" {
    // engine setup / teardown
    pub fn libpostal_setup() -> bool;
    pub fn libpostal_setup_datadir(datadir: *const c_char) -> bool;
    pub fn libpostal_teardown();

    // address expansion
    pub fn libpostal_setup_language_classifier() -> bool;
    pub fn libpostal_setup_language_classifier_datadir(datadir: *const c_char) -> bool;
    pub fn libpostal_teardown_language_classifier();

    pub fn libpostal_get_default_options() -> libpostal_normalize_options;

    pub fn libpostal_expand_address(
        input: *const c_char,
        options: libpostal_normalize_options,
        n: *mut usize,
    ) -> *const *const c_char;

    pub fn libpostal_expand_address_root(
        input: *const c_char,
        options: libpostal_normalize_options,
        n: *mut usize,
    ) -> *const *const c_char;

    pub fn libpostal_expansion_array_destroy(expansions: *const *const c_char, n: usize);

    // address parser
    pub fn libpostal_setup_parser() -> bool;
    pub fn libpostal_setup_parser_datadir(datadir: *const c_char) -> bool;
    pub fn libpostal_teardown_parser();

    pub fn libpostal_get_address_parser_default_options() -> libpostal_address_parser_options;

    pub fn libpostal_parse_address(
        address: *const c_char,
        options: libpostal_address_parser_options,
    ) -> *const libpostal_address_parser_response;

    pub fn libpostal_address_parser_response_destroy(
        parsed: *const libpostal_address_parser_response,
    );

    // enable/disable debug output for parser
    pub fn libpostal_parser_print_features(print_features: bool) -> bool;
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_layout_libpostal_normalize_options() {
        assert_eq!(
            ::std::mem::size_of::<libpostal_normalize_options>(),
            40usize,
            concat!("Size of: ", stringify!(libpostal_normalize_options))
        );
        assert_eq!(
            ::std::mem::align_of::<libpostal_normalize_options>(),
            8usize,
            concat!("Alignment of ", stringify!(libpostal_normalize_options))
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).languages as *const _ as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(languages)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).num_languages as *const _
                    as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(num_languages)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).address_components as *const _
                    as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(address_components)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).latin_ascii as *const _ as usize
            },
            18usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(latin_ascii)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).transliterate as *const _
                    as usize
            },
            19usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(transliterate)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).strip_accents as *const _
                    as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(strip_accents)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).decompose as *const _ as usize
            },
            21usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(decompose)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).lowercase as *const _ as usize
            },
            22usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(lowercase)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).trim_string as *const _ as usize
            },
            23usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(trim_string)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).drop_parentheticals as *const _
                    as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(drop_parentheticals)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).replace_numeric_hyphens
                    as *const _ as usize
            },
            25usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(replace_numeric_hyphens)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).delete_numeric_hyphens
                    as *const _ as usize
            },
            26usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(delete_numeric_hyphens)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).split_alpha_from_numeric
                    as *const _ as usize
            },
            27usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(split_alpha_from_numeric)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).replace_word_hyphens as *const _
                    as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(replace_word_hyphens)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).delete_word_hyphens as *const _
                    as usize
            },
            29usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(delete_word_hyphens)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).delete_final_periods as *const _
                    as usize
            },
            30usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(delete_final_periods)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).delete_acronym_periods
                    as *const _ as usize
            },
            31usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(delete_acronym_periods)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).drop_english_possessives
                    as *const _ as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(drop_english_possessives)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).delete_apostrophes as *const _
                    as usize
            },
            33usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(delete_apostrophes)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).expand_numex as *const _
                    as usize
            },
            34usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(expand_numex)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_normalize_options>())).roman_numerals as *const _
                    as usize
            },
            35usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_normalize_options),
                "::",
                stringify!(roman_numerals)
            )
        );
    }
    #[test]
    fn test_layout_libpostal_address_parser_options() {
        assert_eq!(
            ::std::mem::size_of::<libpostal_address_parser_options>(),
            16usize,
            concat!("Size of: ", stringify!(libpostal_address_parser_options))
        );
        assert_eq!(
            ::std::mem::align_of::<libpostal_address_parser_options>(),
            8usize,
            concat!(
                "Alignment of ",
                stringify!(libpostal_address_parser_options)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_address_parser_options>())).language as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_address_parser_options),
                "::",
                stringify!(language)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_address_parser_options>())).country as *const _
                    as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_address_parser_options),
                "::",
                stringify!(country)
            )
        );
    }

    #[test]
    fn test_layout_libpostal_address_parser_response() {
        assert_eq!(
            ::std::mem::size_of::<libpostal_address_parser_response>(),
            24usize,
            concat!("Size of: ", stringify!(libpostal_address_parser_response))
        );
        assert_eq!(
            ::std::mem::align_of::<libpostal_address_parser_response>(),
            8usize,
            concat!(
                "Alignment of ",
                stringify!(libpostal_address_parser_response)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_address_parser_response>())).num_components as *const _
                    as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_address_parser_response),
                "::",
                stringify!(num_components)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_address_parser_response>())).components as *const _
                    as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_address_parser_response),
                "::",
                stringify!(components)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<libpostal_address_parser_response>())).labels as *const _
                    as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(libpostal_address_parser_response),
                "::",
                stringify!(labels)
            )
        );
    }
}

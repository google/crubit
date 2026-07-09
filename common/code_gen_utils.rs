// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, ensure, Result};
use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::rc::Rc;

use crubit_feature::CrubitFeature;
use dyn_format::Format;
use flagset::FlagSet;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CcConstQualifier {
    Mut,
    Const,
}

/// Returns Some(..) if the cpp_type is a C++ pointer type.
pub fn is_cpp_pointer_type(cpp_type: TokenStream) -> Option<CcConstQualifier> {
    let mut first = None;
    let mut last = None;
    let mut prev_last = None;
    for token in cpp_type.into_iter() {
        first = if first.is_none() { Some(token.clone()) } else { first };
        prev_last = last;
        last = Some(token);
    }

    match (first, prev_last, last) {
        (Some(TokenTree::Ident(first)), _, Some(TokenTree::Punct(last)))
            if first == "const" && last.as_char() == '*' =>
        {
            Some(CcConstQualifier::Const)
        }
        (_, Some(TokenTree::Ident(prev_last)), Some(TokenTree::Punct(last)))
            if prev_last == "const" && last.as_char() == '*' =>
        {
            Some(CcConstQualifier::Const)
        }
        (_, _, Some(TokenTree::Punct(last))) if last.as_char() == '*' => {
            Some(CcConstQualifier::Mut)
        }
        _ => None,
    }
}

/// Returns true if the given identifier is a C++ reserved keyword according to
/// https://en.cppreference.com/w/cpp/keyword
pub fn is_cpp_reserved_keyword(ident: &str) -> bool {
    static RESERVED_CC_KEYWORDS: phf::Set<&'static str> = phf::phf_set! {
        "alignas",
        "alignof",
        "and",
        "and_eq",
        "asm",
        "atomic_cancel",
        "atomic_commit",
        "atomic_noexcept",
        "auto",
        "bitand",
        "bitor",
        "bool",
        "break",
        "case",
        "catch",
        "char",
        "char8_t",
        "char16_t",
        "char32_t",
        "class",
        "compl",
        "concept",
        "const",
        "consteval",
        "constexpr",
        "constinit",
        "const_cast",
        "continue",
        "co_await",
        "co_return",
        "co_yield",
        "decltype",
        "default",
        "delete",
        "do",
        "double",
        "dynamic_cast",
        "else",
        "enum",
        "explicit",
        "export",
        "extern",
        "false",
        "float",
        "for",
        "friend",
        "goto",
        "if",
        "inline",
        "int",
        "long",
        "mutable",
        "namespace",
        "new",
        "noexcept",
        "not",
        "not_eq",
        "nullptr",
        "operator",
        "or",
        "or_eq",
        "private",
        "protected",
        "public",
        "reflexpr",
        "register",
        "reinterpret_cast",
        "requires",
        "return",
        "short",
        "signed",
        "sizeof",
        "static",
        "static_assert",
        "static_cast",
        "struct",
        "switch",
        "synchronized",
        "template",
        "this",
        "thread_local",
        "throw",
        "true",
        "try",
        "typedef",
        "typeid",
        "typename",
        "union",
        "unsigned",
        "using",
        "virtual",
        "void",
        "volatile",
        "wchar_t",
        "while",
        "xor",
        "xor_eq",
    };

    RESERVED_CC_KEYWORDS.contains(ident)
}

/// Returns true if the given identifier is reserved in C++, either as a keyword
/// or as a standard macro that should not be shadowed.
pub fn is_cpp_nonportable_word(ident: &str, features: FlagSet<CrubitFeature>) -> bool {
    if is_cpp_reserved_keyword(ident) {
        return true;
    }

    if !features.contains(CrubitFeature::ReserveStandardMacros) {
        return false;
    }

    // Keep in sync with https://en.cppreference.com/cpp/symbol_index/macro
    static RESERVED_CC_MACROS: phf::Set<&'static str> = phf::phf_set! {
        "__alignas_is_defined",
        "__bool_true_false_are_defined",
        "__STDC_ENDIAN_BIG__",
        "__STDC_ENDIAN_LITTLE__",
        "__STDC_ENDIAN_NATIVE__",
        "__STDC_VERSION_STDBIT_H__",
        "__STDC_VERSION_STDCKDINT_H__",
        "_Atomic",
        "_IOFBF",
        "_IOLBF",
        "_IONBF",
        "assert",
        "ATOMIC_BOOL_LOCK_FREE",
        "ATOMIC_CHAR_LOCK_FREE",
        "ATOMIC_CHAR16_T_LOCK_FREE",
        "ATOMIC_CHAR32_T_LOCK_FREE",
        "ATOMIC_CHAR8_T_LOCK_FREE",
        "ATOMIC_FLAG_INIT",
        "ATOMIC_INT_LOCK_FREE",
        "ATOMIC_LLONG_LOCK_FREE",
        "ATOMIC_LONG_LOCK_FREE",
        "ATOMIC_POINTER_LOCK_FREE",
        "ATOMIC_SHORT_LOCK_FREE",
        "ATOMIC_VAR_INIT",
        "ATOMIC_WCHAR_T_LOCK_FREE",
        "BOOL_WIDTH",
        "BUFSIZ",
        "CHAR_BIT",
        "CHAR_MAX",
        "CHAR_MIN",
        "CHAR_WIDTH",
        "CLOCKS_PER_SEC",
        "DBL_DECIMAL_DIG",
        "DBL_DIG",
        "DBL_EPSILON",
        "DBL_HAS_SUBNORM",
        "DBL_MANT_DIG",
        "DBL_MAX",
        "DBL_MAX_10_EXP",
        "DBL_MAX_EXP",
        "DBL_MIN",
        "DBL_MIN_10_EXP",
        "DBL_MIN_EXP",
        "DBL_TRUE_MIN",
        "DECIMAL_DIG",
        "E2BIG",
        "EACCES",
        "EADDRINUSE",
        "EADDRNOTAVAIL",
        "EAFNOSUPPORT",
        "EAGAIN",
        "EALREADY",
        "EBADF",
        "EBADMSG",
        "EBUSY",
        "ECANCELED",
        "ECHILD",
        "ECONNABORTED",
        "ECONNREFUSED",
        "ECONNRESET",
        "EDEADLK",
        "EDESTADDRREQ",
        "EDOM",
        "EEXIST",
        "EFAULT",
        "EFBIG",
        "EHOSTUNREACH",
        "EIDRM",
        "EILSEQ",
        "EINPROGRESS",
        "EINTR",
        "EINVAL",
        "EIO",
        "EISCONN",
        "EISDIR",
        "ELOOP",
        "EMFILE",
        "EMLINK",
        "EMSGSIZE",
        "ENAMETOOLONG",
        "ENETDOWN",
        "ENETRESET",
        "ENETUNREACH",
        "ENFILE",
        "ENOBUFS",
        "ENODATA",
        "ENODEV",
        "ENOENT",
        "ENOEXEC",
        "ENOLCK",
        "ENOLINK",
        "ENOMEM",
        "ENOMSG",
        "ENOPROTOOPT",
        "ENOSPC",
        "ENOSR",
        "ENOSTR",
        "ENOSYS",
        "ENOTCONN",
        "ENOTDIR",
        "ENOTEMPTY",
        "ENOTRECOVERABLE",
        "ENOTSOCK",
        "ENOTSUP",
        "ENOTTY",
        "ENXIO",
        "EOF",
        "EOPNOTSUPP",
        "EOVERFLOW",
        "EOWNERDEAD",
        "EPERM",
        "EPIPE",
        "EPROTO",
        "EPROTONOSUPPORT",
        "EPROTOTYPE",
        "ERANGE",
        "EROFS",
        "errno",
        "ESPIPE",
        "ESRCH",
        "ETIME",
        "ETIMEDOUT",
        "ETXTBSY",
        "EWOULDBLOCK",
        "EXDEV",
        "EXIT_FAILURE",
        "EXIT_SUCCESS",
        "FE_ALL_EXCEPT",
        "FE_DFL_ENV",
        "FE_DIVBYZERO",
        "FE_DOWNWARD",
        "FE_INEXACT",
        "FE_INVALID",
        "FE_OVERFLOW",
        "FE_TONEAREST",
        "FE_TOWARDZERO",
        "FE_UNDERFLOW",
        "FE_UPWARD",
        "FILENAME_MAX",
        "FLT_DECIMAL_DIG",
        "FLT_DIG",
        "FLT_EPSILON",
        "FLT_EVAL_METHOD",
        "FLT_HAS_SUBNORM",
        "FLT_MANT_DIG",
        "FLT_MAX",
        "FLT_MAX_10_EXP",
        "FLT_MAX_EXP",
        "FLT_MIN",
        "FLT_MIN_10_EXP",
        "FLT_MIN_EXP",
        "FLT_RADIX",
        "FLT_ROUNDS",
        "FLT_TRUE_MIN",
        "FOPEN_MAX",
        "FP_FAST_FMA",
        "FP_FAST_FMAF",
        "FP_FAST_FMAL",
        "FP_ILOGB0",
        "FP_ILOGBNAN",
        "FP_SUBNORMAL",
        "FP_ZERO",
        "FP_INFINITE",
        "FP_NAN",
        "FP_NORMAL",
        "HUGE_VAL",
        "HUGE_VALF",
        "HUGE_VALL",
        "INFINITY",
        "INT_FAST16_MAX",
        "INT_FAST16_MIN",
        "INT_FAST32_MAX",
        "INT_FAST32_MIN",
        "INT_FAST64_MAX",
        "INT_FAST64_MIN",
        "INT_FAST8_MAX",
        "INT_FAST8_MIN",
        "INT_LEAST16_MAX",
        "INT_LEAST16_MIN",
        "INT_LEAST32_MAX",
        "INT_LEAST32_MIN",
        "INT_LEAST64_MAX",
        "INT_LEAST64_MIN",
        "INT_LEAST8_MAX",
        "INT_LEAST8_MIN",
        "INT_MAX",
        "INT_MIN",
        "INT_WIDTH",
        "INT16_C",
        "INT16_MAX",
        "INT16_MIN",
        "INT32_C",
        "INT32_MAX",
        "INT32_MIN",
        "INT64_C",
        "INT64_MAX",
        "INT64_MIN",
        "INT8_C",
        "INT8_MAX",
        "INT8_MIN",
        "INTMAX_C",
        "INTMAX_MAX",
        "INTMAX_MIN",
        "INTPTR_MAX",
        "INTPTR_MIN",
        "L_tmpnam",
        "LC_ALL",
        "LC_COLLATE",
        "LC_CTYPE",
        "LC_MONETARY",
        "LC_NUMERIC",
        "LC_TIME",
        "LDBL_DECIMAL_DIG",
        "LDBL_DIG",
        "LDBL_EPSILON",
        "LDBL_HAS_SUBNORM",
        "LDBL_MANT_DIG",
        "LDBL_MAX",
        "LDBL_MAX_10_EXP",
        "LDBL_MAX_EXP",
        "LDBL_MIN",
        "LDBL_MIN_10_EXP",
        "LDBL_MIN_EXP",
        "LDBL_TRUE_MIN",
        "LLONG_MAX",
        "LLONG_MIN",
        "LLONG_WIDTH",
        "LONG_MAX",
        "LONG_MIN",
        "LONG_WIDTH",
        "MATH_ERREXCEPT",
        "math_errhandling",
        "MATH_ERRNO",
        "MB_CUR_MAX",
        "MB_LEN_MAX",
        "NAN",
        "NULL",
        "offsetof",
        "ONCE_FLAG_INIT",
        "PRId16",
        "PRId32",
        "PRId64",
        "PRId8",
        "PRIdFAST16",
        "PRIdFAST32",
        "PRIdFAST64",
        "PRIdFAST8",
        "PRIdLEAST16",
        "PRIdLEAST32",
        "PRIdLEAST64",
        "PRIdLEAST8",
        "PRIdMAX",
        "PRIdPTR",
        "PRIi16",
        "PRIi32",
        "PRIi64",
        "PRIi8",
        "PRIiFAST16",
        "PRIiFAST32",
        "PRIiFAST64",
        "PRIiFAST8",
        "PRIiLEAST16",
        "PRIiLEAST32",
        "PRIiLEAST64",
        "PRIiLEAST8",
        "PRIiMAX",
        "PRIiPTR",
        "PRIo16",
        "PRIo32",
        "PRIo64",
        "PRIo8",
        "PRIoFAST16",
        "PRIoFAST32",
        "PRIoFAST64",
        "PRIoFAST8",
        "PRIoLEAST16",
        "PRIoLEAST32",
        "PRIoLEAST64",
        "PRIoLEAST8",
        "PRIoMAX",
        "PRIoPTR",
        "PRIu16",
        "PRIu32",
        "PRIu64",
        "PRIu8",
        "PRIuFAST16",
        "PRIuFAST32",
        "PRIuFAST64",
        "PRIuFAST8",
        "PRIuLEAST16",
        "PRIuLEAST32",
        "PRIuLEAST64",
        "PRIuLEAST8",
        "PRIuMAX",
        "PRIuPTR",
        "PRIx16",
        "PRIX16",
        "PRIx32",
        "PRIX32",
        "PRIx64",
        "PRIX64",
        "PRIx8",
        "PRIX8",
        "PRIxFAST16",
        "PRIXFAST16",
        "PRIxFAST32",
        "PRIXFAST32",
        "PRIxFAST64",
        "PRIXFAST64",
        "PRIxFAST8",
        "PRIXFAST8",
        "PRIxLEAST16",
        "PRIXLEAST16",
        "PRIxLEAST32",
        "PRIXLEAST32",
        "PRIxLEAST64",
        "PRIXLEAST64",
        "PRIxLEAST8",
        "PRIXLEAST8",
        "PRIxMAX",
        "PRIXMAX",
        "PRIxPTR",
        "PRIXPTR",
        "PTRDIFF_MAX",
        "PTRDIFF_MIN",
        "RAND_MAX",
        "SCHAR_MAX",
        "SCHAR_MIN",
        "SCHAR_WIDTH",
        "SCNd16",
        "SCNd32",
        "SCNd64",
        "SCNd8",
        "SCNdFAST16",
        "SCNdFAST32",
        "SCNdFAST64",
        "SCNdFAST8",
        "SCNdLEAST16",
        "SCNdLEAST32",
        "SCNdLEAST64",
        "SCNdLEAST8",
        "SCNdMAX",
        "SCNdPTR",
        "SCNi16",
        "SCNi32",
        "SCNi64",
        "SCNi8",
        "SCNiFAST16",
        "SCNiFAST32",
        "SCNiFAST64",
        "SCNiFAST8",
        "SCNiLEAST16",
        "SCNiLEAST32",
        "SCNiLEAST64",
        "SCNiLEAST8",
        "SCNiMAX",
        "SCNiPTR",
        "SCNo16",
        "SCNo32",
        "SCNo64",
        "SCNo8",
        "SCNoFAST16",
        "SCNoFAST32",
        "SCNoFAST64",
        "SCNoFAST8",
        "SCNoLEAST16",
        "SCNoLEAST32",
        "SCNoLEAST64",
        "SCNoLEAST8",
        "SCNoMAX",
        "SCNoPTR",
        "SCNu16",
        "SCNu32",
        "SCNu64",
        "SCNu8",
        "SCNuFAST16",
        "SCNuFAST32",
        "SCNuFAST64",
        "SCNuFAST8",
        "SCNuLEAST16",
        "SCNuLEAST32",
        "SCNuLEAST64",
        "SCNuLEAST8",
        "SCNuMAX",
        "SCNuPTR",
        "SCNx16",
        "SCNx32",
        "SCNx64",
        "SCNx8",
        "SCNxFAST16",
        "SCNxFAST32",
        "SCNxFAST64",
        "SCNxFAST8",
        "SCNxLEAST16",
        "SCNxLEAST32",
        "SCNxLEAST64",
        "SCNxLEAST8",
        "SCNxMAX",
        "SCNxPTR",
        "SEEK_CUR",
        "SEEK_END",
        "SEEK_SET",
        "setjmp",
        "SHRT_MAX",
        "SHRT_MIN",
        "SHRT_WIDTH",
        "SIG_ATOMIC_MAX",
        "SIG_ATOMIC_MIN",
        "SIG_DFL",
        "SIG_ERR",
        "SIG_IGN",
        "SIGABRT",
        "SIGFPE",
        "SIGILL",
        "SIGINT",
        "SIGSEGV",
        "SIGTERM",
        "SIZE_MAX",
        "stderr",
        "stdin",
        "stdout",
        "TIME_UTC",
        "TMP_MAX",
        "UCHAR_MAX",
        "UCHAR_WIDTH",
        "UINT_FAST16_MAX",
        "UINT_FAST32_MAX",
        "UINT_FAST64_MAX",
        "UINT_FAST8_MAX",
        "UINT_LEAST16_MAX",
        "UINT_LEAST32_MAX",
        "UINT_LEAST64_MAX",
        "UINT_LEAST8_MAX",
        "UINT_MAX",
        "UINT_WIDTH",
        "UINT16_C",
        "UINT16_MAX",
        "UINT32_C",
        "UINT32_MAX",
        "UINT64_MAX",
        "UINT64_C",
        "UINT8_C",
        "UINT8_MAX",
        "UINTMAX_C",
        "UINTMAX_MAX",
        "UINTPTR_MAX",
        "ULLONG_MAX",
        "ULLONG_WIDTH",
        "ULONG_MAX",
        "ULONG_WIDTH",
        "USHRT_MAX",
        "USHRT_WIDTH",
        "va_arg",
        "va_copy",
        "va_end",
        "va_start",
        "WCHAR_MAX",
        "WCHAR_MIN",
        "WEOF",
        "WINT_MAX",
        "WINT_MIN",
    };

    RESERVED_CC_MACROS.contains(ident)
}

/// If `ident` is reserved in C++, returns a string with an underscore appended to it.
/// Otherwise, returns `ident`.
pub fn unkeyword_cpp_ident(ident: &str, features: FlagSet<CrubitFeature>) -> Cow<'_, str> {
    if is_cpp_nonportable_word(ident, features) {
        Cow::Owned(format!("{ident}_"))
    } else {
        Cow::Borrowed(ident)
    }
}

/// Formats a C++ identifier. Panics if `ident` is a C++ reserved word.
///
/// This should only be used for generated identifiers that we control, and can guarantee do not
/// collide with reserved words in the standard.
#[track_caller]
pub fn expect_format_cc_ident(ident: &str) -> Ident {
    format_cc_ident(ident, FlagSet::<CrubitFeature>::full())
        .unwrap_or_else(|err| panic!("Can't format `{ident}` as a C++ identifier: {err}"))
}

/// Fallibly parses a [`proc_macro2::Ident`] where Rust keywords are permitted.
///
/// * Unlike [`Ident::new`], this is fallible and returns Err if it fails.
/// * Unlike [`syn::parse_str::<Ident>`], Rust keywords are permitted, making this suitable for
///   parsing identifiers that that Rust doesn't allow but C++ does, like "move".
fn parse_any(ident: &str) -> syn::Result<Ident> {
    syn::parse::Parser::parse_str(<Ident as syn::ext::IdentExt>::parse_any, ident)
}

/// Formats a C++ (qualified) identifier. Returns an error when `ident` is a C++
/// reserved keyword or is an invalid identifier.
pub fn format_cc_ident(ident: &str, features: FlagSet<CrubitFeature>) -> Result<Ident> {
    ensure!(
        !is_cpp_nonportable_word(ident, features),
        "`{ident}` is a C++ reserved word and can't be used as a C++ identifier",
    );
    unchecked_format_cc_ident(ident)
}

/// Formats a C++ (qualified) identifier. Returns an error when `ident` is a C++
/// an invalid identifier.
///
/// Use this function when the identifier originally came from C++, otherwise, use
/// `format_cc_ident`.
pub fn format_nonportable_cc_ident(ident: &str) -> Result<Ident> {
    ensure!(
        !is_cpp_reserved_keyword(ident),
        "`{ident}` is a C++ reserved word and can't be used as a C++ identifier",
    );
    unchecked_format_cc_ident(ident)
}

fn unchecked_format_cc_ident(ident: &str) -> Result<Ident> {
    ensure!(!ident.is_empty(), "Empty string is not a valid C++ identifier");
    // Explicitly mapping the error via `anyhow!`, because `LexError` is not `Sync`
    // (required for `anyhow::Error` to implement `From<LexError>`) and
    // therefore we can't just use `?`.
    parse_any(ident)
        .map_err(|lex_error| anyhow!("Can't format `{ident}` as a C++ identifier: {lex_error}"))
}

/// Formats a C++ type name. Returns an error when `name` is a C++
/// reserved keyword or otherwise an invalid type name.
pub fn format_cc_type_name(name: &str, features: FlagSet<CrubitFeature>) -> Result<TokenStream> {
    check_portable_cc_name(name, features)?;
    unchecked_format_cc_type_name(name)
}

pub fn format_nonportable_cc_type_name(name: &str) -> Result<TokenStream> {
    check_valid_cc_name(name)?;
    unchecked_format_cc_type_name(name)
}

fn unchecked_format_cc_type_name(name: &str) -> Result<TokenStream> {
    match name.parse() {
        Ok(name) => Ok(name),
        Err(_) => {
            // Sometimes valid C++ type names do not parse as TokenStreams.
            // For example, this can happen in the case of literals that are not
            // valid Rust: IntTemplateStruct<'\f'>.
            //
            // As a last ditch effort, we convert the identifier into a string literal.
            // We could use quote! to do the escaping, but instead we do it manually to
            // mirror the code in token_stream_printer.
            let name = name.replace("\\", "\\\\").replace('"', "\\\"");
            let name = format!("__LITERALLY__ \"{name}\"");
            name.parse()
                .map_err(|lex_error| anyhow!("Can't format `{name}` as a C++ type: {lex_error}"))
        }
    }
}

/// Converts a hyphen-separated string to an underscore-separated string.
///
/// Panics if `s` is empty.
fn hyphen_to_underscore(s: &str) -> Cow<'_, str> {
    let mut parts = s.split('-');
    let first_part = parts.next().expect("at least one part expected");
    let Some(second_part) = parts.next() else {
        return Cow::Borrowed(first_part);
    };
    let mut owned = String::with_capacity(s.len());
    let _ = write!(&mut owned, "{first_part}_{second_part}");
    for part in parts {
        let _ = write!(&mut owned, "_{part}");
    }
    Cow::Owned(owned)
}

/// Makes an 'Ident' to be used in the Rust source code. Escapes Rust keywords.
/// Panics if `ident` is empty or is otherwise an invalid identifier.
///
/// Hyphens are converted to underscores in the identifier.
pub fn make_rs_ident(ident: &str) -> Ident {
    try_make_rs_ident(ident).expect("Failed to make Rust identifier")
}

/// Makes an 'Ident' to be used in the Rust source code. Escapes Rust keywords.
/// Returns an error if `ident` is empty or is otherwise an invalid identifier.
///
/// Hyphens are converted to underscores in the identifier.
pub fn try_make_rs_ident(ident: &str) -> Result<Ident> {
    // Target names, which become crate names, may sometimes contain hyphens.
    // Since identifiers cannot contain hyphens, we convert them to underscores.
    let ident = hyphen_to_underscore(ident);

    // TODO(https://github.com/dtolnay/syn/pull/1098): Remove the hardcoded list once syn recognizes
    // newly added keywords.
    // NOTE: the above PR was accepted for 2021 and 2018 editions, but `try` still isn't escaped,
    // so we may need to tweak something.
    if matches!(&*ident, "gen" | "async" | "await" | "try" | "dyn") {
        return Ok(format_ident!("r#{}", ident));
    }

    syn::parse_str(&ident).or_else(|_| syn::parse_str(&format!("r#{ident}"))).map_err(Into::into)
}

/// Makes a 'Lifetime' to be used in the Rust source code as a lifetime name.
/// Panics if `ident` is empty or is otherwise an invalid identifier.
///
/// Hyphens are converted to underscores in the identifier.
pub fn make_rs_lifetime_ident(ident: &str) -> syn::Lifetime {
    if ident == "_" || ident == "static" {
        return syn::Lifetime::new(&format!("'{ident}"), proc_macro2::Span::call_site());
    }
    syn::Lifetime { apostrophe: proc_macro2::Span::call_site(), ident: make_rs_ident(ident) }
}

fn check_valid_cc_name_impl(name: &str) -> Result<()> {
    // https://en.cppreference.com/w/cpp/language/identifiers says that "A valid identifier must
    // begin with a non-digit character (Latin letter, underscore, or Unicode
    // character of class XID_Start)".  One motivation for this check is to
    // explicitly catch names of tuple fields (e.g. `some_tuple.0`).
    let first_char =
        name.chars().next().ok_or_else(|| anyhow!("Empty string is not a valid C++ identifier"))?;
    ensure!(
        unicode_ident::is_xid_start(first_char) || first_char == '_' || first_char == ':',
        "The following character can't be used as a start of a C++ identifier: {first_char}",
    );

    Ok(())
}

/// Checks that `name` is a valid C++ identifier.
pub fn check_valid_cc_name(name: &str) -> Result<()> {
    ensure!(
        !is_cpp_reserved_keyword(name),
        "`{name}` is a C++ reserved keyword and can't be used as a C++ identifier",
    );
    check_valid_cc_name_impl(name)
}

/// Checks that `name` is a valid C++ identifier on typical platforms.
///
/// This also rejects names that collide with macros defined in the C++ standard library.
pub fn check_portable_cc_name(name: &str, features: FlagSet<CrubitFeature>) -> Result<()> {
    ensure!(
        !is_cpp_nonportable_word(name, features),
        "`{name}` is a C++ reserved word and can't be used as a C++ identifier",
    );
    check_valid_cc_name_impl(name)
}

/// Escapes characters that may not appear in a C++ or Rust identifier.
///
/// The implemented escaping algorithm guarantess that different inputs will
/// always produce different outputs (i.e. unique symbols will remain unique
/// after escaping).  Other than that, the implemented escaping algorithm is
/// somewhat arbitrary and should be treated as an implementation detail and not
/// depended upon.
///
/// This transformation allows using escaped symbol names as part of Rust and/or
/// C++ identifiers. In particular note that in practice Rust uses `$` and `.`
/// characters in symbols - for example: "_ZN58_$LT$rust_out..
/// Point$u20$as$u20$core..default..Default$GT$7default17h144069f0ad7be325E".
pub fn escape_non_identifier_chars(symbol: &str) -> String {
    // EXTRA_CAPACITY_PREDICTION has been haphazardly chosen based on a single
    // example encountered in practice where there were 16 characters that needed
    // escaping: 2 x '_', 8 x '$', 6 x '.': "_ZN58_$LT$rust_out..
    // Point$u20$as$u20$core..default..Default$GT$7default17h144069f0ad7be325E"
    const EXTRA_CAPACITY_PREDICTION: usize = 20;
    let mut result = String::with_capacity(symbol.len() + EXTRA_CAPACITY_PREDICTION);

    for (i, c) in symbol.chars().enumerate() {
        match c {
            '_' => result.push_str("_u"),
            '$' => result.push_str("_d"),
            '.' => result.push_str("_p"),
            c => {
                let is_valid_identifier_char = if i == 0 {
                    // `is_xid_start` doesn't cover `'_'` character, but it is okay because we
                    // explicitly handle this character in a match branch above.
                    unicode_ident::is_xid_start(c)
                } else {
                    unicode_ident::is_xid_continue(c)
                };
                if is_valid_identifier_char {
                    result.push(c);
                } else {
                    _ = write!(&mut result, "_x{:08x}", c as u32);
                };
            }
        }
    }

    result
}

/// Representation of `foo::bar::baz` where each component is either the name
/// of a C++ namespace, or the name of a Rust module.
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct NamespaceQualifier {
    // Outer to innermost
    pub namespaces: Vec<Rc<str>>,
    // Outer to innermost. Paired as (rs_name, cc_name)
    pub nested_records: Vec<(Rc<str>, Rc<str>)>,
    /// Whether to prepend `::` when formatting for C++.
    // TODO(b/502939407): Remove this field (it will always be true).
    pub use_leading_colons: bool,
}

impl NamespaceQualifier {
    /// Constructs a new `NamespaceQualifier` from a sequence of names.
    pub fn new<T: Into<Rc<str>>>(
        iter: impl IntoIterator<Item = T>,
        use_leading_colons: bool,
    ) -> Self {
        // TODO(b/258265044): Catch most (all if possible) error conditions early.  For
        // example:
        // - Panic early if any strings are empty, or are not Rust identifiers
        // - Report an error early if any strings are C++ reserved keywords
        // This may make `format_for_cc` and `format_namespace_bound_cc_tokens` infallible.
        Self {
            namespaces: iter.into_iter().map(Into::into).collect(),
            nested_records: vec![],
            use_leading_colons,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.namespaces.is_empty() && self.nested_records.is_empty()
    }

    /// Returns an iterator over the Rust modules in the namespace qualifier.
    /// Parent records are converted to snake case.
    pub fn parts_with_snake_case_record_names(&self) -> impl Iterator<Item = Ident> + use<'_> {
        self.namespaces.iter().map(|ns| make_rs_ident(ns)).chain(
            self.nested_records
                .iter()
                .map(|(rs_name, _cc_name)| make_rs_ident(&rs_name.to_string().to_snake_case())),
        )
    }

    pub fn parts(&self) -> impl Iterator<Item = &Rc<str>> + use<'_> {
        self.namespaces.iter().chain(self.nested_records.iter().map(|(_rs_name, cc_name)| cc_name))
    }

    /// Returns `foo::bar::baz::` (escaping Rust keywords as needed).
    pub fn format_for_rs(&self) -> TokenStream {
        let rust_parts = self.parts_with_snake_case_record_names();
        quote! { #(#rust_parts::)* }
    }

    /// Returns `foo::bar::baz::` (reporting errors for C++ keywords).
    pub fn format_for_cc(&self, features: FlagSet<CrubitFeature>) -> Result<TokenStream> {
        let mut path = if self.use_leading_colons {
            quote! { :: }
        } else {
            quote! {}
        };
        for namespace in &self.namespaces {
            let namespace = format_cc_ident(namespace, features)?;
            path.extend(quote! { #namespace :: });
        }
        for (_rs_name, cc_name) in &self.nested_records {
            let cc_name = format_cc_type_name(cc_name, features)?;
            path.extend(quote! { #cc_name ::});
        }
        Ok(path)
    }

    /// Returns `foo::bar::baz::` (never reporting errors).
    pub fn format_for_cc_debug(&self) -> String {
        let mut path = String::new();
        for part in self.parts() {
            path.push_str(part);
            path.push_str("::");
        }
        path
    }
}

/// `CcInclude` represents a single `#include ...` directive in C++.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CcInclude {
    /// Represents a system header, e.g., `cstdef`, which will be included by
    /// angular brackets.
    SystemHeader(Rc<str>),
    /// Represents a user header, which will be included by quotes.
    UserHeader(Rc<str>),
    /// Represents an `#include` for Crubit C++ support library headers: the
    /// format specifier for what comes after `#include` and path of the support
    /// library header.
    SupportLibHeader(Format<1>, Rc<str>),
}

impl CcInclude {
    /// Creates a `CcInclude` that represents `#include <array>` and provides
    /// C++ types like `std::array`.
    /// https://en.cppreference.com/w/cpp/header/array
    pub fn array() -> Self {
        Self::SystemHeader("array".into())
    }

    /// Creates a `CcInclude` that represents `#include <compare>` and provides
    /// C++ types like `std::strong_ordering`.
    /// https://en.cppreference.com/w/cpp/header/compare
    pub fn compare() -> Self {
        Self::SystemHeader("compare".into())
    }

    /// Creates a `CcInclude` that represents `#include <cstddef>` and provides
    /// C++ types like `std::size_t` or `std::ptrdiff_t`.  See
    /// https://en.cppreference.com/w/cpp/header/cstddef
    pub fn cstddef() -> Self {
        Self::SystemHeader("cstddef".into())
    }

    /// Creates a `CcInclude` that represents `#include <cstdint>` and provides
    /// C++ types like `std::int16_t` or `std::uint32_t`.  See
    /// https://en.cppreference.com/w/cpp/header/cstdint
    pub fn cstdint() -> Self {
        Self::SystemHeader("cstdint".into())
    }

    /// Creates a `CcInclude` that represents `#include <cstring>` and provides
    /// C++ methods like `std::memcpy`.
    /// https://en.cppreference.com/w/cpp/header/cstring
    pub fn cstring() -> Self {
        Self::SystemHeader("cstring".into())
    }

    /// Creates a `CcInclude` that represents `#include <memory>`.
    /// See https://en.cppreference.com/w/cpp/header/memory
    pub fn memory() -> Self {
        Self::SystemHeader("memory".into())
    }

    /// Creates a `CcInclude` that represents `#include <utility>` and provides
    /// C++ functions like `std::move` and C++ types like `std::tuple`.
    /// See https://en.cppreference.com/w/cpp/header/utility
    pub fn utility() -> Self {
        Self::SystemHeader("utility".into())
    }

    /// Creates a `CcInclude` that represents `#include <tuple>` and provides
    /// C++ `std::tuple`, `std::tie`, etc.
    /// See https://en.cppreference.com/w/cpp/header/utility
    pub fn tuple() -> Self {
        Self::SystemHeader("tuple".into())
    }

    /// Creates a `CcInclude` that represents `#include <optional>` and provides
    /// C++ `std::optional`.
    /// See https://en.cppreference.com/w/cpp/header/optional
    pub fn optional() -> Self {
        Self::SystemHeader("optional".into())
    }

    /// Creates a `CcInclude` that represents `#include <bit>` and provides
    /// C++ functions like `std::bit_cast`.
    /// See https://en.cppreference.com/w/cpp/header/bit
    pub fn bit() -> Self {
        Self::SystemHeader("bit".into())
    }

    /// Creates a `CcInclude` that represents `#include <type_traits>` and
    /// provides C++ APIs like `std::is_trivially_copy_constructible_v`.
    /// See https://en.cppreference.com/w/cpp/header/type_traits
    pub fn type_traits() -> Self {
        Self::SystemHeader("type_traits".into())
    }

    /// Creates a user include: `#include "some/path/to/header.h"`.
    pub fn user_header(path: Rc<str>) -> Self {
        Self::UserHeader(path)
    }

    /// Creates a `CcInclude` and detects whether it's a system header or a user
    /// header based on the path.
    ///
    /// System headers are included by angular brackets, e.g., `#include <cstddef>`.
    /// User headers are included by quotes, e.g., `#include "some/path/to/header.h"`.
    pub fn from_path(path: &str) -> Self {
        match (path.starts_with("<"), path.ends_with(">")) {
            (true, true) => Self::SystemHeader(Rc::from(&path[1..path.len() - 1])),
            _ => Self::UserHeader(Rc::from(path)),
        }
    }

    /// Creates a support library header include based on the specified format.
    /// E.g., `\"{header}\"` and `hdr.h` produces `#include "hdr.h"`.
    pub fn support_lib_header(format: Format<1>, path: Rc<str>) -> Self {
        Self::SupportLibHeader(format, path)
    }
}

impl ToTokens for CcInclude {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::SystemHeader(path) => {
                let path: TokenStream = path
                    .parse()
                    .expect("`pub` API of `CcInclude` guarantees validity of system includes");
                quote! { __HASH_TOKEN__ include < #path > __NEWLINE__ }.to_tokens(tokens)
            }
            Self::UserHeader(path) => {
                quote! { __HASH_TOKEN__ include #path __NEWLINE__ }.to_tokens(tokens)
            }
            Self::SupportLibHeader(format, path) => {
                let full_path: TokenStream = format
                    .format(&[&*path])
                    .parse()
                    .expect("Failed to parse support lib `#include` path");
                quote! { __HASH_TOKEN__ include #full_path __NEWLINE__ }.to_tokens(tokens)
            }
        }
    }
}

/// Formats a set of `CcInclude`s, trying to follow the guidance from
/// [the Google C++ Style Guide](https://google.github.io/styleguide/cppguide.html#Names_and_Order_of_Includes).
pub fn format_cc_includes(set_of_includes: &BTreeSet<CcInclude>) -> TokenStream {
    let mut tokens = TokenStream::default();
    let mut iter = set_of_includes.iter().peekable();
    while let Some(include) = iter.next() {
        include.to_tokens(&mut tokens);

        // Add an empty line between system headers and user headers.
        if let (CcInclude::SystemHeader(_), Some(CcInclude::UserHeader(_))) = (include, iter.peek())
        {
            quote! { __NEWLINE__ }.to_tokens(&mut tokens)
        }
    }
    tokens
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use googletest::matchers::starts_with;
    use googletest::{expect_that, gtest};
    use itertools::Itertools;
    use quote::{quote, ToTokens};
    use token_stream_matchers::{assert_cc_matches, assert_rs_matches};
    use token_stream_printer::cc_tokens_to_formatted_string_for_tests;

    #[gtest]
    fn test_format_cc_ident_basic() {
        assert_cc_matches!(
            format_cc_ident("foo", FlagSet::default()).unwrap().to_token_stream(),
            quote! { foo }
        );
    }

    #[gtest]
    fn test_format_cc_ident_exotic_xid_start() {
        assert_cc_matches!(
            format_cc_ident("Łukasz", FlagSet::default()).unwrap().to_token_stream(),
            quote! { Łukasz }
        );
    }

    #[gtest]
    fn test_format_cc_ident_underscore() {
        assert_cc_matches!(
            format_cc_ident("_", FlagSet::default()).unwrap().to_token_stream(),
            quote! { _ }
        );
    }

    #[gtest]
    fn test_format_cc_ident_reserved_rust_keyword() {
        assert_cc_matches!(
            format_cc_ident("impl", FlagSet::default()).unwrap().to_token_stream(),
            quote! { impl }
        );
    }

    #[gtest]
    fn test_format_cc_ident_reserved_cc_keyword() {
        let err = format_cc_ident("reinterpret_cast", FlagSet::default()).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("`reinterpret_cast`"));
        assert!(msg.contains("C++ reserved word"));
    }

    #[gtest]
    fn test_format_cc_ident_reserved_standard_macros() {
        assert_cc_matches!(
            format_cc_ident("stdin", FlagSet::default()).unwrap().to_token_stream(),
            quote! { stdin }
        );

        let err =
            format_cc_ident("stdin", CrubitFeature::ReserveStandardMacros.into()).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("`stdin`"));
        assert!(msg.contains("C++ reserved word"));
    }

    #[gtest]
    fn test_format_cc_ident_unqualified_identifiers() {
        // https://en.cppreference.com/w/cpp/language/identifiers#Unqualified_identifiers

        // These may appear in `IR::Func::name`.
        assert_cc_matches!(
            format_cc_type_name("operator==", FlagSet::default()).unwrap(),
            quote! { operator== }
        );
        assert_cc_matches!(
            format_cc_type_name("operator new", FlagSet::default()).unwrap(),
            quote! { operator new }
        );

        // This may appear in `IR::Record::cc_name` (although in practice these will
        // be namespace-qualified most of the time).
        assert_cc_matches!(
            format_cc_type_name("MyTemplate<int>", FlagSet::default()).unwrap(),
            quote! { MyTemplate<int> }
        );
    }

    /// https://en.cppreference.com/w/cpp/language/identifiers#Qualified_identifiers
    ///
    /// This may appear in `IR::Record::cc_name`, or in
    /// `crubit_annotate::cpp_layout_equivalent(cpp_type=...)`.
    #[gtest]
    fn test_format_cc_ident_qualified_identifiers() {
        assert_cc_matches!(
            format_cc_type_name("std::vector<int>", FlagSet::default()).unwrap(),
            quote! { std::vector<int> }
        );
        assert_cc_matches!(
            format_cc_type_name("::std::vector<int>", FlagSet::default()).unwrap(),
            quote! { ::std::vector<int> }
        );
    }

    #[gtest]
    fn test_format_cc_ident_empty() {
        let err = format_cc_ident("", FlagSet::default()).unwrap_err();
        let msg = err.to_string();
        assert_eq!(msg, "Empty string is not a valid C++ identifier");
    }

    #[gtest]
    fn test_format_cc_ident_invalid_first_char() {
        let tests = vec![
            // `0` and `1 are field names in `struct RustStruct(i32, u16)`.
            "0",
            // `~MyClass` is a valid unqualified identifier in C++, but it is okay if
            // `format_cc_ident` rejects it, because `format_cc_ident` is not used to format
            // destructor names.
            "~MyClass",
            // We used to trim leading and/or trailing whitespace, but stricter validation
            // of leading whitespace seems desirable.
            r#" operator "" _km "#,
            // Other tests
            "(foo",
            "(foo)",
        ];
        for test in tests.into_iter() {
            let err = format_cc_ident(test, FlagSet::default()).unwrap_err();
            let actual_msg = err.to_string();
            let expected_msg = format!("Can't format `{test}` as a C++ identifier: ");
            expect_that!(actual_msg, starts_with(expected_msg));
        }
    }

    #[gtest]
    fn test_make_rs_ident_basic() {
        let id = make_rs_ident("foo");
        assert_rs_matches!(quote! { #id }, quote! { foo });
    }

    #[gtest]
    fn test_make_rs_ident_reserved_cc_keyword() {
        let id = make_rs_ident("reinterpret_cast");
        assert_rs_matches!(quote! { #id }, quote! { reinterpret_cast });
    }

    #[gtest]
    fn test_make_rs_ident_reserved_rust_keyword() {
        let id = make_rs_ident("impl");
        assert_rs_matches!(quote! { #id }, quote! { r#impl });
    }

    #[gtest]
    #[should_panic]
    fn test_make_rs_ident_invalid_identifier() {
        make_rs_ident("foo!%@^$!^%@$!@bar"); // Invalid Rust identifier.
    }

    #[gtest]
    fn test_try_make_rs_ident_unfinished_group() {
        // This test specifically uses `(foo` (instead of some other invalid identifier)
        // so that the input string is not just an invalid identifier, but is also an invalid
        // token tree.  (I vaguely remember that the test was introduced after discovering
        // or suspecting that an old implementation was sensitive to this distinction.)
        let result = try_make_rs_ident("(foo"); // No closing `)`.
        assert!(result.is_err());
    }

    #[gtest]
    fn test_try_make_rs_ident_empty() {
        let result = try_make_rs_ident("");
        assert!(result.is_err());
    }

    #[gtest]
    fn test_cc_include_to_tokens_for_system_header() {
        let include = CcInclude::cstddef();
        assert_cc_matches!(
            quote! { #include },
            quote! {
                __HASH_TOKEN__ include <cstddef>
            }
        );
    }

    #[gtest]
    fn test_cc_include_to_tokens_for_user_header() {
        let include = CcInclude::user_header("some/path/to/header.h".into());
        assert_cc_matches!(
            quote! { #include },
            quote! {
                __HASH_TOKEN__ include "some/path/to/header.h"
            }
        );
    }

    #[gtest]
    fn test_cc_include_ord() {
        let cstddef = CcInclude::cstddef();
        let memory = CcInclude::memory();
        let a = CcInclude::user_header("a.h".into());
        let b = CcInclude::user_header("b.h".into());
        assert!(cstddef < memory);
        assert!(cstddef < a);
        assert!(cstddef < b);
        assert!(memory < a);
        assert!(memory < b);
        assert!(a < b);
    }

    #[gtest]
    fn test_format_cc_includes() {
        let includes = [
            CcInclude::cstddef(),
            CcInclude::memory(),
            CcInclude::user_header("a.h".into()),
            CcInclude::user_header("b.h".into()),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();

        let tokens = format_cc_includes(&includes);
        let actual =
            cc_tokens_to_formatted_string_for_tests(quote! { __NEWLINE__ #tokens }).unwrap();
        assert_eq!(
            actual,
            r#"
#include <cstddef>
#include <memory>

#include "a.h"
#include "b.h"
"#
        );
    }

    #[gtest]
    fn test_namespace_qualifier_empty() {
        let ns = NamespaceQualifier::new::<&str>([], true);
        let actual_rs = ns.format_for_rs();
        assert!(actual_rs.is_empty());
        let actual_cc = ns.format_for_cc(FlagSet::default()).unwrap();
        assert_cc_matches!(actual_cc, quote! { :: });
    }

    #[gtest]
    fn test_namespace_qualifier_basic() {
        let ns = NamespaceQualifier::new(["foo", "bar"], true);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo::bar:: });
        let actual_cc = ns.format_for_cc(FlagSet::default()).unwrap();
        assert_cc_matches!(actual_cc, quote! { :: foo::bar:: });
    }

    #[gtest]
    fn test_namespace_qualifier_reserved_cc_keyword() {
        let ns = NamespaceQualifier::new(["foo", "impl", "bar"], true);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo :: r#impl :: bar :: });
        let actual_cc = ns.format_for_cc(FlagSet::default()).unwrap();
        assert_cc_matches!(actual_cc, quote! { :: foo::impl::bar:: });
    }

    #[gtest]
    fn test_namespace_qualifier_reserved_rust_keyword() {
        let ns = NamespaceQualifier::new(["foo", "reinterpret_cast", "bar"], true);
        let actual_rs = ns.format_for_rs();
        assert_rs_matches!(actual_rs, quote! { foo :: reinterpret_cast :: bar :: });
        let cc_error = ns.format_for_cc(FlagSet::default()).unwrap_err();
        let msg = cc_error.to_string();
        assert!(msg.contains("`reinterpret_cast`"));
        assert!(msg.contains("C++ reserved word"));
    }

    #[gtest]
    fn test_format_cc_include_support_lib_header() {
        let tests = vec![
            (
                "\"crubit/support/path/for/test/{header}\"",
                "header.h",
                "\"crubit/support/path/for/test/header.h\"",
            ),
            (
                "\"crubit/support/path/for/test/{header}\"",
                "subdir/header.h",
                "\"crubit/support/path/for/test/subdir/header.h\"",
            ),
            (
                "<crubit/support/path/for/test/{header}>",
                "header.h",
                "<crubit/support/path/for/test/header.h>",
            ),
            ("\"{header}\"", "header.h", "\"header.h\""),
        ];

        for (support_path_format, header, expected_output) in tests.iter() {
            let support_path_format =
                Format::parse_with_metavars(support_path_format, &["header"]).unwrap();
            let header =
                CcInclude::support_lib_header(support_path_format, header.to_string().into());
            let mut actual_tokens = TokenStream::default();
            header.to_tokens(&mut actual_tokens);

            let expected_output: TokenStream =
                expected_output.parse().expect("Failed to convert expected_output to TokenStream");

            assert_cc_matches!(
                actual_tokens,
                quote! {
                  __HASH_TOKEN__ include #expected_output
                }
            );
        }
    }

    #[gtest]
    fn test_escape_non_identifier_chars() {
        let tests = vec![
            ("", ""),
            ("foo", "foo"),
            ("0abc", "_x00000030abc"),
            ("abc$xyz", "abc_dxyz"),
            ("abc.xyz", "abc_pxyz"),
            ("abc_xyz", "abc_uxyz"),
            ("abc🦀xyz", "abc_x0001f980xyz"),
            // With an escaping scheme like `$` => "_d", `<utf8 dd80 char>` => "_dd80", the
            // following 2 tests would fail the injectivity requirement (they both would map to
            // "_dd80"):
            ("$d80", "_dd80"),
            ("\u{740}", "_x00000740"),
        ];

        for (input, expected_output) in tests.iter() {
            let actual_output = escape_non_identifier_chars(input);
            assert_eq!(&actual_output, expected_output);
        }

        // Asserting that each distinct, unique test input should result in a unique,
        // non-duplicated output.  (This can be seen as a rather lightweight and
        // indirect verification of the injectivity requirement.)
        let duplicate_expectations =
            tests.iter().map(|(_, expected)| *expected).duplicates().collect_vec();
        let empty_vec: Vec<&'static str> = vec![];
        assert_eq!(empty_vec, duplicate_expectations);
    }

    #[gtest]
    fn test_is_cpp_pointer_type() {
        let tests = vec![
            ("Foo", None),
            ("Foo*", Some(CcConstQualifier::Mut)),
            ("Foo const*", Some(CcConstQualifier::Const)),
            ("const Foo*", Some(CcConstQualifier::Const)),
            ("::foo::bar::Fizz * ", Some(CcConstQualifier::Mut)),
            ("::foo::bar::Fizz const *", Some(CcConstQualifier::Const)),
            ("const ::foo::bar::Fizz *", Some(CcConstQualifier::Const)),
        ];

        for (input, expected_output) in tests.into_iter() {
            let actual_output = is_cpp_pointer_type(input.parse().unwrap());
            assert_eq!(actual_output, expected_output);
        }
    }
}

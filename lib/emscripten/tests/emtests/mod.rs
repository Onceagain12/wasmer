// Rust test file autogenerated with cargo build (build/emtests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.

// The _common module is not autogenerated, as it provides common macros for the emtests
#[macro_use]
mod _common;
mod clock_gettime;
mod emscripten_get_compiler_setting;
mod env;
mod fs_exports;
mod getvalue_setvalue;
mod legacy_exported_runtime_numbers;
mod localtime;
mod modularize_closure_pre;
mod printf;
mod puts;
mod stackalloc;
mod test_addr_of_stacked;
mod test_alloca;
mod test_alloca_stack;
mod test_array2;
mod test_array2b;
mod test_atomic;
mod test_atox;
mod test_bsearch;
mod test_ccall;
mod test_complex;
mod test_demangle_stacks;
mod test_demangle_stacks_noassert;
mod test_dlmalloc_partial_2;
mod test_double_varargs;
mod test_em_asm;
mod test_em_asm_2;
mod test_em_asm_parameter_pack;
mod test_em_asm_signatures;
mod test_em_asm_unicode;
mod test_em_asm_unused_arguments;
mod test_em_js;
mod test_emscripten_api;
mod test_erf;
mod test_errar;
mod test_exceptions_2;
mod test_exceptions_multi;
mod test_exceptions_std;
mod test_exceptions_white_list;
mod test_fast_math;
mod test_flexarray_struct;
mod test_float32_precise;
mod test_float_builtins;
mod test_frexp;
mod test_funcptr;
mod test_funcptr_namecollide;
mod test_funcptrfunc;
mod test_funcs;
mod test_functionpointer_libfunc_varargs;
mod test_fwrite_0;
mod test_getcwd;
mod test_getgep;
mod test_getloadavg;
mod test_getopt;
mod test_getopt_long;
mod test_globaldoubles;
mod test_globals;
mod test_gmtime;
mod test_hello_world;
mod test_i16_emcc_intrinsic;
mod test_i32_mul_precise;
mod test_i64;
mod test_i64_4;
mod test_i64_7z;
mod test_i64_cmp2;
mod test_i64_i16;
mod test_i64_llabs;
mod test_i64_precise;
mod test_i64_precise_needed;
mod test_i64_precise_unneeded;
mod test_i64_qdouble;
mod test_i64_umul;
mod test_i64_varargs;
mod test_i64_zextneg;
mod test_if;
mod test_if_else;
mod test_indirectbr;
mod test_indirectbr_many;
mod test_isnan;
mod test_libcextra;
mod test_libgen;
mod test_literal_negative_zero;
mod test_llrint;
mod test_llvm_fabs;
mod test_llvm_intrinsics;
mod test_llvmswitch;
mod test_longjmp;
mod test_longjmp2;
mod test_longjmp3;
mod test_longjmp4;
mod test_longjmp_exc;
mod test_longjmp_funcptr;
mod test_longjmp_repeat;
mod test_longjmp_stacked;
mod test_longjmp_throw;
mod test_longjmp_unwind;
mod test_loop;
mod test_lower_intrinsics;
mod test_main_thread_async_em_asm;
mod test_mainenv;
mod test_mathfuncptr;
mod test_memcpy2;
mod test_memcpy3;
mod test_memcpy_memcmp;
mod test_memmove;
mod test_memmove2;
mod test_memmove3;
mod test_memset;
mod test_mmap;
mod test_negative_zero;
mod test_nested_struct_varargs;
mod test_nl_types;
mod test_perrar;
mod test_phiundef;
mod test_poll;
mod test_posixtime;
mod test_printf_2;
mod test_printf_more;
mod test_regex;
mod test_relocatable_void_function;
mod test_rounding;
mod test_set_align;
mod test_siglongjmp;
mod test_sintvars;
mod test_sizeof;
mod test_sscanf;
mod test_sscanf_3;
mod test_sscanf_4;
mod test_sscanf_5;
mod test_sscanf_6;
mod test_sscanf_caps;
mod test_sscanf_float;
mod test_sscanf_hex;
mod test_sscanf_n;
mod test_sscanf_other_whitespace;
mod test_sscanf_skip;
mod test_sscanf_whitespace;
mod test_stack_varargs;
mod test_stack_void;
mod test_statvfs;
mod test_std_cout_new;
mod test_strcasecmp;
mod test_strcmp_uni;
mod test_strftime;
mod test_strings;
mod test_strndup;
mod test_strptime_days;
mod test_strptime_reentrant;
mod test_strstr;
mod test_strtod;
mod test_strtok;
mod test_strtol_bin;
mod test_strtol_dec;
mod test_strtol_hex;
mod test_strtol_oct;
mod test_strtold;
mod test_strtoll_bin;
mod test_strtoll_dec;
mod test_strtoll_hex;
mod test_strtoll_oct;
mod test_struct_varargs;
mod test_time_c;
mod test_tracing;
mod test_transtrcase;
mod test_trickystring;
mod test_uname;
mod test_unary_literal;
mod test_utf;
mod test_varargs;
mod test_varargs_multi;
mod test_vprintf;
mod test_vsnprintf;
mod test_wprintf;
mod test_write_stdout_fileno;
mod test_zero_multiplication;
mod test_zerodiv;

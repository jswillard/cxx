use crate::gen::block::Block;
use crate::gen::ifndef;
use crate::gen::out::{Content, OutFile};

#[derive(Default, PartialEq)]
pub struct Builtins<'a> {
    pub panic: bool,
    pub rust_string: bool,
    pub rust_str: bool,
    pub rust_slice: bool,
    pub rust_box: bool,
    pub rust_vec: bool,
    pub rust_fn: bool,
    pub rust_isize: bool,
    pub opaque: bool,
    pub layout: bool,
    pub unsafe_bitcopy: bool,
    pub rust_error: bool,
    pub manually_drop: bool,
    pub maybe_uninit: bool,
    pub trycatch: bool,
    pub ptr_len: bool,
    pub exception: bool,
    pub relocatable: bool,
    pub friend_impl: bool,
    pub is_complete: bool,
    pub deleter_if: bool,
    pub content: Content<'a>,
}

impl<'a> Builtins<'a> {
    pub fn new() -> Self {
        Builtins::default()
    }
}

pub(super) fn write(out: &mut OutFile) {
    if out.builtin == Default::default() {
        return;
    }

    let include = &mut out.include;
    let builtin = &mut out.builtin;
    let out = &mut builtin.content;

    if builtin.rust_string {
        include.array = true;
        include.cstdint = true;
        include.string = true;
    }

    if builtin.rust_str {
        include.cstdint = true;
        include.string = true;
        builtin.friend_impl = true;
    }

    if builtin.rust_vec {
        include.algorithm = true;
        include.array = true;
        include.cstddef = true;
        include.initializer_list = true;
        include.iterator = true;
        include.new = true;
        include.type_traits = true;
        include.utility = true;
        builtin.panic = true;
        builtin.rust_slice = true;
        builtin.unsafe_bitcopy = true;
    }

    if builtin.rust_slice {
        include.cstddef = true;
        include.iterator = true;
        include.type_traits = true;
        builtin.friend_impl = true;
        builtin.layout = true;
        builtin.panic = true;
    }

    if builtin.rust_box {
        include.new = true;
        include.type_traits = true;
        include.utility = true;
    }

    if builtin.rust_fn {
        include.utility = true;
    }

    if builtin.rust_error {
        include.exception = true;
        builtin.friend_impl = true;
    }

    if builtin.rust_isize {
        include.basetsd = true;
        include.sys_types = true;
    }

    if builtin.relocatable {
        include.type_traits = true;
    }

    if builtin.layout {
        include.type_traits = true;
        include.cstddef = true;
        builtin.is_complete = true;
    }

    if builtin.is_complete {
        include.cstddef = true;
        include.type_traits = true;
    }

    out.begin_block(Block::Namespace("rust"));
    out.begin_block(Block::InlineNamespace("cxxbridge1"));
    writeln!(out, "// #include \"rust/cxx.h\"");

    ifndef::write(out, builtin.panic, "CXXBRIDGE1_PANIC");

    if builtin.rust_string {
        out.next_section();
        writeln!(out, "struct unsafe_bitcopy_t;");
    }

    if builtin.friend_impl {
        out.begin_block(Block::AnonymousNamespace);
        writeln!(out, "template <typename T>");
        writeln!(out, "class impl;");
        out.end_block(Block::AnonymousNamespace);
    }

    out.next_section();
    if builtin.rust_str && !builtin.rust_string {
        writeln!(out, "class String;");
    }
    if builtin.layout && !builtin.opaque {
        writeln!(out, "class Opaque;");
    }

    if builtin.rust_slice {
        out.next_section();
        writeln!(out, "template <typename T>");
        writeln!(out, "::std::size_t size_of();");
        writeln!(out, "template <typename T>");
        writeln!(out, "::std::size_t align_of();");
    }

    ifndef::write(out, builtin.rust_string, "CXXBRIDGE1_RUST_STRING");
    ifndef::write(out, builtin.rust_str, "CXXBRIDGE1_RUST_STR");
    ifndef::write(out, builtin.rust_slice, "CXXBRIDGE1_RUST_SLICE");
    ifndef::write(out, builtin.rust_box, "CXXBRIDGE1_RUST_BOX");
    ifndef::write(out, builtin.unsafe_bitcopy, "CXXBRIDGE1_RUST_BITCOPY");
    ifndef::write(out, builtin.rust_vec, "CXXBRIDGE1_RUST_VEC");
    ifndef::write(out, builtin.rust_fn, "CXXBRIDGE1_RUST_FN");
    ifndef::write(out, builtin.rust_error, "CXXBRIDGE1_RUST_ERROR");
    ifndef::write(out, builtin.rust_isize, "CXXBRIDGE1_RUST_ISIZE");
    ifndef::write(out, builtin.opaque, "CXXBRIDGE1_RUST_OPAQUE");
    ifndef::write(out, builtin.is_complete, "CXXBRIDGE1_IS_COMPLETE");
    ifndef::write(out, builtin.layout, "CXXBRIDGE1_LAYOUT");
    ifndef::write(out, builtin.relocatable, "CXXBRIDGE1_RELOCATABLE");

    out.begin_block(Block::Namespace("detail"));

    if builtin.maybe_uninit {
        include.cstddef = true;
        include.new = true;
        out.next_section();
        writeln!(out, "template <typename T, typename = void *>");
        writeln!(out, "struct operator_new {{");
        writeln!(
            out,
            "  void *operator()(::std::size_t sz) {{ return ::operator new(sz); }}",
        );
        writeln!(out, "}};");
        out.next_section();
        writeln!(out, "template <typename T>");
        writeln!(
            out,
            "struct operator_new<T, decltype(T::operator new(sizeof(T)))> {{",
        );
        writeln!(
            out,
            "  void *operator()(::std::size_t sz) {{ return T::operator new(sz); }}",
        );
        writeln!(out, "}};");
    }

    out.end_block(Block::Namespace("detail"));

    if builtin.manually_drop {
        out.next_section();
        include.utility = true;
        writeln!(out, "template <typename T>");
        writeln!(out, "union ManuallyDrop {{");
        writeln!(out, "  T value;");
        writeln!(
            out,
            "  ManuallyDrop(T &&value) : value(::std::move(value)) {{}}",
        );
        writeln!(out, "  ~ManuallyDrop() {{}}");
        writeln!(out, "}};");
    }

    if builtin.maybe_uninit {
        include.cstddef = true;
        out.next_section();
        writeln!(out, "template <typename T>");
        writeln!(out, "union MaybeUninit {{");
        writeln!(out, "  T value;");
        writeln!(
            out,
            "  void *operator new(::std::size_t sz) {{ return detail::operator_new<T>{{}}(sz); }}",
        );
        writeln!(out, "  MaybeUninit() {{}}");
        writeln!(out, "  ~MaybeUninit() {{}}");
        writeln!(out, "}};");
    }

    out.begin_block(Block::AnonymousNamespace);

    if builtin.ptr_len {
        include.cstddef = true;
        out.begin_block(Block::Namespace("repr"));
        writeln!(out, "struct PtrLen final {{");
        writeln!(out, "  void *ptr;");
        writeln!(out, "  ::std::size_t len;");
        writeln!(out, "}};");
        out.end_block(Block::Namespace("repr"));
    }

    if builtin.rust_error {
        out.next_section();
        writeln!(out, "template <>");
        writeln!(out, "class impl<Error> final {{");
        writeln!(out, "public:");
        writeln!(out, "  static Error error(repr::PtrLen repr) noexcept {{");
        writeln!(out, "    Error error;");
        writeln!(out, "    error.msg = static_cast<const char *>(repr.ptr);");
        writeln!(out, "    error.len = repr.len;");
        writeln!(out, "    return error;");
        writeln!(out, "  }}");
        writeln!(out, "}};");
    }

    if builtin.deleter_if {
        out.next_section();
        writeln!(out, "template <bool> struct deleter_if {{");
        writeln!(out, "  template <typename T> void operator()(T *) {{}}");
        writeln!(out, "}};");
        out.next_section();
        writeln!(out, "template <> struct deleter_if<true> {{");
        writeln!(
            out,
            "  template <typename T> void operator()(T *ptr) {{ ptr->~T(); }}",
        );
        writeln!(out, "}};");
    }

    out.end_block(Block::AnonymousNamespace);
    out.end_block(Block::InlineNamespace("cxxbridge1"));

    if builtin.trycatch {
        out.begin_block(Block::Namespace("behavior"));
        include.exception = true;
        include.type_traits = true;
        include.utility = true;
        writeln!(out, "class missing {{}};");
        writeln!(out, "missing trycatch(...);");
        writeln!(out);
        writeln!(out, "template <typename Try, typename Fail>");
        writeln!(out, "static typename ::std::enable_if<");
        writeln!(
            out,
            "    ::std::is_same<decltype(trycatch(::std::declval<Try>(), ::std::declval<Fail>())),",
        );
        writeln!(out, "                 missing>::value>::type");
        writeln!(out, "trycatch(Try &&func, Fail &&fail) noexcept try {{");
        writeln!(out, "  func();");
        writeln!(out, "}} catch (const ::std::exception &e) {{");
        writeln!(out, "  fail(e.what());");
        writeln!(out, "}}");
        out.end_block(Block::Namespace("behavior"));
    }

    out.end_block(Block::Namespace("rust"));

    if builtin.exception {
        include.cstddef = true;
        out.begin_block(Block::ExternC);
        writeln!(
            out,
            "const char *cxxbridge1$exception(const char *, ::std::size_t);",
        );
        out.end_block(Block::ExternC);
    }
}

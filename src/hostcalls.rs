//!

use crate::Wasi;
use lucet_runtime::c_api::lucet_result_tag_name;
use lucet_runtime::{lucet_hostcall, lucet_hostcall_terminate, vmctx::Vmctx};
use std::{mem, rc::Rc};
use wasi_common::{wasi, wasi32, WasiCtx};

// misc

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_proc_exit(_lucet_ctx: &mut Vmctx, rval: wasi::__wasi_exitcode_t) -> ! {
    export_wasi_funcs();
    lucet_hostcall_terminate!(rval);
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_proc_raise(
    lucet_ctx: &mut Vmctx,
    sig: wasi::__wasi_signal_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.proc_raise(heap, sig)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_args_get(
    lucet_ctx: &mut Vmctx,
    argv_ptr: wasi32::uintptr_t,
    argv_buf: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.args_get(heap, argv_ptr, argv_buf)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_args_sizes_get(
    lucet_ctx: &mut Vmctx,
    argc_ptr: wasi32::uintptr_t,
    argv_buf_size_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.args_sizes_get(heap, argc_ptr, argv_buf_size_ptr)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_clock_res_get(
    lucet_ctx: &mut Vmctx,
    clock_id: wasi::__wasi_clockid_t,
    resolution_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.clock_res_get(heap, clock_id, resolution_ptr)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_clock_time_get(
    lucet_ctx: &mut Vmctx,
    clock_id: wasi::__wasi_clockid_t,
    precision: wasi::__wasi_timestamp_t,
    time_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.clock_time_get(heap, clock_id, precision, time_ptr)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_environ_get(
    lucet_ctx: &mut Vmctx,
    environ_ptr: wasi32::uintptr_t,
    environ_buf: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.environ_get(heap, environ_ptr, environ_buf)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_environ_sizes_get(
    lucet_ctx: &mut Vmctx,
    environ_count_ptr: wasi32::uintptr_t,
    environ_size_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.environ_sizes_get(heap, environ_count_ptr, environ_size_ptr)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_sched_yield(lucet_ctx: &mut Vmctx) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.sched_yield(heap)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_poll_oneoff(
    lucet_ctx: &mut Vmctx,
    input: wasi32::uintptr_t,
    output: wasi32::uintptr_t,
    nsubscriptions: wasi32::size_t,
    nevents: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.poll_oneoff(heap, input, output, nsubscriptions, nevents)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_random_get(
    lucet_ctx: &mut Vmctx,
    buf_ptr: wasi32::uintptr_t,
    buf_len: wasi32::size_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.random_get(heap, buf_ptr, buf_len)
}

// file

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_allocate(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    offset: wasi::__wasi_filesize_t,
    len: wasi::__wasi_filesize_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_allocate(heap, fd, offset, len)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_advise(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    offset: wasi::__wasi_filesize_t,
    len: wasi::__wasi_filesize_t,
    advice: wasi::__wasi_advice_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_advise(heap, fd, offset, len, advice)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_datasync(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_datasync(heap, fd)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_sync(lucet_ctx: &mut Vmctx, fd: wasi::__wasi_fd_t) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_sync(heap, fd)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_tell(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    offset: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_tell(heap, fd, offset)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_seek(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    offset: wasi::__wasi_filedelta_t,
    whence: wasi::__wasi_whence_t,
    newoffset: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_seek(heap, fd, offset, whence, newoffset)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_read(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    iovs_ptr: wasi32::uintptr_t,
    iovs_len: wasi32::size_t,
    nread: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_read(heap, fd, iovs_ptr, iovs_len, nread)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_write(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    iovs_ptr: wasi32::uintptr_t,
    iovs_len: wasi32::size_t,
    nwritten: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_write(heap, fd, iovs_ptr, iovs_len, nwritten)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_close(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_close(heap, fd)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_pread(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    iovs_ptr: wasi32::uintptr_t,
    iovs_len: wasi32::size_t,
    offset: wasi::__wasi_filesize_t,
    nread: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_pread(heap, fd, iovs_ptr, iovs_len, offset, nread)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_pwrite(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    iovs_ptr: wasi32::uintptr_t,
    iovs_len: wasi32::size_t,
    offset: wasi::__wasi_filesize_t,
    nwritten: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_pwrite(heap, fd, iovs_ptr, iovs_len, offset, nwritten)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_fdstat_get(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    fdstat_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_fdstat_get(heap, fd, fdstat_ptr)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_fdstat_set_flags(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    fdflags: wasi::__wasi_fdflags_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_fdstat_set_flags(heap, fd, fdflags)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_fdstat_set_rights(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    fs_rights_base: wasi::__wasi_rights_t,
    fs_rights_inheriting: wasi::__wasi_rights_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_fdstat_set_rights(heap, fd, fs_rights_base, fs_rights_inheriting)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_filestat_get(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    filestat_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_filestat_get(heap, fd, filestat_ptr)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_filestat_set_size(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    st_size: wasi::__wasi_filesize_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_filestat_set_size(heap, fd, st_size)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_filestat_set_times(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    st_atim: wasi::__wasi_timestamp_t,
    st_mtim: wasi::__wasi_timestamp_t,
    fst_flags: wasi::__wasi_fstflags_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_filestat_set_times(heap, fd, st_atim, st_mtim, fst_flags)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_prestat_get(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    prestat_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_prestat_get(heap, fd, prestat_ptr)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_prestat_dir_name(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    path_ptr: wasi32::uintptr_t,
    path_len: wasi32::size_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_prestat_dir_name(heap, fd, path_ptr, path_len)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_readdir(
    lucet_ctx: &mut Vmctx,
    fd: wasi::__wasi_fd_t,
    buf: wasi32::uintptr_t,
    buf_len: wasi32::size_t,
    cookie: wasi::__wasi_dircookie_t,
    bufused: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_readdir(heap, fd, buf, buf_len, cookie, bufused)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_fd_renumber(
    lucet_ctx: &mut Vmctx,
    from: wasi::__wasi_fd_t,
    to: wasi::__wasi_fd_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.fd_renumber(heap, from, to)
}

// path

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_open(
    lucet_ctx: &mut Vmctx,
    dirfd: wasi::__wasi_fd_t,
    dirflags: wasi::__wasi_lookupflags_t,
    path_ptr: wasi32::uintptr_t,
    path_len: wasi32::size_t,
    oflags: wasi::__wasi_oflags_t,
    fs_rights_base: wasi::__wasi_rights_t,
    fs_rights_inheriting: wasi::__wasi_rights_t,
    fs_flags: wasi::__wasi_fdflags_t,
    fd_out_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &mut lucet_ctx.get_embed_ctx_mut::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_open(
        heap,
        dirfd,
        dirflags,
        path_ptr,
        path_len,
        oflags,
        fs_rights_base,
        fs_rights_inheriting,
        fs_flags,
        fd_out_ptr,
    )
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_link(
    lucet_ctx: &mut Vmctx,
    old_fd: wasi::__wasi_fd_t,
    old_flags: wasi::__wasi_lookupflags_t,
    old_path_ptr: wasi32::uintptr_t,
    old_path_len: wasi32::size_t,
    new_fd: wasi::__wasi_fd_t,
    new_path_ptr: wasi32::uintptr_t,
    new_path_len: wasi32::size_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_link(
        heap,
        old_fd,
        old_flags,
        old_path_ptr,
        old_path_len,
        new_fd,
        new_path_ptr,
        new_path_len,
    )
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_rename(
    lucet_ctx: &mut Vmctx,
    old_dirfd: wasi::__wasi_fd_t,
    old_path_ptr: wasi32::uintptr_t,
    old_path_len: wasi32::size_t,
    new_dirfd: wasi::__wasi_fd_t,
    new_path_ptr: wasi32::uintptr_t,
    new_path_len: wasi32::size_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_rename(
        heap,
        old_dirfd,
        old_path_ptr,
        old_path_len,
        new_dirfd,
        new_path_ptr,
        new_path_len,
    )
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_symlink(
    lucet_ctx: &mut Vmctx,
    old_path_ptr: wasi32::uintptr_t,
    old_path_len: wasi32::size_t,
    dir_fd: wasi::__wasi_fd_t,
    new_path_ptr: wasi32::uintptr_t,
    new_path_len: wasi32::size_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_symlink(
        heap,
        old_path_ptr,
        old_path_len,
        dir_fd,
        new_path_ptr,
        new_path_len,
    )
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_unlink_file(
    lucet_ctx: &mut Vmctx,
    dirfd: wasi::__wasi_fd_t,
    path_ptr: wasi32::uintptr_t,
    path_len: wasi32::size_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_unlink_file(heap, dirfd, path_ptr, path_len)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_filestat_get(
    lucet_ctx: &mut Vmctx,
    dirfd: wasi::__wasi_fd_t,
    dirflags: wasi::__wasi_lookupflags_t,
    path_ptr: wasi32::uintptr_t,
    path_len: wasi32::size_t,
    filestat_ptr: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_filestat_get(heap, dirfd, dirflags, path_ptr, path_len, filestat_ptr)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_filestat_set_times(
    lucet_ctx: &mut Vmctx,
    dirfd: wasi::__wasi_fd_t,
    dirflags: wasi::__wasi_lookupflags_t,
    path_ptr: wasi32::uintptr_t,
    path_len: wasi32::size_t,
    st_atim: wasi::__wasi_timestamp_t,
    st_mtim: wasi::__wasi_timestamp_t,
    fst_flags: wasi::__wasi_fstflags_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_filestat_set_times(
        heap, dirfd, dirflags, path_ptr, path_len, st_atim, st_mtim, fst_flags,
    )
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_readlink(
    lucet_ctx: &mut Vmctx,
    dirfd: wasi::__wasi_fd_t,
    path_ptr: wasi32::uintptr_t,
    path_len: wasi32::size_t,
    buf_ptr: wasi32::uintptr_t,
    buf_len: wasi32::size_t,
    bufused: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_readlink(heap, dirfd, path_ptr, path_len, buf_ptr, buf_len, bufused)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_create_directory(
    lucet_ctx: &mut Vmctx,
    dirfd: wasi::__wasi_fd_t,
    path_ptr: wasi32::uintptr_t,
    path_len: wasi32::size_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_create_directory(heap, dirfd, path_ptr, path_len)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_path_remove_directory(
    lucet_ctx: &mut Vmctx,
    dirfd: wasi::__wasi_fd_t,
    path_ptr: wasi32::uintptr_t,
    path_len: wasi32::size_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.path_remove_directory(heap, dirfd, path_ptr, path_len)
}

// sockets

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_sock_recv(
    lucet_ctx: &mut Vmctx,
    sock: wasi::__wasi_fd_t,
    ri_data: wasi32::uintptr_t,
    ri_data_len: wasi32::size_t,
    ri_flags: wasi::__wasi_riflags_t,
    ro_datalen: wasi32::uintptr_t,
    ro_flags: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.sock_recv(
        heap,
        sock,
        ri_data,
        ri_data_len,
        ri_flags,
        ro_datalen,
        ro_flags,
    )
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_sock_send(
    lucet_ctx: &mut Vmctx,
    sock: wasi::__wasi_fd_t,
    si_data: wasi32::uintptr_t,
    si_data_len: wasi32::size_t,
    si_flags: wasi::__wasi_siflags_t,
    so_datalen: wasi32::uintptr_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.sock_send(heap, sock, si_data, si_data_len, si_flags, so_datalen)
}

#[lucet_hostcall]
#[no_mangle]
pub unsafe fn __wasi_sock_shutdown(
    lucet_ctx: &mut Vmctx,
    sock: wasi::__wasi_fd_t,
    how: wasi::__wasi_sdflags_t,
) -> wasi::__wasi_errno_t {
    let wasi_ctx = &lucet_ctx.get_embed_ctx::<Box<dyn Wasi>>();
    let heap = &mut lucet_ctx.heap_mut();
    wasi_ctx.sock_shutdown(heap, sock, how)
}

pub fn export_wasi_funcs() {
    let funcs: &[*const extern "C" fn()] = &[
        __wasi_args_get as _,
        __wasi_args_sizes_get as _,
        __wasi_clock_res_get as _,
        __wasi_clock_time_get as _,
        __wasi_environ_get as _,
        __wasi_environ_sizes_get as _,
        __wasi_fd_advise as _,
        __wasi_fd_allocate as _,
        __wasi_fd_close as _,
        __wasi_fd_datasync as _,
        __wasi_fd_fdstat_get as _,
        __wasi_fd_fdstat_set_flags as _,
        __wasi_fd_fdstat_set_rights as _,
        __wasi_fd_filestat_get as _,
        __wasi_fd_filestat_set_size as _,
        __wasi_fd_filestat_set_times as _,
        __wasi_fd_pread as _,
        __wasi_fd_prestat_get as _,
        __wasi_fd_prestat_dir_name as _,
        __wasi_fd_pwrite as _,
        __wasi_fd_read as _,
        __wasi_fd_readdir as _,
        __wasi_fd_renumber as _,
        __wasi_fd_seek as _,
        __wasi_fd_sync as _,
        __wasi_fd_tell as _,
        __wasi_fd_write as _,
        __wasi_path_create_directory as _,
        __wasi_path_filestat_get as _,
        __wasi_path_filestat_set_times as _,
        __wasi_path_link as _,
        __wasi_path_open as _,
        __wasi_path_readlink as _,
        __wasi_path_remove_directory as _,
        __wasi_path_rename as _,
        __wasi_path_unlink_file as _,
        __wasi_path_symlink as _,
        __wasi_poll_oneoff as _,
        __wasi_proc_exit as _,
        __wasi_proc_raise as _,
        __wasi_random_get as _,
        __wasi_sched_yield as _,
        __wasi_sock_recv as _,
        __wasi_sock_send as _,
        __wasi_sock_shutdown as _,
    ];
    mem::forget(Rc::new(funcs));
}

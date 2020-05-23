use crate::Error;
use lucet_runtime::vmctx::Vmctx;
use wasi_common::{hostcalls as wasi_hostcalls, wasi, wasi32, WasiCtx};

///
pub trait Wasi {
    ///
    fn proc_raise(&self, heap: &mut [u8], sig: wasi::__wasi_signal_t) -> wasi::__wasi_errno_t;

    ///
    fn args_get(
        &self,
        heap: &mut [u8],
        argv_ptr: wasi32::uintptr_t,
        argv_buf: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn args_sizes_get(
        &self,
        heap: &mut [u8],
        argc_ptr: wasi32::uintptr_t,
        argv_buf_size_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn clock_res_get(
        &self,
        heap: &mut [u8],
        clock_id: wasi::__wasi_clockid_t,
        resolution_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn clock_time_get(
        &self,
        heap: &mut [u8],
        clock_id: wasi::__wasi_clockid_t,
        precision: wasi::__wasi_timestamp_t,
        time_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn environ_get(
        &self,
        heap: &mut [u8],
        environ_ptr: wasi32::uintptr_t,
        environ_buf: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn environ_sizes_get(
        &self,
        heap: &mut [u8],
        environ_count_ptr: wasi32::uintptr_t,
        environ_size_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn sched_yield(&self, heap: &mut [u8]) -> wasi::__wasi_errno_t;

    ///
    fn poll_oneoff(
        &self,
        heap: &mut [u8],
        input: wasi32::uintptr_t,
        output: wasi32::uintptr_t,
        nsubscriptions: wasi32::size_t,
        nevents: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn random_get(
        &self,
        heap: &mut [u8],
        buf_ptr: wasi32::uintptr_t,
        buf_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t;

    // file

    ///
    fn fd_allocate(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        offset: wasi::__wasi_filesize_t,
        len: wasi::__wasi_filesize_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_advise(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        offset: wasi::__wasi_filesize_t,
        len: wasi::__wasi_filesize_t,
        advice: wasi::__wasi_advice_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_datasync(&self, heap: &mut [u8], fd: wasi::__wasi_fd_t) -> wasi::__wasi_errno_t;

    ///
    fn fd_sync(&self, heap: &mut [u8], fd: wasi::__wasi_fd_t) -> wasi::__wasi_errno_t;

    ///
    fn fd_tell(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        offset: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_seek(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        offset: wasi::__wasi_filedelta_t,
        whence: wasi::__wasi_whence_t,
        newoffset: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_read(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        iovs_ptr: wasi32::uintptr_t,
        iovs_len: wasi32::size_t,
        nread: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_write(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        iovs_ptr: wasi32::uintptr_t,
        iovs_len: wasi32::size_t,
        nwritten: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_close(&mut self, heap: &mut [u8], fd: wasi::__wasi_fd_t) -> wasi::__wasi_errno_t;

    ///
    fn fd_pread(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        iovs_ptr: wasi32::uintptr_t,
        iovs_len: wasi32::size_t,
        offset: wasi::__wasi_filesize_t,
        nread: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_pwrite(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        iovs_ptr: wasi32::uintptr_t,
        iovs_len: wasi32::size_t,
        offset: wasi::__wasi_filesize_t,
        nwritten: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_fdstat_get(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        fdstat_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_fdstat_set_flags(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        fdflags: wasi::__wasi_fdflags_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_fdstat_set_rights(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        fs_rights_base: wasi::__wasi_rights_t,
        fs_rights_inheriting: wasi::__wasi_rights_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_filestat_get(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        filestat_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_filestat_set_size(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        st_size: wasi::__wasi_filesize_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_filestat_set_times(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        st_atim: wasi::__wasi_timestamp_t,
        st_mtim: wasi::__wasi_timestamp_t,
        fst_flags: wasi::__wasi_fstflags_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_prestat_get(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        prestat_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_prestat_dir_name(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_readdir(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        buf: wasi32::uintptr_t,
        buf_len: wasi32::size_t,
        cookie: wasi::__wasi_dircookie_t,
        bufused: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn fd_renumber(
        &mut self,
        heap: &mut [u8],
        from: wasi::__wasi_fd_t,
        to: wasi::__wasi_fd_t,
    ) -> wasi::__wasi_errno_t;

    // path

    ///
    fn path_open(
        &mut self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        dirflags: wasi::__wasi_lookupflags_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
        oflags: wasi::__wasi_oflags_t,
        fs_rights_base: wasi::__wasi_rights_t,
        fs_rights_inheriting: wasi::__wasi_rights_t,
        fs_flags: wasi::__wasi_fdflags_t,
        fd_out_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_link(
        &self,
        heap: &mut [u8],
        old_fd: wasi::__wasi_fd_t,
        old_flags: wasi::__wasi_lookupflags_t,
        old_path_ptr: wasi32::uintptr_t,
        old_path_len: wasi32::size_t,
        new_fd: wasi::__wasi_fd_t,
        new_path_ptr: wasi32::uintptr_t,
        new_path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_rename(
        &self,
        heap: &mut [u8],
        old_dirfd: wasi::__wasi_fd_t,
        old_path_ptr: wasi32::uintptr_t,
        old_path_len: wasi32::size_t,
        new_dirfd: wasi::__wasi_fd_t,
        new_path_ptr: wasi32::uintptr_t,
        new_path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_symlink(
        &self,
        heap: &mut [u8],
        old_path_ptr: wasi32::uintptr_t,
        old_path_len: wasi32::size_t,
        dir_fd: wasi::__wasi_fd_t,
        new_path_ptr: wasi32::uintptr_t,
        new_path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_unlink_file(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_filestat_get(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        dirflags: wasi::__wasi_lookupflags_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
        filestat_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_filestat_set_times(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        dirflags: wasi::__wasi_lookupflags_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
        st_atim: wasi::__wasi_timestamp_t,
        st_mtim: wasi::__wasi_timestamp_t,
        fst_flags: wasi::__wasi_fstflags_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_readlink(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
        buf_ptr: wasi32::uintptr_t,
        buf_len: wasi32::size_t,
        bufused: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_create_directory(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn path_remove_directory(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t;

    // sockets

    ///
    fn sock_recv(
        &self,
        heap: &mut [u8],
        sock: wasi::__wasi_fd_t,
        ri_data: wasi32::uintptr_t,
        ri_data_len: wasi32::size_t,
        ri_flags: wasi::__wasi_riflags_t,
        ro_datalen: wasi32::uintptr_t,
        ro_flags: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn sock_send(
        &self,
        heap: &mut [u8],
        sock: wasi::__wasi_fd_t,
        si_data: wasi32::uintptr_t,
        si_data_len: wasi32::size_t,
        si_flags: wasi::__wasi_siflags_t,
        so_datalen: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t;

    ///
    fn sock_shutdown(
        &self,
        heap: &mut [u8],
        sock: wasi::__wasi_fd_t,
        how: wasi::__wasi_sdflags_t,
    ) -> wasi::__wasi_errno_t;
}

impl Wasi for WasiCtx {
    fn proc_raise(&self, heap: &mut [u8], sig: wasi::__wasi_signal_t) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::proc_raise(self, heap, sig) }
    }

    fn args_get(
        &self,
        heap: &mut [u8],
        argv_ptr: wasi32::uintptr_t,
        argv_buf: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::args_get(self, heap, argv_ptr, argv_buf) }
    }

    fn args_sizes_get(
        &self,
        heap: &mut [u8],
        argc_ptr: wasi32::uintptr_t,
        argv_buf_size_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::args_sizes_get(self, heap, argc_ptr, argv_buf_size_ptr) }
    }

    fn clock_res_get(
        &self,
        heap: &mut [u8],
        clock_id: wasi::__wasi_clockid_t,
        resolution_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::clock_res_get(heap, clock_id, resolution_ptr) }
    }

    fn clock_time_get(
        &self,
        heap: &mut [u8],
        clock_id: wasi::__wasi_clockid_t,
        precision: wasi::__wasi_timestamp_t,
        time_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::clock_time_get(heap, clock_id, precision, time_ptr) }
    }

    fn environ_get(
        &self,
        heap: &mut [u8],
        environ_ptr: wasi32::uintptr_t,
        environ_buf: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::environ_get(self, heap, environ_ptr, environ_buf) }
    }

    fn environ_sizes_get(
        &self,
        heap: &mut [u8],
        environ_count_ptr: wasi32::uintptr_t,
        environ_size_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::environ_sizes_get(self, heap, environ_count_ptr, environ_size_ptr)
        }
    }

    fn sched_yield(&self, _heap: &mut [u8]) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::sched_yield() }
    }

    fn poll_oneoff(
        &self,
        heap: &mut [u8],
        input: wasi32::uintptr_t,
        output: wasi32::uintptr_t,
        nsubscriptions: wasi32::size_t,
        nevents: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::poll_oneoff(self, heap, input, output, nsubscriptions, nevents) }
    }

    fn random_get(
        &self,
        heap: &mut [u8],
        buf_ptr: wasi32::uintptr_t,
        buf_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::random_get(heap, buf_ptr, buf_len) }
    }

    // file

    fn fd_allocate(
        &self,
        _heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        offset: wasi::__wasi_filesize_t,
        len: wasi::__wasi_filesize_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_allocate(self, fd, offset, len) }
    }

    fn fd_advise(
        &self,
        _heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        offset: wasi::__wasi_filesize_t,
        len: wasi::__wasi_filesize_t,
        advice: wasi::__wasi_advice_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_advise(self, fd, offset, len, advice) }
    }

    fn fd_datasync(&self, _heap: &mut [u8], fd: wasi::__wasi_fd_t) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_datasync(self, fd) }
    }

    fn fd_sync(&self, _heap: &mut [u8], fd: wasi::__wasi_fd_t) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_sync(self, fd) }
    }

    fn fd_tell(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        offset: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_tell(self, heap, fd, offset) }
    }

    fn fd_seek(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        offset: wasi::__wasi_filedelta_t,
        whence: wasi::__wasi_whence_t,
        newoffset: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_seek(self, heap, fd, offset, whence, newoffset) }
    }

    fn fd_read(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        iovs_ptr: wasi32::uintptr_t,
        iovs_len: wasi32::size_t,
        nread: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_read(self, heap, fd, iovs_ptr, iovs_len, nread) }
    }

    fn fd_write(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        iovs_ptr: wasi32::uintptr_t,
        iovs_len: wasi32::size_t,
        nwritten: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_write(self, heap, fd, iovs_ptr, iovs_len, nwritten) }
    }

    fn fd_close(&mut self, _heap: &mut [u8], fd: wasi::__wasi_fd_t) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_close(self, fd) }
    }

    fn fd_pread(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        iovs_ptr: wasi32::uintptr_t,
        iovs_len: wasi32::size_t,
        offset: wasi::__wasi_filesize_t,
        nread: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_pread(self, heap, fd, iovs_ptr, iovs_len, offset, nread) }
    }

    fn fd_pwrite(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        iovs_ptr: wasi32::uintptr_t,
        iovs_len: wasi32::size_t,
        offset: wasi::__wasi_filesize_t,
        nwritten: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_pwrite(self, heap, fd, iovs_ptr, iovs_len, offset, nwritten) }
    }

    fn fd_fdstat_get(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        fdstat_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_fdstat_get(self, heap, fd, fdstat_ptr) }
    }

    fn fd_fdstat_set_flags(
        &self,
        _heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        fdflags: wasi::__wasi_fdflags_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_fdstat_set_flags(self, fd, fdflags) }
    }

    fn fd_fdstat_set_rights(
        &mut self,
        _heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        fs_rights_base: wasi::__wasi_rights_t,
        fs_rights_inheriting: wasi::__wasi_rights_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::fd_fdstat_set_rights(self, fd, fs_rights_base, fs_rights_inheriting)
        }
    }

    fn fd_filestat_get(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        filestat_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_filestat_get(self, heap, fd, filestat_ptr) }
    }

    fn fd_filestat_set_size(
        &self,
        _heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        st_size: wasi::__wasi_filesize_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_filestat_set_size(self, fd, st_size) }
    }

    fn fd_filestat_set_times(
        &self,
        _heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        st_atim: wasi::__wasi_timestamp_t,
        st_mtim: wasi::__wasi_timestamp_t,
        fst_flags: wasi::__wasi_fstflags_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_filestat_set_times(self, fd, st_atim, st_mtim, fst_flags) }
    }

    fn fd_prestat_get(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        prestat_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_prestat_get(self, heap, fd, prestat_ptr) }
    }

    fn fd_prestat_dir_name(
        &self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_prestat_dir_name(self, heap, fd, path_ptr, path_len) }
    }

    fn fd_readdir(
        &mut self,
        heap: &mut [u8],
        fd: wasi::__wasi_fd_t,
        buf: wasi32::uintptr_t,
        buf_len: wasi32::size_t,
        cookie: wasi::__wasi_dircookie_t,
        bufused: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_readdir(self, heap, fd, buf, buf_len, cookie, bufused) }
    }

    fn fd_renumber(
        &mut self,
        _heap: &mut [u8],
        from: wasi::__wasi_fd_t,
        to: wasi::__wasi_fd_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::fd_renumber(self, from, to) }
    }

    // path

    fn path_open(
        &mut self,
        heap: &mut [u8],
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
        unsafe {
            wasi_hostcalls::path_open(
                self,
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
    }

    fn path_link(
        &self,
        heap: &mut [u8],
        old_dirfd: wasi::__wasi_fd_t,
        old_flags: wasi::__wasi_lookupflags_t,
        old_path_ptr: wasi32::uintptr_t,
        old_path_len: wasi32::size_t,
        new_dirfd: wasi::__wasi_fd_t,
        new_path_ptr: wasi32::uintptr_t,
        new_path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::path_link(
                self,
                heap,
                old_dirfd,
                old_flags,
                old_path_ptr,
                old_path_len,
                new_dirfd,
                new_path_ptr,
                new_path_len,
            )
        }
    }

    fn path_rename(
        &self,
        heap: &mut [u8],
        old_dirfd: wasi::__wasi_fd_t,
        old_path_ptr: wasi32::uintptr_t,
        old_path_len: wasi32::size_t,
        new_dirfd: wasi::__wasi_fd_t,
        new_path_ptr: wasi32::uintptr_t,
        new_path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::path_rename(
                self,
                heap,
                old_dirfd,
                old_path_ptr,
                old_path_len,
                new_dirfd,
                new_path_ptr,
                new_path_len,
            )
        }
    }

    fn path_symlink(
        &self,
        heap: &mut [u8],
        old_path_ptr: wasi32::uintptr_t,
        old_path_len: wasi32::size_t,
        dir_fd: wasi::__wasi_fd_t,
        new_path_ptr: wasi32::uintptr_t,
        new_path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::path_symlink(
                self,
                heap,
                old_path_ptr,
                old_path_len,
                dir_fd,
                new_path_ptr,
                new_path_len,
            )
        }
    }

    fn path_unlink_file(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::path_unlink_file(self, heap, dirfd, path_ptr, path_len) }
    }

    fn path_filestat_get(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        dirflags: wasi::__wasi_lookupflags_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
        filestat_ptr: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::path_filestat_get(
                self,
                heap,
                dirfd,
                dirflags,
                path_ptr,
                path_len,
                filestat_ptr,
            )
        }
    }

    fn path_filestat_set_times(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        dirflags: wasi::__wasi_lookupflags_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
        st_atim: wasi::__wasi_timestamp_t,
        st_mtim: wasi::__wasi_timestamp_t,
        fst_flags: wasi::__wasi_fstflags_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::path_filestat_set_times(
                self, heap, dirflags, dirflags, path_ptr, path_len, st_atim, st_mtim, fst_flags,
            )
        }
    }

    fn path_readlink(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
        buf_ptr: wasi32::uintptr_t,
        buf_len: wasi32::size_t,
        bufused: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::path_readlink(
                self, heap, dirfd, path_ptr, path_len, buf_ptr, buf_len, bufused,
            )
        }
    }

    fn path_create_directory(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::path_create_directory(self, heap, dirfd, path_ptr, path_len) }
    }

    fn path_remove_directory(
        &self,
        heap: &mut [u8],
        dirfd: wasi::__wasi_fd_t,
        path_ptr: wasi32::uintptr_t,
        path_len: wasi32::size_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::path_remove_directory(self, heap, dirfd, path_ptr, path_len) }
    }

    // sockets

    fn sock_recv(
        &self,
        heap: &mut [u8],
        sock: wasi::__wasi_fd_t,
        ri_data: wasi32::uintptr_t,
        ri_data_len: wasi32::size_t,
        ri_flags: wasi::__wasi_riflags_t,
        ro_datalen: wasi32::uintptr_t,
        ro_flags: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::sock_recv(
                self,
                heap,
                sock,
                ri_data,
                ri_data_len,
                ri_flags,
                ro_datalen,
                ro_flags,
            )
        }
    }

    fn sock_send(
        &self,
        heap: &mut [u8],
        sock: wasi::__wasi_fd_t,
        si_data: wasi32::uintptr_t,
        si_data_len: wasi32::size_t,
        si_flags: wasi::__wasi_siflags_t,
        so_datalen: wasi32::uintptr_t,
    ) -> wasi::__wasi_errno_t {
        unsafe {
            wasi_hostcalls::sock_send(self, heap, sock, si_data, si_data_len, si_flags, so_datalen)
        }
    }

    fn sock_shutdown(
        &self,
        heap: &mut [u8],
        sock: wasi::__wasi_fd_t,
        how: wasi::__wasi_sdflags_t,
    ) -> wasi::__wasi_errno_t {
        unsafe { wasi_hostcalls::sock_shutdown(self, heap, sock, how) }
    }
}

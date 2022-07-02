// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// THIS FILE IS AUTOGENERATED BY CARGO-LIBBPF-GEN!

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(clippy::transmute_ptr_to_ref)]
#![allow(clippy::upper_case_acronyms)]

use libbpf_rs::libbpf_sys;

fn build_skel_config() -> libbpf_rs::Result<libbpf_rs::skeleton::ObjectSkeletonConfig<'static>> {
    let mut builder = libbpf_rs::skeleton::ObjectSkeletonConfigBuilder::new(DATA);
    builder
        .name("sys_enter_bpf")
        .map("syscall_count", false)
        .prog("sys_enter");

    builder.build()
}

#[derive(Default)]
pub struct SysEnterSkelBuilder {
    pub obj_builder: libbpf_rs::ObjectBuilder,
}

impl<'a> SysEnterSkelBuilder {
    pub fn open(mut self) -> libbpf_rs::Result<OpenSysEnterSkel<'a>> {
        let mut skel_config = build_skel_config()?;
        let open_opts = self.obj_builder.opts(std::ptr::null());

        let ret = unsafe { libbpf_sys::bpf_object__open_skeleton(skel_config.get(), &open_opts) };
        if ret != 0 {
            return Err(libbpf_rs::Error::System(-ret));
        }

        let obj = unsafe { libbpf_rs::OpenObject::from_ptr(skel_config.object_ptr())? };

        Ok(OpenSysEnterSkel { obj, skel_config })
    }
}

pub struct OpenSysEnterMaps<'a> {
    inner: &'a libbpf_rs::OpenObject,
}

impl<'a> OpenSysEnterMaps<'a> {
    pub fn syscall_count(&self) -> &libbpf_rs::OpenMap {
        self.inner.map("syscall_count").unwrap()
    }
}

pub struct OpenSysEnterMapsMut<'a> {
    inner: &'a mut libbpf_rs::OpenObject,
}

impl<'a> OpenSysEnterMapsMut<'a> {
    pub fn syscall_count(&mut self) -> &mut libbpf_rs::OpenMap {
        self.inner.map_mut("syscall_count").unwrap()
    }
}

pub struct OpenSysEnterProgs<'a> {
    inner: &'a libbpf_rs::OpenObject,
}

impl<'a> OpenSysEnterProgs<'a> {
    pub fn sys_enter(&self) -> &libbpf_rs::OpenProgram {
        self.inner.prog("sys_enter").unwrap()
    }
}

pub struct OpenSysEnterProgsMut<'a> {
    inner: &'a mut libbpf_rs::OpenObject,
}

impl<'a> OpenSysEnterProgsMut<'a> {
    pub fn sys_enter(&mut self) -> &mut libbpf_rs::OpenProgram {
        self.inner.prog_mut("sys_enter").unwrap()
    }
}

pub struct OpenSysEnterSkel<'a> {
    pub obj: libbpf_rs::OpenObject,
    skel_config: libbpf_rs::skeleton::ObjectSkeletonConfig<'a>,
}

impl<'a> OpenSysEnterSkel<'a> {
    pub fn load(mut self) -> libbpf_rs::Result<SysEnterSkel<'a>> {
        let ret = unsafe { libbpf_sys::bpf_object__load_skeleton(self.skel_config.get()) };
        if ret != 0 {
            return Err(libbpf_rs::Error::System(-ret));
        }

        let obj = unsafe { libbpf_rs::Object::from_ptr(self.obj.take_ptr())? };

        Ok(SysEnterSkel {
            obj,
            skel_config: self.skel_config,
            links: SysEnterLinks::default(),
        })
    }

    pub fn progs(&self) -> OpenSysEnterProgs {
        OpenSysEnterProgs { inner: &self.obj }
    }

    pub fn progs_mut(&mut self) -> OpenSysEnterProgsMut {
        OpenSysEnterProgsMut {
            inner: &mut self.obj,
        }
    }

    pub fn maps(&self) -> OpenSysEnterMaps {
        OpenSysEnterMaps { inner: &self.obj }
    }

    pub fn maps_mut(&mut self) -> OpenSysEnterMapsMut {
        OpenSysEnterMapsMut {
            inner: &mut self.obj,
        }
    }
}

pub struct SysEnterMaps<'a> {
    inner: &'a libbpf_rs::Object,
}

impl<'a> SysEnterMaps<'a> {
    pub fn syscall_count(&self) -> &libbpf_rs::Map {
        self.inner.map("syscall_count").unwrap()
    }
}

pub struct SysEnterMapsMut<'a> {
    inner: &'a mut libbpf_rs::Object,
}

impl<'a> SysEnterMapsMut<'a> {
    pub fn syscall_count(&mut self) -> &mut libbpf_rs::Map {
        self.inner.map_mut("syscall_count").unwrap()
    }
}

pub struct SysEnterProgs<'a> {
    inner: &'a libbpf_rs::Object,
}

impl<'a> SysEnterProgs<'a> {
    pub fn sys_enter(&self) -> &libbpf_rs::Program {
        self.inner.prog("sys_enter").unwrap()
    }
}

pub struct SysEnterProgsMut<'a> {
    inner: &'a mut libbpf_rs::Object,
}

impl<'a> SysEnterProgsMut<'a> {
    pub fn sys_enter(&mut self) -> &mut libbpf_rs::Program {
        self.inner.prog_mut("sys_enter").unwrap()
    }
}

#[derive(Default)]
pub struct SysEnterLinks {
    pub sys_enter: Option<libbpf_rs::Link>,
}

pub struct SysEnterSkel<'a> {
    pub obj: libbpf_rs::Object,
    skel_config: libbpf_rs::skeleton::ObjectSkeletonConfig<'a>,
    pub links: SysEnterLinks,
}

unsafe impl<'a> Send for SysEnterSkel<'a> {}

impl<'a> SysEnterSkel<'a> {
    pub fn progs(&self) -> SysEnterProgs {
        SysEnterProgs { inner: &self.obj }
    }

    pub fn progs_mut(&mut self) -> SysEnterProgsMut {
        SysEnterProgsMut {
            inner: &mut self.obj,
        }
    }

    pub fn maps(&self) -> SysEnterMaps {
        SysEnterMaps { inner: &self.obj }
    }

    pub fn maps_mut(&mut self) -> SysEnterMapsMut {
        SysEnterMapsMut {
            inner: &mut self.obj,
        }
    }

    pub fn attach(&mut self) -> libbpf_rs::Result<()> {
        let ret = unsafe { libbpf_sys::bpf_object__attach_skeleton(self.skel_config.get()) };
        if ret != 0 {
            return Err(libbpf_rs::Error::System(-ret));
        }

        self.links = SysEnterLinks {
            sys_enter: (|| {
                let ptr = self.skel_config.prog_link_ptr(0)?;
                if ptr.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(unsafe { libbpf_rs::Link::from_ptr(ptr) }))
                }
            })()?,
        };

        Ok(())
    }
}

const DATA: &[u8] = &[
    127, 69, 76, 70, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 247, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 64,
    0, 21, 0, 1, 0, 183, 1, 0, 0, 0, 0, 0, 0, 99, 26, 252, 255, 0, 0, 0, 0, 183, 6, 0, 0, 1, 0, 0,
    0, 123, 106, 240, 255, 0, 0, 0, 0, 191, 162, 0, 0, 0, 0, 0, 0, 7, 2, 0, 0, 252, 255, 255, 255,
    24, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 133, 0, 0, 0, 1, 0, 0, 0, 85, 0, 9, 0, 0, 0,
    0, 0, 191, 162, 0, 0, 0, 0, 0, 0, 7, 2, 0, 0, 252, 255, 255, 255, 191, 163, 0, 0, 0, 0, 0, 0,
    7, 3, 0, 0, 240, 255, 255, 255, 24, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 4, 0, 0,
    0, 0, 0, 0, 133, 0, 0, 0, 2, 0, 0, 0, 5, 0, 1, 0, 0, 0, 0, 0, 219, 96, 0, 0, 0, 0, 0, 0, 183,
    0, 0, 0, 0, 0, 0, 0, 149, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0,
    0, 2, 0, 48, 159, 32, 0, 0, 0, 0, 0, 0, 0, 176, 0, 0, 0, 0, 0, 0, 0, 2, 0, 122, 12, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 2, 0, 49,
    159, 80, 0, 0, 0, 0, 0, 0, 0, 152, 0, 0, 0, 0, 0, 0, 0, 2, 0, 122, 0, 152, 0, 0, 0, 0, 0, 0, 0,
    160, 0, 0, 0, 0, 0, 0, 0, 2, 0, 49, 159, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0,
    0, 0, 0, 0, 0, 0, 144, 0, 0, 0, 0, 0, 0, 0, 1, 0, 80, 152, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0,
    0, 0, 0, 0, 1, 0, 80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 17, 1, 37, 14, 19, 5,
    3, 14, 16, 23, 27, 14, 17, 1, 18, 6, 0, 0, 2, 52, 0, 3, 14, 73, 19, 63, 25, 58, 11, 59, 11, 2,
    24, 0, 0, 3, 19, 1, 11, 11, 58, 11, 59, 11, 0, 0, 4, 13, 0, 3, 14, 73, 19, 58, 11, 59, 11, 56,
    11, 0, 0, 5, 15, 0, 73, 19, 0, 0, 6, 1, 1, 73, 19, 0, 0, 7, 33, 0, 73, 19, 55, 11, 0, 0, 8, 36,
    0, 3, 14, 62, 11, 11, 11, 0, 0, 9, 36, 0, 3, 14, 11, 11, 62, 11, 0, 0, 10, 22, 0, 73, 19, 3,
    14, 58, 11, 59, 11, 0, 0, 11, 52, 0, 3, 14, 73, 19, 58, 11, 59, 11, 0, 0, 12, 21, 1, 73, 19,
    39, 25, 0, 0, 13, 5, 0, 73, 19, 0, 0, 14, 15, 0, 0, 0, 15, 38, 0, 0, 0, 16, 4, 1, 73, 19, 11,
    11, 58, 11, 59, 5, 0, 0, 17, 40, 0, 3, 14, 28, 15, 0, 0, 18, 46, 1, 17, 1, 18, 6, 64, 24, 151,
    66, 25, 3, 14, 58, 11, 59, 11, 73, 19, 63, 25, 0, 0, 19, 52, 0, 2, 23, 3, 14, 58, 11, 59, 11,
    73, 19, 0, 0, 0, 166, 1, 0, 0, 4, 0, 0, 0, 0, 0, 8, 1, 0, 0, 0, 0, 12, 0, 21, 0, 0, 0, 0, 0, 0,
    0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 176, 0, 0, 0, 2, 90, 0, 0, 0, 63, 0, 0, 0, 1, 10, 9, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 32, 1, 5, 4, 104, 0, 0, 0, 116, 0, 0, 0, 1, 6, 0, 4, 133, 0, 0, 0,
    147, 0, 0, 0, 1, 7, 8, 4, 160, 0, 0, 0, 181, 0, 0, 0, 1, 8, 16, 4, 199, 0, 0, 0, 215, 0, 0, 0,
    1, 9, 24, 0, 5, 121, 0, 0, 0, 6, 133, 0, 0, 0, 7, 140, 0, 0, 0, 2, 0, 8, 109, 0, 0, 0, 5, 4, 9,
    113, 0, 0, 0, 8, 7, 5, 152, 0, 0, 0, 10, 163, 0, 0, 0, 156, 0, 0, 0, 2, 115, 10, 174, 0, 0, 0,
    150, 0, 0, 0, 2, 107, 8, 137, 0, 0, 0, 7, 4, 5, 186, 0, 0, 0, 10, 197, 0, 0, 0, 195, 0, 0, 0,
    2, 12, 10, 208, 0, 0, 0, 189, 0, 0, 0, 2, 10, 8, 166, 0, 0, 0, 7, 8, 5, 220, 0, 0, 0, 6, 133,
    0, 0, 0, 7, 140, 0, 0, 0, 1, 0, 11, 211, 0, 0, 0, 243, 0, 0, 0, 3, 51, 5, 248, 0, 0, 0, 12, 8,
    1, 0, 0, 13, 8, 1, 0, 0, 13, 9, 1, 0, 0, 0, 14, 5, 14, 1, 0, 0, 15, 11, 231, 0, 0, 0, 26, 1, 0,
    0, 3, 73, 5, 31, 1, 0, 0, 12, 57, 1, 0, 0, 13, 8, 1, 0, 0, 13, 9, 1, 0, 0, 13, 9, 1, 0, 0, 13,
    197, 0, 0, 0, 0, 8, 251, 0, 0, 0, 5, 8, 16, 174, 0, 0, 0, 4, 2, 141, 161, 17, 4, 1, 0, 0, 0,
    17, 12, 1, 0, 0, 1, 17, 24, 1, 0, 0, 2, 17, 34, 1, 0, 0, 4, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 176,
    0, 0, 0, 1, 90, 45, 1, 0, 0, 1, 15, 133, 0, 0, 0, 19, 0, 0, 0, 0, 133, 0, 0, 0, 1, 16, 152, 0,
    0, 0, 19, 56, 0, 0, 0, 55, 1, 0, 0, 1, 17, 186, 0, 0, 0, 19, 132, 0, 0, 0, 64, 1, 0, 0, 1, 17,
    181, 0, 0, 0, 0, 0, 99, 108, 97, 110, 103, 32, 118, 101, 114, 115, 105, 111, 110, 32, 49, 51,
    46, 48, 46, 49, 0, 98, 112, 102, 47, 115, 121, 115, 99, 97, 108, 108, 115, 47, 115, 121, 115,
    95, 101, 110, 116, 101, 114, 46, 98, 112, 102, 46, 99, 0, 47, 104, 111, 109, 101, 47, 108, 111,
    115, 104, 122, 47, 100, 101, 118, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 108,
    111, 115, 104, 122, 47, 108, 101, 109, 117, 114, 115, 0, 115, 121, 115, 99, 97, 108, 108, 95,
    99, 111, 117, 110, 116, 0, 116, 121, 112, 101, 0, 105, 110, 116, 0, 95, 95, 65, 82, 82, 65, 89,
    95, 83, 73, 90, 69, 95, 84, 89, 80, 69, 95, 95, 0, 107, 101, 121, 0, 117, 110, 115, 105, 103,
    110, 101, 100, 32, 105, 110, 116, 0, 95, 95, 117, 51, 50, 0, 117, 51, 50, 0, 118, 97, 108, 117,
    101, 0, 108, 111, 110, 103, 32, 108, 111, 110, 103, 32, 117, 110, 115, 105, 103, 110, 101, 100,
    32, 105, 110, 116, 0, 95, 95, 117, 54, 52, 0, 117, 54, 52, 0, 109, 97, 120, 95, 101, 110, 116,
    114, 105, 101, 115, 0, 98, 112, 102, 95, 109, 97, 112, 95, 108, 111, 111, 107, 117, 112, 95,
    101, 108, 101, 109, 0, 98, 112, 102, 95, 109, 97, 112, 95, 117, 112, 100, 97, 116, 101, 95,
    101, 108, 101, 109, 0, 108, 111, 110, 103, 32, 105, 110, 116, 0, 66, 80, 70, 95, 65, 78, 89, 0,
    66, 80, 70, 95, 78, 79, 69, 88, 73, 83, 84, 0, 66, 80, 70, 95, 69, 88, 73, 83, 84, 0, 66, 80,
    70, 95, 70, 95, 76, 79, 67, 75, 0, 115, 121, 115, 95, 101, 110, 116, 101, 114, 0, 105, 110,
    105, 116, 95, 118, 97, 108, 0, 99, 111, 117, 110, 116, 0, 159, 235, 1, 0, 24, 0, 0, 0, 0, 0, 0,
    0, 76, 1, 0, 0, 76, 1, 0, 0, 220, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 1, 4, 0, 0, 0, 32, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0,
    2, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 1, 4, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 6, 0, 0,
    0, 25, 0, 0, 0, 0, 0, 0, 8, 7, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 8, 8, 0, 0, 0, 35, 0, 0, 0, 0, 0,
    0, 1, 4, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 10, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 8,
    11, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 8, 12, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 1, 8, 0, 0, 0, 64, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 2, 0, 0, 0, 4,
    0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 4, 32, 0, 0, 0, 81, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    86, 0, 0, 0, 5, 0, 0, 0, 64, 0, 0, 0, 90, 0, 0, 0, 9, 0, 0, 0, 128, 0, 0, 0, 96, 0, 0, 0, 13,
    0, 0, 0, 192, 0, 0, 0, 108, 0, 0, 0, 0, 0, 0, 14, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    13, 2, 0, 0, 0, 122, 0, 0, 0, 1, 0, 0, 12, 17, 0, 0, 0, 214, 1, 0, 0, 1, 0, 0, 15, 0, 0, 0, 0,
    16, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 105, 110, 116, 0, 95, 95, 65, 82, 82, 65, 89, 95, 83,
    73, 90, 69, 95, 84, 89, 80, 69, 95, 95, 0, 117, 51, 50, 0, 95, 95, 117, 51, 50, 0, 117, 110,
    115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 0, 117, 54, 52, 0, 95, 95, 117, 54, 52, 0,
    108, 111, 110, 103, 32, 108, 111, 110, 103, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32,
    105, 110, 116, 0, 116, 121, 112, 101, 0, 107, 101, 121, 0, 118, 97, 108, 117, 101, 0, 109, 97,
    120, 95, 101, 110, 116, 114, 105, 101, 115, 0, 115, 121, 115, 99, 97, 108, 108, 95, 99, 111,
    117, 110, 116, 0, 115, 121, 115, 95, 101, 110, 116, 101, 114, 0, 116, 114, 97, 99, 101, 112,
    111, 105, 110, 116, 47, 114, 97, 119, 95, 115, 121, 115, 99, 97, 108, 108, 115, 47, 115, 121,
    115, 95, 101, 110, 116, 101, 114, 0, 47, 104, 111, 109, 101, 47, 108, 111, 115, 104, 122, 47,
    100, 101, 118, 47, 103, 105, 116, 104, 117, 98, 46, 99, 111, 109, 47, 108, 111, 115, 104, 122,
    47, 108, 101, 109, 117, 114, 115, 47, 46, 47, 98, 112, 102, 47, 115, 121, 115, 99, 97, 108,
    108, 115, 47, 115, 121, 115, 95, 101, 110, 116, 101, 114, 46, 98, 112, 102, 46, 99, 0, 105,
    110, 116, 32, 115, 121, 115, 95, 101, 110, 116, 101, 114, 40, 41, 32, 123, 0, 32, 32, 117, 51,
    50, 32, 107, 101, 121, 32, 61, 32, 48, 59, 0, 32, 32, 117, 54, 52, 32, 105, 110, 105, 116, 95,
    118, 97, 108, 32, 61, 32, 49, 44, 32, 42, 99, 111, 117, 110, 116, 59, 0, 32, 32, 99, 111, 117,
    110, 116, 32, 61, 32, 98, 112, 102, 95, 109, 97, 112, 95, 108, 111, 111, 107, 117, 112, 95,
    101, 108, 101, 109, 40, 38, 115, 121, 115, 99, 97, 108, 108, 95, 99, 111, 117, 110, 116, 44,
    32, 38, 107, 101, 121, 41, 59, 0, 32, 32, 105, 102, 32, 40, 33, 99, 111, 117, 110, 116, 41, 32,
    123, 0, 32, 32, 32, 32, 98, 112, 102, 95, 109, 97, 112, 95, 117, 112, 100, 97, 116, 101, 95,
    101, 108, 101, 109, 40, 38, 115, 121, 115, 99, 97, 108, 108, 95, 99, 111, 117, 110, 116, 44,
    32, 38, 107, 101, 121, 44, 32, 38, 105, 110, 105, 116, 95, 118, 97, 108, 44, 32, 66, 80, 70,
    95, 65, 78, 89, 41, 59, 0, 32, 32, 95, 95, 115, 121, 110, 99, 95, 102, 101, 116, 99, 104, 95,
    97, 110, 100, 95, 97, 100, 100, 40, 99, 111, 117, 110, 116, 44, 32, 49, 41, 59, 0, 125, 0, 46,
    109, 97, 112, 115, 0, 159, 235, 1, 0, 32, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 20, 0, 0, 0, 172,
    0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 132, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0,
    0, 16, 0, 0, 0, 132, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 166, 0, 0, 0, 237, 0, 0, 0, 0, 60, 0, 0,
    8, 0, 0, 0, 166, 0, 0, 0, 255, 0, 0, 0, 7, 64, 0, 0, 24, 0, 0, 0, 166, 0, 0, 0, 14, 1, 0, 0, 7,
    68, 0, 0, 40, 0, 0, 0, 166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 166, 0, 0, 0, 42, 1,
    0, 0, 11, 76, 0, 0, 72, 0, 0, 0, 166, 0, 0, 0, 95, 1, 0, 0, 7, 80, 0, 0, 88, 0, 0, 0, 166, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 0, 0, 0, 166, 0, 0, 0, 111, 1, 0, 0, 5, 84, 0, 0, 152, 0, 0,
    0, 166, 0, 0, 0, 178, 1, 0, 0, 3, 96, 0, 0, 160, 0, 0, 0, 166, 0, 0, 0, 212, 1, 0, 0, 1, 108,
    0, 0, 0, 12, 0, 0, 0, 255, 255, 255, 255, 4, 0, 8, 0, 8, 124, 11, 0, 20, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 176, 0, 0, 0, 0, 0, 0, 0, 188, 0, 0, 0, 4, 0, 122, 0, 0, 0, 8, 1, 1,
    251, 14, 13, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 46, 47, 98, 112, 102, 47, 115, 121, 115, 99,
    97, 108, 108, 115, 0, 46, 47, 98, 112, 102, 0, 47, 116, 109, 112, 47, 46, 116, 109, 112, 85,
    89, 56, 109, 109, 86, 47, 98, 112, 102, 47, 115, 114, 99, 47, 98, 112, 102, 0, 0, 115, 121,
    115, 95, 101, 110, 116, 101, 114, 46, 98, 112, 102, 46, 99, 0, 1, 0, 0, 118, 109, 108, 105,
    110, 117, 120, 46, 104, 0, 2, 0, 0, 98, 112, 102, 95, 104, 101, 108, 112, 101, 114, 95, 100,
    101, 102, 115, 46, 104, 0, 3, 0, 0, 0, 0, 9, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 14, 1, 5, 7, 10, 33,
    47, 6, 3, 111, 32, 5, 11, 6, 3, 19, 46, 5, 7, 61, 6, 3, 108, 32, 5, 5, 6, 3, 21, 74, 6, 3, 107,
    74, 5, 3, 6, 3, 24, 32, 5, 1, 35, 2, 2, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 177, 0, 0, 0, 4, 0, 241, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 225, 0, 0, 0,
    0, 0, 3, 0, 152, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 218, 0, 0, 0, 0, 0, 3, 0, 160, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 3, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 15, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 93, 0, 0, 0, 18, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 176, 0, 0, 0, 0, 0,
    0, 0, 34, 0, 0, 0, 17, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0,
    0, 0, 0, 0, 1, 0, 0, 0, 11, 0, 0, 0, 112, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 11, 0, 0, 0, 6, 0,
    0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 6, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 18,
    0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 9, 0, 0, 0,
    26, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0,
    0, 43, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 11, 0,
    0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7,
    0, 0, 0, 92, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 104, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0,
    7, 0, 0, 0, 134, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 141, 0, 0, 0, 0, 0, 0, 0, 3, 0,
    0, 0, 7, 0, 0, 0, 157, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 168, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 7, 0, 0, 0, 175, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 191, 0, 0, 0, 0, 0,
    0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 202, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 209, 0, 0, 0,
    0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 233, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 16, 1, 0,
    0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 58, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 74, 1,
    0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 80, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 86,
    1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 92, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0,
    99, 1, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 113, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0,
    0, 124, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 5, 0, 0, 0, 128, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 7,
    0, 0, 0, 139, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 5, 0, 0, 0, 143, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0,
    0, 7, 0, 0, 0, 154, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 5, 0, 0, 0, 158, 1, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 7, 0, 0, 0, 92, 1, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 11, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0,
    4, 0, 0, 0, 2, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 80, 0, 0, 0, 0, 0, 0,
    0, 4, 0, 0, 0, 2, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 112, 0, 0, 0, 0, 0,
    0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 144, 0, 0, 0,
    0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 176, 0,
    0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0,
    208, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 8, 0, 0,
    0, 24, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 135, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0,
    0, 0, 10, 11, 0, 46, 100, 101, 98, 117, 103, 95, 97, 98, 98, 114, 101, 118, 0, 46, 116, 101,
    120, 116, 0, 46, 114, 101, 108, 46, 66, 84, 70, 46, 101, 120, 116, 0, 115, 121, 115, 99, 97,
    108, 108, 95, 99, 111, 117, 110, 116, 0, 46, 109, 97, 112, 115, 0, 46, 100, 101, 98, 117, 103,
    95, 115, 116, 114, 0, 46, 114, 101, 108, 116, 114, 97, 99, 101, 112, 111, 105, 110, 116, 47,
    114, 97, 119, 95, 115, 121, 115, 99, 97, 108, 108, 115, 47, 115, 121, 115, 95, 101, 110, 116,
    101, 114, 0, 46, 114, 101, 108, 46, 100, 101, 98, 117, 103, 95, 105, 110, 102, 111, 0, 46, 108,
    108, 118, 109, 95, 97, 100, 100, 114, 115, 105, 103, 0, 46, 114, 101, 108, 46, 100, 101, 98,
    117, 103, 95, 108, 105, 110, 101, 0, 46, 114, 101, 108, 46, 100, 101, 98, 117, 103, 95, 102,
    114, 97, 109, 101, 0, 46, 100, 101, 98, 117, 103, 95, 108, 111, 99, 0, 115, 121, 115, 95, 101,
    110, 116, 101, 114, 46, 98, 112, 102, 46, 99, 0, 46, 115, 116, 114, 116, 97, 98, 0, 46, 115,
    121, 109, 116, 97, 98, 0, 46, 114, 101, 108, 46, 66, 84, 70, 0, 76, 66, 66, 48, 95, 51, 0, 76,
    66, 66, 48, 95, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 193, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 2, 15, 0, 0, 0, 0, 0, 0, 232, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 1, 0, 0, 0, 6, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 176, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 11, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 20,
    0, 0, 0, 3, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 1, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 166, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 1, 0, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 202, 1, 0, 0, 0, 0, 0, 0, 221, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 107, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 167, 2, 0, 0, 0, 0, 0, 0,
    170, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 103, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 11, 0, 0, 0,
    0, 0, 0, 48, 2, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 8, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0,
    0, 0, 0, 0, 54, 0, 0, 0, 1, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 81, 4, 0,
    0, 0, 0, 0, 0, 70, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 213, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    151, 5, 0, 0, 0, 0, 0, 0, 64, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 209, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 16, 14, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 11, 0, 0, 0, 8, 0, 0, 0,
    0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 215, 8, 0, 0, 0, 0, 0, 0, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 32, 14, 0, 0, 0, 0, 0, 0, 176, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 13, 0,
    0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 184, 9, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 149, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 208, 14, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 20,
    0, 0, 0, 15, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 137, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 9, 0, 0, 0, 0, 0, 0, 192, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 133, 0, 0, 0,
    9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 240, 14, 0, 0, 0, 0, 0, 0, 16, 0,
    0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 17, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0,
    119, 0, 0, 0, 3, 76, 255, 111, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0,
    0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 201, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160,
    10, 0, 0, 0, 0, 0, 0, 32, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 10, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0,
    24, 0, 0, 0, 0, 0, 0, 0,
];

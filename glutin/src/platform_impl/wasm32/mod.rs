#![cfg(target_arch = "wasm32")]

use crate::{Api, ContextError, CreationError, GlAttributes};
use crate::{PixelFormat, PixelFormatRequirements};

use winit;

pub enum Context {
    WindowedContext, //(winit::Window),
}

impl Context {
    #[inline]
    pub fn new(
        wb: winit::WindowBuilder,
        el: &winit::EventsLoop,
        _pf_reqs: &PixelFormatRequirements,
        _gl_attr: &GlAttributes<&Context>,
    ) -> Result<(winit::Window, Self), CreationError> {
        let window = wb.build(el)?;

        Ok((window, Context::WindowedContext))
    }

    #[inline]
    pub fn new_context(
        el: &winit::EventsLoop,
        pf_reqs: &PixelFormatRequirements,
        gl_attr: &GlAttributes<&Context>,
    ) -> Result<Self, CreationError> {
        let wb = winit::WindowBuilder::new().with_visibility(false);
        Self::new(wb, el, pf_reqs, gl_attr).map(|(_w, c)| match c {
            _ => panic!(),
        })
    }

    /// See the docs in the crate root file.
    #[inline]
    pub fn new_separated(
        _window: &winit::Window,
        _el: &winit::EventsLoop,
        _pf_reqs: &PixelFormatRequirements,
        _gl_attr: &GlAttributes<&Context>,
    ) -> Result<Self, CreationError> {
        unimplemented!()
    }

    #[inline]
    pub fn resize(&self, _width: u32, _height: u32) {
        match self {
            Context::WindowedContext => unreachable!(),
        }
    }

    #[inline]
    pub unsafe fn make_current(&self) -> Result<(), ContextError> {
        Ok(())
    }

    #[inline]
    pub fn is_current(&self) -> bool {
        true
    }

    #[inline]
    pub fn get_proc_address(&self, _addr: &str) -> *const () {
        0 as *const _
    }

    #[inline]
    pub fn swap_buffers(&self) -> Result<(), ContextError> {
        Ok(())
    }

    #[inline]
    pub fn get_api(&self) -> Api {
        Api::WebGl
    }

    #[inline]
    pub fn get_pixel_format(&self) -> PixelFormat {
        // FIXME: this is a dummy pixel format
        PixelFormat {
            hardware_accelerated: true,
            color_bits: 24,
            alpha_bits: 8,
            depth_bits: 24,
            stencil_bits: 8,
            stereoscopy: false,
            double_buffer: true,
            multisampling: None,
            srgb: true,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
    }
}

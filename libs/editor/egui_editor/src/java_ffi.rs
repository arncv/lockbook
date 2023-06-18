use crate::{wgpu, CompositeAlphaMode, Editor, WgpuEditor};
use core::ffi::c_void;
use egui::{Context, Visuals};
use egui_wgpu_backend::ScreenDescriptor;
use jni::objects::{JString, JClass};
use jni::sys::{jfloat, jlong, jobject, jstring};
use jni::JNIEnv;
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, HasRawDisplayHandle, HasRawWindowHandle,
    RawDisplayHandle, RawWindowHandle,
};
use std::ffi::{c_char, CStr};
use std::time::Instant;

#[no_mangle]
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditor_createWgpuCanvas(
    mut env: JNIEnv, _: JClass, surface: jobject, content: JString, scale_factor: jfloat, dark_mode: bool,
) -> jlong {
    let native_window = NativeWindow::new(&env, surface);
    let backends = wgpu::Backends::VULKAN;
    let instance_desc = wgpu::InstanceDescriptor { backends, ..Default::default() };
    let instance = wgpu::Instance::new(instance_desc);
    let surface = unsafe { instance.create_surface(&native_window).unwrap() };
    let (adapter, device, queue) =
        pollster::block_on(request_device(&instance, backends, &surface));
    let format = surface.get_capabilities(&adapter).formats[0];
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: native_window.get_width(),
        height: native_window.get_height(),
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
    };
    surface.configure(&device, &config);
    let rpass = egui_wgpu_backend::RenderPass::new(&device, format, 1);

    let context = Context::default();
    context.set_visuals(if dark_mode { Visuals::dark() } else { Visuals::light() });
    let mut editor = Editor::default();
    editor.set_font(&context);

    let content: String = match env
        .get_string(&content) {
        Ok(cont) => cont.into(),
        Err(err) => format!("# The error is: {:?}", err)
    };
    editor.buffer = content.as_str().into();

    let start_time = Instant::now();
    let mut obj = WgpuEditor {
        start_time,
        device,
        queue,
        surface,
        adapter,
        rpass,
        screen: ScreenDescriptor {
            physical_width: native_window.get_width(),
            physical_height: native_window.get_height(),
            scale_factor,
        },
        context,
        raw_input: Default::default(),
        from_egui: None,
        from_host: None,
        editor,
    };

    obj.frame();

    Box::into_raw(Box::new(obj)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditor_enterFrame(
    mut _env: JNIEnv, _: JClass, obj: jlong,
) {
    let obj = unsafe { &mut *(obj as *mut WgpuEditor) };
    obj.frame();
}

#[no_mangle]
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditor_setText(
    mut env: JNIEnv, _: JClass, obj: jlong, content: JString,
) {
    let obj = unsafe { &mut *(obj as *mut WgpuEditor) };

    let content: String = match env.get_string(&content) {
        Ok(cont) => cont.into(),
        Err(err) => format!("# The error is: {:?}", err)
    };
    obj.editor.buffer = content.as_str().into();
    obj.frame();
}

#[no_mangle]
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditor_dropWgpuCanvas(
    mut _env: JNIEnv, _: JClass, obj: jlong,
) {
    let _obj: Box<WgpuEditor> = unsafe { Box::from_raw(obj as *mut _) };
}

async fn request_device(
    instance: &wgpu::Instance, backend: wgpu::Backends, surface: &wgpu::Surface,
) -> (wgpu::Adapter, wgpu::Device, wgpu::Queue) {
    let adapter =
        wgpu::util::initialize_adapter_from_env_or_default(instance, backend, Some(surface))
            .await
            .expect("No suitable GPU adapters found on the system!");
    let adapter_info = adapter.get_info();
    println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);
    let res = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: adapter.features(),
                limits: adapter.limits(),
            },
            None,
        )
        .await;
    match res {
        Err(err) => {
            panic!("request_device failed: {:?}", err);
        }
        Ok((device, queue)) => (adapter, device, queue),
    }
}

pub struct NativeWindow {
    a_native_window: *mut ndk_sys::ANativeWindow,
}

impl NativeWindow {
    fn new(env: &JNIEnv, surface: jobject) -> Self {
        let a_native_window = unsafe {
            ndk_sys::ANativeWindow_fromSurface(env.get_raw() as *mut _, surface as *mut _)
        };
        Self { a_native_window }
    }

    pub fn get_raw_window(&self) -> *mut ndk_sys::ANativeWindow {
        self.a_native_window
    }

    fn get_width(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getWidth(self.a_native_window) as u32 }
    }

    fn get_height(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getHeight(self.a_native_window) as u32 }
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        unsafe {
            ndk_sys::ANativeWindow_release(self.a_native_window);
        }
    }
}

unsafe impl HasRawWindowHandle for NativeWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = AndroidNdkWindowHandle::empty();
        handle.a_native_window = self.a_native_window as *mut _ as *mut c_void;
        RawWindowHandle::AndroidNdk(handle)
    }
}

unsafe impl HasRawDisplayHandle for NativeWindow {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::Android(AndroidDisplayHandle::empty())
    }
}

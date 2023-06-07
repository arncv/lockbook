use std::time::Instant;
use egui::{Context, Visuals};
use egui_wgpu_backend::ScreenDescriptor;
use jni::JNIEnv;
use jni::objects::JString;
use jni::sys::jstring;

use jni::objects::JClass;
use jni::sys::{jint, jlong, jobject};
use crate::{CompositeAlphaMode, Editor, wgpu, WgpuEditor};
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, HasRawDisplayHandle, HasRawWindowHandle,
    RawDisplayHandle, RawWindowHandle,
};
use std::ffi::c_void;

#[no_mangle]
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditorKt_hello<'local>(
    mut env: JNIEnv<'local>) -> jstring {

    println!("STEP 1");

    let output = env.new_string("Hello!".to_string())
        .expect("Couldn't create java string!");

    println!("STEP 2");

    output.into_raw()
}

pub struct NativeWindow {
    a_native_window: *mut ndk_sys::ANativeWindow,
}

#[no_mangle]
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditorKt_createWgpuCanvas(env: *mut JNIEnv, _: JClass, vulkan_surface: jobject, idx: jint) -> jlong {
    let native_window = NativeWindow::new(env, vulkan_surface);

    let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
    let instance_desc = wgpu::InstanceDescriptor { backends, ..Default::default() };
    let instance = wgpu::Instance::new(instance_desc);
    let surface = unsafe { instance.create_surface(&native_window).unwrap() };
    let (adapter, device, queue) =
        pollster::block_on(request_device(&instance, backends, &surface));
    let format = surface.get_capabilities(&adapter).formats[0];
    let screen =
        ScreenDescriptor { physical_width: 1000, physical_height: 1000, scale_factor: 1.0 };
    let surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: native_window.get_width(), // TODO get from context or something
        height: native_window.get_height(),
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![],
    };
    surface.configure(&device, &surface_config);
    let rpass = egui_wgpu_backend::RenderPass::new(&device, format, 1);

    let context = Context::default();
    context.set_visuals(Visuals::light());
    let mut editor = Editor::default();
    editor.set_font(&context);

    let start_time = Instant::now();
    let mut obj = WgpuEditor {
        start_time,
        device,
        queue,
        surface,
        adapter,
        rpass,
        screen,
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
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditorKt_enterFrame(_env: *mut JNIEnv, _: JClass, obj: jlong) {
    let obj = unsafe { &mut *(obj as *mut WgpuEditor) };
    obj.frame();
}

#[no_mangle]
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditorKt_dropWgpuCanvas(_env: *mut JNIEnv, _: JClass, obj: jlong) {
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

impl NativeWindow {
    fn new(env: *mut JNIEnv, surface: jobject) -> Self {
        let a_native_window = unsafe {
            ndk_sys::ANativeWindow_fromSurface(env as *mut _, surface as *mut _)
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
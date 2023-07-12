use crate::{CompositeAlphaMode, Editor, wgpu, WgpuEditor};
use crate::android::window::NativeWindow;
use egui::{Context, Visuals};
use egui_wgpu_backend::ScreenDescriptor;
use jni::objects::{JClass, JString};
use jni::sys::{jfloat, jint, jlong, jobject, jstring};
use jni::JNIEnv;
use std::time::Instant;
use crate::android::window;
use crate::input::cursor::Cursor;

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
        pollster::block_on(window::request_device(&instance, backends, &surface));
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

#[no_mangle]
pub extern "system" fn Java_app_lockbook_egui_1editor_EGUIEditor_getTextBeforeCursor(
    mut env: JNIEnv, _: JClass, obj: jlong, n: jint,
) -> jstring {
    let obj = unsafe { &mut *(obj as *mut WgpuEditor) };

    let cursor: Cursor = (
        obj.editor.buffer.current.cursor.selection.0.0 - n,
        obj.editor.buffer.current.cursor.selection.0
    )
        .into();

    let buffer = &obj.editor.buffer.current;
    let text = cursor.selection_text(buffer);

    env.new_string(text)
        .expect("Couldn't create JString from rust string!")
        .into_inner()
}




// override fun getTextBeforeCursor(n: Int, flags: Int): CharSequence? {
//         TODO("Not yet implemented")
//     }
//
//     override fun getTextAfterCursor(n: Int, flags: Int): CharSequence? {
//         TODO("Not yet implemented")
//     }
//
//     override fun getSelectedText(flags: Int): CharSequence {
//         TODO("Not yet implemented")
//     }
//
//     override fun getCursorCapsMode(reqModes: Int): Int {
//         TODO("Not yet implemented")
//     }
//
//     override fun getExtractedText(request: ExtractedTextRequest?, flags: Int): ExtractedText {
//         TODO("Not yet implemented")
//     }
//
//     override fun deleteSurroundingText(beforeLength: Int, afterLength: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun deleteSurroundingTextInCodePoints(beforeLength: Int, afterLength: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun setComposingText(text: CharSequence?, newCursorPosition: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun setComposingRegion(start: Int, end: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun finishComposingText(): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun commitText(text: CharSequence?, newCursorPosition: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun commitCompletion(text: CompletionInfo?): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun commitCorrection(correctionInfo: CorrectionInfo?): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun setSelection(start: Int, end: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun performEditorAction(editorAction: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun performContextMenuAction(id: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun beginBatchEdit(): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun endBatchEdit(): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun sendKeyEvent(event: KeyEvent?): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun clearMetaKeyStates(states: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun reportFullscreenMode(enabled: Boolean): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun performPrivateCommand(action: String?, data: Bundle?): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun requestCursorUpdates(cursorUpdateMode: Int): Boolean {
//         TODO("Not yet implemented")
//     }
//
//     override fun getHandler(): Handler {
//         TODO("Not yet implemented")
//     }
//
//     override fun closeConnection() {
//         TODO("Not yet implemented")
//     }
//
//     override fun commitContent(
//         inputContentInfo: InputContentInfo,
//         flags: Int,
//         opts: Bundle?
//     ): Boolean {
//         TODO("Not yet implemented")
//     }
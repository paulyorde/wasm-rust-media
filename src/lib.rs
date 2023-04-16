use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, Navigator};
use web_sys::media_devices::{MediaStreamConstraints, MediaDevices};
use js_sys::Promise;

#[wasm_bindgen]
pub async fn start_recording() -> Result<JsValue, JsValue> {
    let navigator = web_sys::window().unwrap().navigator();
    let media_devices = navigator.media_devices().unwrap();

    let constraints = MediaStreamConstraints::new()
        .audio(&JsValue::TRUE)
        .video(&JsValue::FALSE);

    let media_stream_promise: Promise = media_devices.get_user_media_with_constraints(&constraints).unwrap();
    let media_stream = JsFuture::from(media_stream_promise).await.unwrap();

    let audio_context = AudioContext::new().unwrap();
    let source = audio_context.create_media_stream_source(&media_stream.unchecked_into()).unwrap();
    let destination = audio_context.destination();

    let worklet = audio_context.audio_worklet().unwrap();
    worklet
        .add_module("path/to/processor.js")
        .await
        .unwrap();

    let processor_node = audio_context
        .create_audio_worklet_node("my-processor")
        .unwrap();

    source.connect_with_audio_node(&processor_node).unwrap();
    processor_node.connect_with_audio_node(&destination).unwrap();

    Ok(JsValue::from_str("Recording started!"))
}

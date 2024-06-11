
use web_sys::AudioContext;
use web_sys::OscillatorNode;
use web_sys::OscillatorType;

fn create_new_oscillator() -> OscillatorNode {
    let audio_context = AudioContext::new().unwrap();
    let oscillator = audio_context.create_oscillator().unwrap();
    oscillator.set_type(OscillatorType::Square);
    oscillator.frequency().set_value(440.0);
    oscillator.connect_with_audio_node(&audio_context.destination()).unwrap();
    oscillator
}

pub fn play_sound() {
    let oscillator = create_new_oscillator();
    wasm_bindgen_futures::spawn_local(async move {
        oscillator.start().unwrap();
        gloo_timers::future::TimeoutFuture::new(100).await;
        oscillator.stop().unwrap();
    });
}

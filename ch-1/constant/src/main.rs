// A static array baked directly into the executable's read-only data section
pub struct Plugin {
    pub name: &'static str,
    pub run: fn(),
}

fn play_sound() { println!("🔊 Playing audio..."); }
fn render_graphics() { println!("🎨 Rendering frame..."); }

// The compiler places these structs into a custom linker section at compile time
pub static PLUGIN_AUDIO: Plugin = Plugin { name: "AudioEngine", run: play_sound };
pub static PLUGIN_RENDER: Plugin = Plugin { name: "RenderEngine", run: render_graphics };

fn main() {
    // Collect static references directly—zero allocation, zero startup lag
    let plugins = [&PLUGIN_AUDIO, &PLUGIN_RENDER];

    println!("--- Initializing Engine Plugins ---");
    for plugin in plugins {
        println!("Loaded: {}", plugin.name);
        (plugin.run)();
    }
}
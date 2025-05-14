use zed_extension_api as zed;

struct Hl7V2 {
    // ... state
}

impl zed::Extension for Hl7V2 {
    fn new() -> Self {
        Self {}
    }
}

zed::register_extension!(Hl7V2);

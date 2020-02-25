pub struct Virtio {
    textbuf: char, //one-char buffer for text output
}

impl Virtio {
    pub fn new() -> Virtio {
        Virtio {
            textbuf: '\0', //NUL char
        }
    }

    pub fn start(&mut self) {
        debug!("Starting virtual IO devices");

    }
}

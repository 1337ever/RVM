use std::sync::mpsc;


pub struct Virtio {
    textbuf: std::sync::mpsc::Receiver<char>, //one-char buffer for text output
    ctrl_channel: std::sync::mpsc::Receiver<u32>,
}

impl Virtio {
    pub fn new(ctrl_channel: std::sync::mpsc::Receiver<u32>, textbuf: std::sync::mpsc::Receiver<char>) -> Virtio {
        Virtio {
            textbuf: textbuf,
            ctrl_channel: ctrl_channel,
        }
    }

    pub fn run(&mut self) {
        debug!("Starting virtual IO devices");

        while true {
            let ctrl = match self.ctrl_channel.try_recv() {
                Ok(v) => v,
                Err(v) => 0,
            };
            if ctrl == 1 {
                break;
            }
            match self.textbuf.try_recv() {
                Ok(v) => print!("{}", v),
                Err(v) => (),
            };
        }
    }
}

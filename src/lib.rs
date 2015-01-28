use std::sync::mpsc::channel;
use std::io::{ChanReader,ChanWriter};
use std::thread::Thread;

struct Pipe {
    incoming: ChanReader
}

impl Pipe {
    fn source(source: &mut Buffer) -> Pipe {
        let (tx, rx) = channel();
        let reader = ChanReader::new(rx);
        let mut writer = ChanWriter::new(tx);

        loop {
            match source.read_char() {
                Ok(c) => writer.write_char(c),
                Err(_) => break
            };
        };

        Pipe { incoming: reader }
    }

    fn sink(&mut self, sink: &mut Writer) {
        loop {
            match self.incoming.read_char() {
                Ok(c) => sink.write_char(c),
                Err(_) => break
            };
        };
    }

    fn pipe(&self, transform: Box<Fn(&mut ChanReader, &mut ChanWriter)+Send>) -> Pipe {
        let (tx, rx) = channel();
        let reader = ChanReader::new(rx);
        let mut writer = ChanWriter::new(tx);

        Thread::spawn(move || {
            transform(&self.incoming, &writer);
        });

        Pipe { incoming: reader }
    }
}

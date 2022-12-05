pub mod io {
    use std::{
        fs::File,
        io::{self, prelude::*},
        path::Path,
        rc::Rc,
    };

    pub struct Reader {
        reader: io::BufReader<File>,
        buf: Rc<String>,
    }

    fn new_buf() -> Rc<String> {
        Rc::new(String::with_capacity(1024)) // Tweakable capacity
    }

    impl Reader {
        pub fn open(file_name: &str) -> io::Result<Self> {
            let path = std::env::current_dir()
                .expect("expected directory")
                .join(Path::new(file_name));

            let file = File::open(path)?;
            let reader = io::BufReader::new(file);
            let buf = new_buf();

            Ok(Self { reader, buf })
        }

        pub fn read_line<'buf>(&mut self, buffer: &'buf mut String) -> &'buf mut String {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|_| buffer)
                .expect("should be line")
        }

        pub fn read<'buf>(&mut self, buffer: &'buf mut String) {
            self.reader
                .read_to_string(buffer)
                .expect("failed to read file");
        }
    }

    impl Iterator for Reader {
        type Item = io::Result<Rc<String>>;

        fn next(&mut self) -> Option<Self::Item> {
            let buf = match Rc::get_mut(&mut self.buf) {
                Some(buf) => {
                    buf.clear();
                    buf
                }
                None => {
                    self.buf = new_buf();
                    Rc::make_mut(&mut self.buf)
                }
            };

            self.reader
                .read_line(buf)
                .map(|u| {
                    if u == 0 {
                        None
                    } else {
                        Some(Rc::clone(&self.buf))
                    }
                })
                .transpose()
        }
    }
}

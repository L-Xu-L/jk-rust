use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::fs::File;
use std::net::TcpStream;


struct MyReader<R> {
    reader: R,
    buf: String,
}

impl<R> MyReader<R>  {
    pub fn new(reader: R) -> Self {
        Self { reader, buf: String::with_capacity(1024) }
    }
}


impl<R> MyReader<R>
where 
    R: Read,
{
    pub fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}

fn main() {
    let f = File::open("/etc/hosts").unwrap();
    let mut reader = MyReader::new(BufReader::new(f));

    let size = reader.process().unwrap();
    println!("total size read: {}", size);


    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut writer = MyWriter::new(BufWriter::new(stream));
    writer.write("hello world").unwrap();
}

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W>  {
    pub fn new(writer: W) -> Self{
        // let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        // Self { writer: BufWriter::new(stream) }
        Self { writer}
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()>{
        self.writer.write_all(buf.as_bytes())
    }
}


// impl MyWriter<BufWriter<TcpStream>> {
//     pub fn new(addr: &str) -> Self{
//         let stream = TcpStream::connect(addr).unwrap();
//         Self { writer: BufWriter::new(stream) }
//     }
// }
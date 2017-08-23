extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::{io, str};
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder, Framed};

use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};

pub struct LineProto;
pub struct LineCodec;

impl Decoder for LineCodec {
  type Item = String;
  type Error = io::Error;

  fn decode (&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
    if let Some(i) = buf.iter().position(|&b| b == b'\n') {
      // remove the serialized frame from the buffer.
      let line = buf.split_to(i);

      // Also remove the '\n'
      buf.split_to(1);

      // turn this data into a UTF string and return it in a Frame
      match str::from_utf8(&line) {
        Ok(s) => Ok(Some(s.to_string())),
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
      }
    }
    else {
      Ok(None)
    }
  }
}

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
  /// For this protocol style, `Request` matches the `Item` type of the
  /// codec's `Decoder`
  type Request = String;

  /// For this protocol style, `Response` matches the `Item` type of the
  /// codec's `Decoder`
  type Response = String;

  // A bit of boilerplate to hook in the codec:
  type Transport = Framed<T, LineCodec>;
  type BindTransport = Result<Self::Transport, io::Error>;
  
  fn bind_transport (&self, io: T) -> Self::BindTransport {
    Ok(io.framed(LineCodec))
  }
}

// Next up: Step 3!

impl Encoder for LineCodec {
  type Item = String;
  type Error = io::Error;
  
  fn encode (&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
    buf.extend(msg.as_bytes());
    buf.extend(b"\n");
    Ok(())
  }
}

fn main() {
  println!("Hello, world!");

}

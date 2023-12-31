use bytes::{Bytes, BytesMut};
use tokio::net::TcpStream;
use mini_redis::{Frame, Result};
use mini_redis::frame::Error::Incomplete;
use std::io::Cursor;
use bytes::Buf;

// Defining Redis Protocol Frame Enum

enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

//Defining HTTP Frame

enum HttpFrame {
    RequestHead {
        method: Method,
        uri: Uri,
        version: Version,
        headers: HeaderMap,
    },
    ResponseHead {
        status: StatusCode,
        version: Version,
        headers: HeaderMap,
    },
    BodyChunk {
        chunk: Bytes,
    },
}

struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,

}

impl Connection {

    // Establishing New connection
    pub fn new(stream: TcpStream) -> Connection {
        Connection{
            stream,
            // Allocate the buffer with 4kb of capacity.
            buffer: vec![0; 4096],
            cursor: 0,
        }
    }
    /// Read a frame from the connection.
    /// 
    /// Returns `None` if EOF is reached
    pub async fn read_frame(&mut self)-> Result<Option<Frame>> {
        // implementation here
        loop {
            // Attempt to parse a frame from the buffered data. If
            // enough data has been buffered, the frame is
            // returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            //Emsure the buffer has capacity 
            if self.buffer.len() == self.cursor {
                // Grow the buffer
                self.buffer.resize(self.cursor*2, 0);
            }

            // Read into the buffer, tracking the number
            // of bytes read
            let n = self.stream.read(
                &mut self.buffer[self.cursor..]).await?;

            if 0 == n {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            } else {
                // Update our cursor
                self.cursor += n;
            }
        
        }
    }

    /// Write a frame to the connection.
    pub async fn write_frame(&mut self, frame: &Frame)
        -> Result<()>
    {
        // implementation here
    }

    fn parse_frame(&mut self)
        -> Result<Option<Frame>>
    {
        // Create the `T: Buf` type.
        let mut buf = Cursor::new(&self.buffer[..]);

        // Check whether a full frame is available
        match Frame::check(&mut buf) {
            Ok(_) => {
                // Get the byte length of the frame
                let len = buf.position() as usize;

                // Reset the internal cursor for the
                // call to `parse`.
                buf.set_position(0);

                // Parse the frame
                let frame = Frame::parse(&mut buf)?;

                // Discard the frame from the buffer
                self.buffer.advance(len);

                // Return the frame to the caller.
                Ok(Some(frame))
            }
            // Not enough data has been buffered
            Err(Incomplete) => Ok(None),
            // An error was encountered
            Err(e) => Err(e.into()),
        }
    }
}


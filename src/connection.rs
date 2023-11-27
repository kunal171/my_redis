use bytes::{Bytes, BytesMut};
use tokio::net::TcpStream;
use mini_redis::{Frame, Result};

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
            buffer: BytesMut::with_capacity(4096),
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
    
            // There is not enough buffered data to read a frame.
            // Attempt to read more data from the socket.
            //
            // On success, the number of bytes is returned. `0`
            // indicates "end of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    /// Write a frame to the connection.
    pub async fn write_frame(&mut self, frame: &Frame)
        -> Result<()>
    {
        // implementation here
    }
}


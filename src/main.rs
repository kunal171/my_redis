use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]

async fn main() {

    //Bind the listner to address
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop{
        let (socket, _) = listner.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self,Get,Set};
    use std::collections::HashMap;

    //A Hashmap is used to store a data
    let mut db = HashMap::new();

    //The `connection` lets us read/write redis **frames** instead
    //byte streans. The `Connection` type is defined by mini-redis
    let mut connection = Connection::new(socket);

   //Use `read_frame` to receive a command from the connection.
   while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
        Set(cmd) => {
            //The value is Stored as `Vec<u8>`
            db.insert(cmd.key().to_string(), cmd.value().to_vec());
            Frame::Simple("OK".to_string())
        }  
        Get(cmd) => {
            if let Some(value) = db.get(cmd.key()) {
                // `Frame::Bulk` expects data to be of type `Bytes`. This
                // type will be covered later in the tutorial. For now,
                // `&Vec<u8>` is converted to `Bytes` using `into()`.
                Frame::Bulk(value.clone().into())
            } else {
                Frame::Null
            }
        }
        cmd => panic!("unimplemnted {:?}", cmd)
    };
    //Write the response to the client
    connection.write_frame(&response).await.unwrap();
   }
}
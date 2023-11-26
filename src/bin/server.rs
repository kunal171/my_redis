use bytes::Bytes;
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use mini_redis::Command::{self,Get,Set};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {

    //Bind the listner to address
    let listner = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listenning");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop{
        let (socket, _) = listner.accept().await.unwrap();

        //Clone the handle to the hashmap.
        let db = db.clone();

        println!("Accepted");
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}



async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    //The `connection` lets us read/write redis **frames** instead
    //byte streans. The `Connection` type is defined by mini-redis
    let mut connection = Connection::new(socket);

   //Use `read_frame` to receive a command from the connection.
   while let Some(frame) = connection.read_frame().await.unwrap() {
    let response = match Command::from_frame(frame).unwrap() {
        Set(cmd) => {
            let mut db = db.lock().unwrap();
            db.insert(cmd.key().to_string(), cmd.value().clone());
            Frame::Simple("OK".to_string())
        }  
        Get(cmd) => {
            let db = db.lock().unwrap();
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
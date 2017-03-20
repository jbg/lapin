extern crate lapin_async as lapin;
#[macro_use] extern crate log;
extern crate env_logger;

use std::net::TcpStream;
use std::collections::HashMap;
use std::{thread,time};

use lapin::connection::*;
use lapin::buffer::Buffer;

fn main() {
      env_logger::init().unwrap();
      let mut stream = TcpStream::connect("127.0.0.1:5672").unwrap();
      stream.set_nonblocking(true);

      let capacity = 8192;
      let mut send_buffer    = Buffer::with_capacity(capacity as usize);
      let mut receive_buffer = Buffer::with_capacity(capacity as usize);

      let mut conn: Connection = Connection::new();
      assert_eq!(conn.connect().unwrap(), ConnectionState::Connecting(ConnectingState::SentProtocolHeader));
      loop {
        match conn.run(&mut stream, &mut send_buffer, &mut receive_buffer) {
          Err(e) => panic!("could not connect: {:?}", e),
          Ok(ConnectionState::Connected) => break,
          Ok(state) => info!("now at state {:?}, continue", state),
        }
        thread::sleep(time::Duration::from_millis(100));
      }
      info!("CONNECTED");

      //now connected

      let channel_a: u16 = conn.create_channel();
      let channel_b: u16 = conn.create_channel();
      //send channel
      conn.channel_open(channel_a, "".to_string()).expect("channel_open");
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());
      thread::sleep(time::Duration::from_millis(100));
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());

      //receive channel
      conn.channel_open(channel_b, "".to_string()).expect("channel_open");
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());
      thread::sleep(time::Duration::from_millis(100));
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());

      //create the hello queue
      conn.queue_declare(channel_a, 0, "hello".to_string(), false, false, false, false, false, HashMap::new()).expect("queue_declare");
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());
      thread::sleep(time::Duration::from_millis(100));
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());

      conn.queue_declare(channel_b, 0, "hello".to_string(), false, false, false, false, false, HashMap::new()).expect("queue_declare");
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());
      thread::sleep(time::Duration::from_millis(100));
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());

      info!("will consume");
      conn.basic_consume(channel_b, 0, "hello".to_string(), "my_consumer".to_string(), false, true, false, false, HashMap::new()).expect("basic_consume");
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());
      thread::sleep(time::Duration::from_millis(100));
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());

      info!("will publish");
      conn.basic_publish(channel_a, 0, "".to_string(), "hello".to_string(), false, false).expect("basic_publish");
      let payload = b"Hello world!";
      conn.send_content_frames(channel_a, 60, payload);
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());
      thread::sleep(time::Duration::from_millis(100));
      info!("[{}] state: {:?}", line!(), conn.run(&mut stream, &mut send_buffer, &mut receive_buffer).unwrap());
      info!("received message: {:?}", conn.next_message(channel_b, "hello", "my_consumer").unwrap());
      panic!();
}


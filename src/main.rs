extern crate websocket;

use std::thread;
use std::sync::mpsc::{channel};
use std::sync::mpsc;

use websocket::{Message, Sender, Receiver};
use websocket::message::Type;
use websocket::client::request::Url;
use websocket::Client;

fn main() {
	let url = Url::parse("ws://127.0.0.1:7777/simulate_lockup").unwrap();
    
    let (test_sender, test_reciever) = mpsc::channel();

	println!("Connecting to {}", url);

	let request = Client::connect(url).unwrap();
	let response = request.send().unwrap();
	response.validate().unwrap(); // Validate the response

	println!("Successfully connected");

	let (mut sender, mut receiver) = response.begin().split();

	let (tx, rx) = channel();

	let tx_1 = tx.clone();
    let thread_test_sender = test_sender.clone();

	let send_loop = thread::spawn(move || {
		loop {
			// Send loop
			let message: Message = match rx.recv() {
				Ok(m) => m,
				Err(e) => {
					println!("Send Loop: {:?}", e);
					return;
				}
			};
			match message.opcode {
				Type::Close => {
					let _ = sender.send_message(&message);

                    thread_test_sender.send(0).unwrap();
                    break;
				},
				_ => (),
			}
			// Send the message
			match sender.send_message(&message) {
				Ok(()) => (),
				Err(e) => {
					println!("Send Loop: {:?}", e);
					let _ = sender.send_message(&Message::close());
					return;
				}
			}
		}
	});

	let receive_loop = thread::spawn(move || {
		// Receive loop
		for message in receiver.incoming_messages() {
			let message: Message = match message {
				Ok(m) => m,
				Err(e) => {
					println!("Receive Loop: {:?}", e);
					let _ = tx_1.send(Message::close());
					return;
				}
			};
			match message.opcode {
				Type::Close => {
					// Got a close message, so send a close message and return
					let _ = tx_1.send(Message::close());
					return;
				}
				Type::Ping => match tx_1.send(Message::pong(message.payload)) {
					// Send a pong in response
					Ok(()) => (),
					Err(e) => {
						println!("Receive Loop: {:?}", e);
						return;
					}
				},
				// Say what we received
				_ => println!("Receive Loop: {:?}", message),
			}
		}
	});


	loop {
        test_reciever.recv().unwrap();
        break;
	}

	// We're exiting

	println!("Waiting for child threads to exit");

	let _ = send_loop.join();
	let _ = receive_loop.join();

	println!("Exited");
}

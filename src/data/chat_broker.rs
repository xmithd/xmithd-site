//! `ChatBroker` is an actor. It maintains list of connection client session.
//! And manages available rooms. Peers send messages to other peers in same
//! room through `ChatBroker`.
//! This is almost copy-pasted from this example:
//! https://github.com/actix/examples/blob/master/websocket-chat/src/server.rs

use actix::prelude::*;
use std::collections::{HashMap, HashSet};
use rand::{self, rngs::ThreadRng, Rng};
use log::{debug};

/// Chat broker sends this message to the websocket actor
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// Message for the broker communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

/// Send message to a specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
    /// Room name
    pub room: String,
}

/// List of available rooms
pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    /// Client id
    pub id: usize,
    /// Room name
    pub name: String,
}

/// `ChatBroker` manages chat rooms and is responsible for coordinating
/// the chat session.
pub struct ChatBroker {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
}

impl Default for ChatBroker {
    fn default() -> Self {
        let mut rooms = HashMap::new();
        rooms.insert("Main".to_owned(), HashSet::new());
        Self {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
        }
    }
}

impl ChatBroker {
    /// Send message to all users in the room
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        let _ = addr.do_send(Message(message.to_owned()));
                    }
                }
            }
        }
    }
}

impl Actor for ChatBroker {
    /// We are going to use simple Context:
    type Context = Context<Self>;
}

/// Handler for Connect message
impl Handler<Connect> for ChatBroker {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();
        let logtxt = format!("user with id {} joined the Main room.", id);
        debug!("{}", &logtxt);
        // notify all users in the same room
        self.send_message(&"Main".to_owned(), &logtxt, 0);

        // register session with random id
        // Chance of collision is really slim
        self.sessions.insert(id, msg.addr);

        // auto join session to Main room
        self.rooms.get_mut(&"Main".to_owned()).unwrap().insert(id);

        // send id back
        id
    }
}

impl Handler<Disconnect> for ChatBroker {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let logtxt = format!("user with id {} disconnected.", &msg.id);
        debug!("{}", &logtxt);
        let mut rooms: Vec<String> = Vec::new();

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove sessions from all rooms
            for (name, sessions) in &mut self.rooms {
                if sessions.remove(&msg.id) {
                    rooms.push(name.to_owned());
                }
            }
        }

        // send message to other users
        for room in rooms {
            self.send_message(&room, &logtxt, 0);
        }
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatBroker {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(&msg.room, &msg.msg, msg.id);
    }
}

/// Handler for `ListRooms` message.
impl Handler<ListRooms> for ChatBroker {
    type Result = MessageResult<ListRooms>;

    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned());
        }

        MessageResult(rooms)
    }
}

/// Join room, send disconnect message to old room
/// send join message to new room
impl Handler<Join> for ChatBroker {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Context<Self>) {
        let Join { id, name } = msg;
        let mut rooms = Vec::new();

        // remove session from all rooms
        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&id) {
                rooms.push(n.to_owned());
            }
        }
        // send message to other users
        for room in rooms {
            self.send_message(&room, &format!("User {} disconnected", id), 0);
        }

        if self.rooms.get_mut(&name).is_none() {
            self.rooms.insert(name.clone(), HashSet::new());
        }
        self.send_message(&name, &format!("User {} connected", id), id);
        self.rooms.get_mut(&name).unwrap().insert(id);
    }
}

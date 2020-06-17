extern crate crossbeam_channel;

use crate::scenes::playing::PlayingState;
use crate::scenes::playing_online::EnemyState;
use crate::util::types::ToResTString;

use crossbeam_channel::{unbounded, Receiver, SendError, Sender};
use laminar::{Config, Packet, Socket, SocketEvent};
use std::net::SocketAddr;
use std::thread;
use std::time::{Duration, Instant};

pub const TIMEOUT: f64 = 2.;
pub const HEARTBEAT_INTERVAL: f64 = 0.2;

const HELLO_STR: &str = "Hello Tetris!";
const MSG_GAME_OVER: &str = "GameOver";
const MSG_HEARTBEAT: &str = "TetrisHeartbeat";
const MSG_HEIGHT: &str = "Height";
const MSG_LINES: &str = "Lines";

#[derive(Clone, Debug)]
pub struct Netinfo {
    // my_addr: SocketConnectionState,
    // my_addr: SocketAddr,
    peer_addr: SocketAddr,
    sender: Sender<Packet>,
    receiver: Receiver<SocketEvent>,
    pub last_response: f64,
    pub last_sent: f64,
    pub enemy: EnemyState,
}

impl Netinfo {
    /// Returns a new instance of Netinfo after validating the parameters
    /// parameters:
    ///     peer_addr_str -> vec!["127.0.0.1", ...]
    pub fn new(peer_address: String) -> Result<Netinfo, String> {
        let my_addresses_str: Vec<&str> = vec!["127.0.0.1:55755", "127.0.0.1:55756"];
        let mut my_addresses: Vec<SocketAddr> = vec![];
        for a in my_addresses_str {
            my_addresses.push(a.parse::<SocketAddr>().to_str_err()?);
        }

        let peer_addresses_str = vec![
            format!("{}:55755", peer_address),
            format!("{}:55756", peer_address),
        ];
        let mut peer_addresses: Vec<SocketAddr> = vec![];
        for a in peer_addresses_str {
            peer_addresses.push(a.parse::<SocketAddr>().to_str_err()?);
        }

        let mut sock_conf = Config::default();
        sock_conf.heartbeat_interval = Some(Duration::from_millis(200));
        println!("binding to either of {:?}", my_addresses);
        let mut me = Socket::bind_with_config(my_addresses.as_slice(), sock_conf).to_str_err()?;
        let me_addr = me.local_addr().unwrap();
        if let Some(index) = peer_addresses.iter().position(|pa| pa == &me_addr) {
            peer_addresses.remove(index);
        }
        println!("success. bound to {:?}", me_addr);

        let (sender, receiver) = (me.get_packet_sender(), me.get_event_receiver());

        let (ts, tr) = unbounded::<bool>();

        let poll_thread = thread::spawn(move || loop {
            if let Ok(should_end) = tr.try_recv() {
                if should_end {
                    return;
                }
            }
            me.manual_poll(Instant::now());
            std::thread::sleep(Duration::from_millis(50));
        });

        let mut repetitions = 0;
        let mut need_to_listen: Option<SocketAddr> = None;
        let mut need_to_send = true;
        while need_to_listen.is_none() || need_to_send {
            if need_to_listen.is_none() {
                // println!("listening");
                if let Ok(se) = receiver.try_recv() {
                    // println!("received");
                    match se {
                        SocketEvent::Packet(p) => {
                            if Self::is_hello(&p) {
                                need_to_listen = Some(p.addr());
                            }
                        }
                        SocketEvent::Connect(_) => {
                            if let Ok(SocketEvent::Packet(p)) = receiver.try_recv() {
                                if Self::is_hello(&p) {
                                    need_to_listen = Some(p.addr());
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            // if need_to_send {
            for address in &peer_addresses {
                // println!("sending");
                if let Ok(()) = Self::hello(address, &sender) {
                    need_to_send = false;
                }
                // Ok(()) => {

                // println!("sent")
                //     }
                //     Err(_) => {
                //         // println!("couldnt send. err: {:?}", err);
                //     }
                // }
            }
            // }
            repetitions += 1;
            if repetitions > 20 || (!need_to_send && need_to_listen.is_some()) {
                break;
            } else {
                std::thread::sleep(Duration::from_millis(500));
            }
        }
        if let Some(addr) = need_to_listen {
            if !need_to_send {
                return Ok(Netinfo {
                    peer_addr: addr,
                    sender,
                    receiver,
                    last_response: 0.,
                    last_sent: 0.,
                    enemy: EnemyState::new(),
                });
            }
        }
        ts.send(true).unwrap();
        poll_thread.join().unwrap();
        Err(String::from("Couldn't establish connection"))
    }

    pub fn delta(&mut self, delta: f64) {
        self.last_response += delta;
        self.last_sent += delta;
    }

    pub fn heartbeat(&mut self) {
        self.send(String::from(MSG_HEARTBEAT));
    }

    /// sends game over packet
    pub fn game_over(&mut self) {
        self.send(String::from(MSG_GAME_OVER));
    }

    pub fn lines(&mut self, amount: usize) {
        self.send(format!("{}{:02}", MSG_LINES, amount))
    }

    pub fn height(&mut self, height: usize) {
        self.send(format!("{}{:02}", MSG_HEIGHT, height));
    }

    pub fn receive(&mut self, playing_state: &mut PlayingState) -> Option<PlayingState> {
        // self.receiver.
        while let Ok(se) = self.receiver.try_recv() {
            if let SocketEvent::Packet(p) = se {
                // match se {
                //  => {
                let msg = String::from_utf8(p.payload().to_vec()).unwrap();
                println!("Got packet saying {:?}", msg);
                let mut valid_packet = true;
                if msg.starts_with(MSG_GAME_OVER) {
                    self.enemy.game_over = true;
                } else if msg.starts_with(MSG_HEARTBEAT) {
                } else if msg.starts_with(MSG_HEIGHT) {
                    let height: usize = msg[MSG_HEIGHT.len()..MSG_HEIGHT.len() + 2]
                        .parse()
                        .expect("Client sending invalid msg");
                    self.enemy.height = height;
                } else if msg.starts_with(MSG_LINES) {
                    let amount: usize = msg[MSG_LINES.len()..MSG_LINES.len() + 2]
                        .parse()
                        .expect("Client sending invalid msg");
                    playing_state.add_garbage_lines(amount);
                } else {
                    println!("Doesnt match!");
                    valid_packet = false;
                }

                if valid_packet {
                    self.last_response = 0.;
                }
                // }
                // _ => {}
            }
        }
        // if let Ok(SocketEvent::Packet(p)) = self.receiver.try_recv() {
        //     println!("p");
        // }
        None
    }

    fn send(&mut self, s: String) {
        self.sender.send(self.pkt(s)).unwrap();
        self.last_sent = 0.;
    }

    fn hello(peer_addr: &SocketAddr, sender: &Sender<Packet>) -> Result<(), SendError<Packet>> {
        sender.send(Packet::reliable_unordered(
            *peer_addr,
            HELLO_STR.to_string().into_bytes(),
        ))
    }

    fn is_hello(packet: &Packet) -> bool {
        String::from_utf8(packet.payload().to_vec()).unwrap() == HELLO_STR
    }

    fn pkt(&self, s: String) -> Packet {
        Packet::reliable_unordered(self.peer_addr, s.into_bytes())
    }
}

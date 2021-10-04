use crate::Result;
use thrussh::*;
use thrussh_keys::*;
use thrussh_config;
use thrussh::server::{Auth, Session};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io::Write;
use std::result::Result as StdResult;

#[derive(Clone)]
pub struct SshServer {
    pub client_pubkey: Arc<thrussh_keys::key::PublicKey>,
    pub clients: Arc<Mutex<HashMap<(usize, ChannelId), thrussh::server::Handle>>>,
    pub id: usize,
}

impl server::Server for SshServer {
    type Handler = Self;
    fn new(&mut self, _: Option<std::net::SocketAddr>) -> Self {
        let s = self.clone();
        self.id += 1;
        s
    }
}

impl server::Handler for SshServer {
    type Error = crate::Error;
    type FutureAuth = futures::future::Ready<Result<(Self, server::Auth)>>;
    type FutureUnit = futures::future::Ready<Result<(Self, Session)>>;
    type FutureBool = futures::future::Ready<Result<(Self, Session, bool)>>;

    fn finished_auth(mut self, auth: Auth) -> Self::FutureAuth {
        futures::future::ready(Ok((self, auth)))
    }
    fn finished_bool(self, b: bool, s: Session) -> Self::FutureBool {
        futures::future::ready(Ok((self, s, b)))
    }
    fn finished(self, s: Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, s)))
    }
    fn channel_open_session(self, channel: ChannelId, session: Session) -> Self::FutureUnit {
        {
            let mut clients = self.clients.lock().unwrap();
            clients.insert((self.id, channel), session.handle());
        }
        self.finished(session)
    }
    fn auth_publickey(self, _: &str, _: &key::PublicKey) -> Self::FutureAuth {
        self.finished_auth(server::Auth::Accept)
    }
    fn data(self, channel: ChannelId, data: &[u8], mut session: Session) -> Self::FutureUnit {
        {
            let mut clients = self.clients.lock().unwrap();
            for ((id, channel), ref mut s) in clients.iter_mut() {
                if *id != self.id {
                    s.data(*channel, CryptoVec::from_slice(data));
                }
            }
        }
        session.data(channel, CryptoVec::from_slice(data));
        self.finished(session)
    }
}

pub struct SshClient;

impl client::Handler for SshClient {
   type Error = crate::Error;
   type FutureUnit = futures::future::Ready<Result<(Self, client::Session)>>;
   type FutureBool = futures::future::Ready<Result<(Self, bool)>>;

   fn finished_bool(self, b: bool) -> Self::FutureBool {
       futures::future::ready(Ok((self, b)))
   }
   fn finished(self, session: client::Session) -> Self::FutureUnit {
       futures::future::ready(Ok((self, session)))
   }
   fn check_server_key(self, server_public_key: &key::PublicKey) -> Self::FutureBool {
       println!("check_server_key: {:?}", server_public_key);
       self.finished_bool(true)
   }
   fn channel_open_confirmation(self, channel: ChannelId, max_packet_size: u32, window_size: u32, session: client::Session) -> Self::FutureUnit {
       println!("channel_open_confirmation: {:?}", channel);
       self.finished(session)
   }
   fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
       println!("data on channel {:?}: {:?}", channel, std::str::from_utf8(data));
       self.finished(session)
   }
}

struct CommandResult {
    output: Vec<u8>,
    code: Option<u32>,
}

impl CommandResult {
    fn output(&self) -> String {
        String::from_utf8_lossy(&self.output).into()
    }

    fn success(&self) -> bool {
        self.code == Some(0)
    }
}

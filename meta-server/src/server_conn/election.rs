use tokio::time::Duration;
use tokio::time::Instant;

use mac_address::{get_mac_address, MacAddress};
use tokio::sync::mpsc;
use tokio::time::timeout_at;
use tracing::info;

use crate::server_conn::election;
use crate::server_conn::protocol::ElectionMsg;

pub enum ElectionResult {
    Winner,
    Loser,
}

pub async fn maintain_heartbeat(state: State) {}

/// future that returns if no heartbeat has been recieved for 
async fn monitor_heartbeat(rx: &mut mpsc::Receiver<ElectionMsg>) {
    const HB_TIMEOUT: Duration = Duration::from_secs(2);
    let mut hb_deadline = Instant::now() + HB_TIMEOUT;
    loop {
        match timeout_at(hb_deadline, rx.recv()).await {
            Ok(Some(ElectionMsg::HeartBeat)) => hb_deadline += HB_TIMEOUT,
            Ok(Some(msg)) => todo!("unhandled electionmsg: {:?}", msg),
            Ok(None) => panic!("should never get an empty msg"),
            Err(_) => break,
        }
    }
}

pub struct State {
    term: u64,
    id: MacAddress,
}

impl State {
    pub fn new() -> Self {
        let mac_addr = get_mac_address()
            .unwrap()
            .expect("there should be at least one network decive");
        Self {
            term: 0,
            id: mac_addr,
        }
    }
}

async fn host_election(rx: &mut mpsc::Receiver<ElectionMsg>, state: &mut State) -> ElectionResult {
    info!("hosting leader election");
    ElectionResult::Loser
}

pub async fn election_cycle(mut rx: mpsc::Receiver<ElectionMsg>, state: &mut election::State) {
    use election::ElectionResult::Winner;

    loop {
        monitor_heartbeat(&mut rx).await;

        // TODO get cmd server listener in here
        if let Winner = host_election(&mut rx, state).await {
            info!("won election");
            break;
        }
    }
}

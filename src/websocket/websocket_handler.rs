pub trait WebSocketHandler {
    fn start() -> tokio::task::JoinHandle<()>;
}

use super::{Method, Msg};
use crate::info;
use anyhow::{Error, Result, anyhow};
use futures_util::{SinkExt, StreamExt};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, OnceLock};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

static CANCEL_TOKEN: OnceLock<CancellationToken> = OnceLock::new();
const DEFAULT_PORT: u16 = 9927;
const LOCAL_IP: &str = "127.0.0.1";
const ANY_IP: &str = "0.0.0.0";

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    pub ip: String,
    pub port: u16,
    #[serde(skip)]
    pub handle: Option<JoinHandle<()>>,
}

impl Engine {
    pub fn new<T: AsRef<str>>(ip: T, port: u16) -> Self {
        let ip = ip.as_ref().to_string();
        Self {
            ip,
            port,
            handle: None,
        }
    }

    fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    async fn serve(self: Arc<Self>, listener: TcpListener, token: CancellationToken) -> Result<()> {
        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    break;
                }
                Ok((stream, _)) = listener.accept() => {
                    // 并行处理多个连接为每个新客户端 spawn 一个独立任务，支持多个客户端同时连接
                    tokio::spawn({
                        let this = Arc::clone(&self);
                        async move {
                            if let Err(e) = this.accept(stream).await {
                                info!("WebSocket accept error: {}", e);
                            }
                        }
                    });
                }
            }
        }

        Ok(())
    }

    async fn accept(self: Arc<Self>, stream: TcpStream) -> Result<()> {
        let ws_stream = match accept_async(stream).await {
            Ok(ws) => ws,
            Err(e) => {
                info!("WebSocket handshake failed: {}", e);
                return Ok(());
            }
        };
        let (write, mut read) = ws_stream.split();
        let write = Arc::new(Mutex::new(write));

        while let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(utf8_bytes) => {
                    let write = Arc::clone(&write);
                    // 为同一客户端的每条消息 spawn 一个任务，实现消息级并行处理
                    tokio::spawn(async move {
                        let value: Value = match serde_json::from_str(&utf8_bytes) {
                            Ok(v) => v,
                            Err(e) => {
                                let mut write = write.lock().await;
                                let _ = write
                                    .send(Msg::error(format!("Invalid JSON: {}", e)).into())
                                    .await;
                                return;
                            }
                        };
                        let method = match value.get("method").and_then(|m| m.as_str()) {
                            Some(m) => {
                                let method: Method = match m.try_into() {
                                    Ok(m) => m,
                                    Err(e) => {
                                        let mut write = write.lock().await;
                                        let _ = write.send(Msg::error(e.to_string()).into()).await;
                                        return;
                                    }
                                };
                                method
                            }
                            None => {
                                let mut write = write.lock().await;
                                let _ = write
                                    .send(Msg::error("Missing 'method' field").into())
                                    .await;
                                return;
                            }
                        };
                        let args = value.get("args").cloned().unwrap_or(Value::Null);

                        match method.handle(args) {
                            Ok(result) => {
                                let mut write = write.lock().await;
                                let _ = write.send(Msg::ok(result).into()).await;
                            }
                            Err(e) => {
                                let mut write = write.lock().await;
                                let _ = write.send(Msg::error(e.to_string()).into()).await;
                            }
                        }
                    });
                }
                Message::Binary(_bytes) => unimplemented!(),
                Message::Ping(_bytes) => unimplemented!(),
                Message::Pong(_bytes) => unimplemented!(),
                Message::Close(_close_frame) => break,
                Message::Frame(_frame) => unimplemented!(),
            }
        }
        Ok(())
    }

    pub async fn is_remote_alive(address: impl Into<String>) -> Result<bool> {
        let address = format!("ws://{}", address.into());
        let (stream, _) = connect_async(address).await?;
        let (mut write, mut read) = stream.split();

        write
            .send(Message::Text(
                r#"{"method":"health_check"}"#.to_string().into(),
            ))
            .await?;

        if let Some(Ok(Message::Text(resp))) = read.next().await {
            let ok = serde_json::from_str::<Value>(&resp)
                .ok()
                .and_then(|v| v.get("data")?.get("health")?.as_str().map(|s| s == "ok"))
                .unwrap_or(false);
            return Ok(ok);
        }

        Ok(false)
    }

    // 127.0.0.1
    pub fn local_address() -> String {
        format!("{}:{}", LOCAL_IP, DEFAULT_PORT)
    }

    // 192.168.x.x
    pub fn lan_address() -> String {
        format!("{}:{}", local_ip().unwrap(), DEFAULT_PORT)
    }

    pub fn to_local(&mut self) -> &mut Self {
        self.ip = LOCAL_IP.to_string();
        self.port = DEFAULT_PORT;
        self
    }

    /**
     * macos 下测试：
     * 1. wscat -c ws://192.168.5.55:9927，如果提示：error: socket hang up，则表示可能防火墙没开这个局域网端口进入白名单。
     * 2. sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate，如果提示：Firewall is enabled. (State = 1)，则表示防火墙已开启，可临时关闭防火墙用于测试。
     * 3. sudo /usr/libexec/ApplicationFirewall/socketfilterfw --setglobalstate off，临时关闭防火墙进行测试。
     * macos 生产环境下不用管，系统在创建局域网websocket时会自动弹出“是否允许‘YourApp.app’接收入站网络连接？”，选择是就可以使用局域网连接了。
     */
    pub fn to_any(&mut self) -> &mut Self {
        self.ip = ANY_IP.to_string();
        self.port = DEFAULT_PORT;
        self
    }

    pub async fn serve_in_background(
        &mut self,
        failed_handler: Box<dyn Fn(Error) + 'static + Send>,
    ) -> Result<()> {
        let listener = loop {
            match TcpListener::bind(&self.address()).await {
                Ok(l) => break l,
                Err(error) => {
                    if self.port >= 65535 {
                        return Err(anyhow!(t!(
                            "Unable to start WebSocket Service.",
                            port = self.port,
                            error = error.to_string()
                        )));
                    }
                    self.port += 1;
                    continue;
                }
            }
        };
        let token = CancellationToken::new();
        let _ = CANCEL_TOKEN.set(token.clone());
        let this = Arc::new(std::mem::replace(
            self,
            Self::new(self.ip.clone(), self.port),
        ));

        let handle = tokio::spawn({
            let this = Arc::clone(&this);
            async move {
                if let Err(error) = this.serve(listener, token).await {
                    failed_handler(error);
                }
            }
        });

        self.handle = Some(handle);

        Ok(())
    }

    pub async fn shutdown(&mut self) {
        if let Some(token) = CANCEL_TOKEN.get() {
            token.cancel();
        }
        if let Some(handle) = &self.handle {
            handle.abort();
        }
        self.handle = None;
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new(LOCAL_IP, DEFAULT_PORT)
    }
}

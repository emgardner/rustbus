use crate::gui::components::connection::ConnectionParams;
use iced::{subscription, Subscription};
use log::{debug};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_modbus::client::{rtu, tcp, Client, Context};
use tokio_modbus::prelude::{Request, Response};
use tokio_serial::SerialPortBuilderExt;

pub enum WorkerState {
    Disconnected,
    Ready(UnboundedReceiver<Commands>),
    Connected(UnboundedReceiver<Commands>, Context),
    Error,
}

#[derive(Debug, Clone)]
pub enum Commands {
    Nothing,
    Disconnect,
    Connect(ConnectionParams),
    DeviceCommand(Request),
    RequestCommand(Request),
}

#[derive(Debug, Clone)]
pub enum WorkerEvent {
    WorkerHandle(UnboundedSender<Commands>),
    DeviceResponse(Request, Response),
    RequestResponse(Request, Response),
    Connected,
    Disconnected,
    Idle,
    Error(String),
}

async fn create_context(conn_params: ConnectionParams) -> Result<Context, std::io::Error> {
    match conn_params {
        ConnectionParams::Serial(sp) => {
            debug!("Opening Port: {:?}", sp.port);
            let port = tokio_serial::new(sp.port, sp.baudrate)
                .data_bits(sp.data_bits)
                .flow_control(tokio_serial::FlowControl::None)
                .stop_bits(sp.stop_bits)
                .parity(sp.parity)
                .timeout(sp.timeout)
                .open_native_async()?;
            rtu::connect(port).await
        }
        ConnectionParams::Tcp(tcp_params) => {
            debug!("Opening Port: {:?}", tcp_params);
            let addr = Ipv4Addr::from(tcp_params.ip);
            let socket_addr = SocketAddrV4::new(addr, tcp_params.port);
            tcp::connect(std::net::SocketAddr::V4(socket_addr)).await
        }
    }
}

pub fn connect() -> Subscription<WorkerEvent> {
    struct Worker;
    subscription::unfold(
        std::any::TypeId::of::<Worker>(),
        WorkerState::Disconnected,
        |state| async move {
            match state {
                WorkerState::Disconnected => {
                    let (mtx, srx) = unbounded_channel::<Commands>();
                    (
                        Some(WorkerEvent::WorkerHandle(mtx)),
                        WorkerState::Ready(srx),
                    )
                }
                WorkerState::Ready(mut srx) => {
                    if let Some(command) = srx.recv().await {
                        match command {
                            Commands::Connect(p) => {
                                let ctx = create_context(p).await;
                                match ctx {
                                    Ok(p) => (
                                        Some(WorkerEvent::Connected),
                                        WorkerState::Connected(srx, p),
                                    ),
                                    Err(e) => (
                                        Some(WorkerEvent::Error(e.to_string())),
                                        WorkerState::Ready(srx),
                                    ),
                                }
                            }
                            _ => (Some(WorkerEvent::Idle), WorkerState::Ready(srx)),
                        }
                    } else {
                        (Some(WorkerEvent::Idle), WorkerState::Ready(srx))
                    }
                }
                WorkerState::Connected(mut srx, mut ctx) => {
                    if let Some(command) = srx.recv().await {
                        match command {
                            Commands::Nothing => (None, WorkerState::Connected(srx, ctx)),
                            Commands::Disconnect => {
                                (Some(WorkerEvent::Disconnected), WorkerState::Ready(srx))
                            }
                            Commands::DeviceCommand(cmd) => {
                                let res = ctx.call(cmd.clone()).await;
                                match res {
                                    Ok(resp) => (
                                        Some(WorkerEvent::DeviceResponse(cmd, resp)),
                                        WorkerState::Connected(srx, ctx),
                                    ),
                                    Err(e) => (
                                        Some(WorkerEvent::Error(e.to_string())),
                                        WorkerState::Connected(srx, ctx),
                                    ),
                                }
                            }
                            Commands::RequestCommand(cmd) => {
                                let res = ctx.call(cmd.clone()).await;
                                match res {
                                    Ok(resp) => (
                                        Some(WorkerEvent::RequestResponse(cmd, resp)),
                                        WorkerState::Connected(srx, ctx),
                                    ),
                                    Err(e) => (
                                        Some(WorkerEvent::Error(e.to_string())),
                                        WorkerState::Connected(srx, ctx),
                                    ),
                                }
                            }
                            _ => (
                                Some(WorkerEvent::Error("Invalid Command".to_string())),
                                WorkerState::Error,
                            ),
                        }
                    } else {
                        (Some(WorkerEvent::Idle), WorkerState::Connected(srx, ctx))
                    }
                }
                WorkerState::Error => (
                    Some(WorkerEvent::Error("error".to_string())),
                    WorkerState::Error,
                ),
            }
        },
    )
}

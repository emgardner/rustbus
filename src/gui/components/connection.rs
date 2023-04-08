use crate::gui::app::App;
use crate::gui::protocol::Protocol;
use iced::{
    widget::{button, column, pick_list, row},
    Element,
};

const CONNECT_OPTIONS: [&str; 2] = ["SERIAL", "TCP"];

pub mod serial;
pub mod tcp;

use self::serial::{SerialPortComponent, SerialPortParams};
use self::tcp::{TcpComponent, TcpParams};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Copy, Clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionParams {
    Serial(SerialPortParams),
    Tcp(TcpParams),
}

impl ConnectionParams {
    pub fn new() -> Self {
        ConnectionParams::Serial(SerialPortParams::default())
    }

    pub fn get_string_option(&self) -> &'static str {
        match self {
            ConnectionParams::Serial(_p) => CONNECT_OPTIONS[0],
            ConnectionParams::Tcp(_p) => CONNECT_OPTIONS[1],
        }
    }
}

pub fn connection<'a>(app: &App) -> Element<'a, Protocol> {
    let c: Element<_> = match &app.connection {
        ConnectionParams::Serial(params) => {
            SerialPortComponent::<Protocol>::new(params.clone(), |params| {
                Protocol::ConnectionChanged(ConnectionParams::Serial(params))
            })
            .into()
        }
        ConnectionParams::Tcp(params) => TcpComponent::<Protocol>::new(params.clone(), |params| {
            Protocol::ConnectionChanged(ConnectionParams::Tcp(params))
        })
        .into(),
    };
    let b: Element<_> = match &app.connected {
        true => button("Disconnect")
            .on_press(Protocol::Disconnect)
            .style(iced_style::theme::Button::Destructive)
            .into(),
        false => button("Connect")
            .on_press(Protocol::Connect(app.connection.clone()))
            .style(iced_style::theme::Button::Primary)
            .into(),
    };
    let current_type = app.connection.get_string_option();
    row![
        column![
            "Connection Type",
            pick_list(&CONNECT_OPTIONS[..], Some(&current_type), |val| {
                match val {
                    "SERIAL" => Protocol::ConnectionChanged(ConnectionParams::Serial(
                        SerialPortParams::new(),
                    )),
                    "TCP" => Protocol::ConnectionChanged(ConnectionParams::Tcp(TcpParams::new(
                        [127, 0, 0, 1],
                        502,
                        255,
                    ))),
                    _ => Protocol::ConnectionChanged(ConnectionParams::Serial(
                        SerialPortParams::new(),
                    )),
                }
            })
            .width(100),
        ]
        .spacing(10),
        c,
        b
    ]
    .spacing(10)
    .padding(10)
    .align_items(iced::Alignment::End)
    .into()
}

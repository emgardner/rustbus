use std::net::Ipv4Addr;

use iced::alignment::Alignment;
use iced::widget::{self, text_input};
use iced::widget::{column, row, text};
use iced::Element;
use iced_lazy::Component;
use iced_native;
use serde::de::{Deserializer, Error};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct TcpParams {
    #[serde(deserialize_with = "deserialize_ip")]
    pub ip: [u8; 4],
    pub port: u16,
    pub address: u8,
}

impl Serialize for TcpParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("serial_port", 2)?;
        let ip = Ipv4Addr::new(self.ip[0], self.ip[1], self.ip[2], self.ip[3]);
        state.serialize_field("ip", &ip.to_string())?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("address", &self.address)?;
        state.end()
    }
}

fn deserialize_ip<'de, D>(deserializer: D) -> Result<[u8; 4], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.parse::<Ipv4Addr>() {
        Ok(ip) => Ok(ip.octets()),
        Err(_e) => Err(D::Error::custom("Invalid IP")),
    }
}

impl TcpParams {
    pub fn new(ip: [u8; 4], port: u16, address: u8) -> Self {
        Self { ip, port, address }
    }
}

pub struct TcpComponent<Message> {
    params: TcpParams,
    on_change: Box<dyn Fn(TcpParams) -> Message>,
}

impl<Message> TcpComponent<Message> {
    pub fn new(params: TcpParams, on_change: impl Fn(TcpParams) -> Message + 'static) -> Self {
        Self {
            params: params,
            on_change: Box::new(on_change),
        }
    }
}

impl<Message, Renderer> Component<Message, Renderer> for TcpComponent<Message>
where
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme:
        widget::text::StyleSheet + widget::text_input::StyleSheet + widget::container::StyleSheet,
{
    type State = ();
    type Event = TcpParamsMessage;

    fn update(&mut self, _state: &mut Self::State, event: TcpParamsMessage) -> Option<Message> {
        match event {
            TcpParamsMessage::Ip1Changed(v) => self.params.ip[0] = v,
            TcpParamsMessage::Ip2Changed(v) => self.params.ip[1] = v,
            TcpParamsMessage::Ip3Changed(v) => self.params.ip[2] = v,
            TcpParamsMessage::Ip4Changed(v) => self.params.ip[3] = v,
            TcpParamsMessage::PortChanged(v) => self.params.port = v,
            TcpParamsMessage::AddressChanged(v) => self.params.address = v,
            _ => (),
        };
        Some(self.on_change.as_ref()(self.params))
    }

    fn view(&self, _state: &Self::State) -> Element<'static, Self::Event, Renderer> {
        row![
            column![
                text("Address: "),
                row![
                    text_input("127", &self.params.ip[0].to_string(), |new_val| {
                        let num = new_val.parse::<u8>();
                        match num {
                            Ok(n) => TcpParamsMessage::Ip1Changed(n),
                            _ => TcpParamsMessage::None,
                        }
                    })
                    .width(50.0),
                    text_input("127", &self.params.ip[1].to_string(), |new_val| {
                        let num = new_val.parse::<u8>();
                        match num {
                            Ok(n) => TcpParamsMessage::Ip2Changed(n),
                            _ => TcpParamsMessage::None,
                        }
                    })
                    .width(50.0),
                    text_input("127", &self.params.ip[2].to_string(), |new_val| {
                        let num = new_val.parse::<u8>();
                        match num {
                            Ok(n) => TcpParamsMessage::Ip3Changed(n),
                            _ => TcpParamsMessage::None,
                        }
                    })
                    .width(50.0),
                    text_input("127", &self.params.ip[3].to_string(), |new_val| {
                        let num = new_val.parse::<u8>();
                        match num {
                            Ok(n) => TcpParamsMessage::Ip4Changed(n),
                            _ => TcpParamsMessage::None,
                        }
                    })
                    .width(50.0),
                ]
                .align_items(Alignment::Center)
                .spacing(10.0)
            ]
            .align_items(Alignment::Center)
            .spacing(10),
            column![
                text("Port: "),
                text_input("502", &self.params.port.to_string(), |new_val| {
                    let num = new_val.parse::<u16>();
                    match num {
                        Ok(n) => TcpParamsMessage::PortChanged(n),
                        _ => TcpParamsMessage::None,
                    }
                })
                .width(50.0)
            ]
            .align_items(Alignment::Center)
            .spacing(10.0),
            column![
                text("Address"),
                text_input("255", &self.params.address.to_string(), |new_val| {
                    let num = new_val.parse::<u8>();
                    match num {
                        Ok(n) => TcpParamsMessage::AddressChanged(n),
                        _ => TcpParamsMessage::None,
                    }
                })
                .width(50.0)
            ]
            .align_items(Alignment::Center)
            .spacing(10.0)
        ]
        .align_items(iced::Alignment::Center)
        .spacing(10.0)
        .into()
    }
}

impl<'a, Message, Renderer> From<TcpComponent<Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme:
        widget::text::StyleSheet + widget::text_input::StyleSheet + widget::container::StyleSheet,
{
    fn from(tcp: TcpComponent<Message>) -> Self {
        iced_lazy::component(tcp)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpParamsMessage {
    Ip1Changed(u8),
    Ip2Changed(u8),
    Ip3Changed(u8),
    Ip4Changed(u8),
    AddressChanged(u8),
    PortChanged(u16),
    None,
}

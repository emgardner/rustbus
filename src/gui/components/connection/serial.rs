use iced::alignment::Alignment;
use iced::widget::{self, column, text_input};
use iced::widget::{pick_list, row};
use iced::Element;
use iced_lazy::Component;
use iced_native;
use iced_style;
use serde::de::{Deserializer, Error};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio_serial::{self, DataBits, Parity, StopBits};

const BAUDRATES: [u32; 14] = [
    110, 300, 600, 1200, 2400, 4800, 9600, 14400, 19200, 38400, 57600, 115200, 128000, 256000,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuiParity(Parity);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuiDataBits(DataBits);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuiStopBits(StopBits);

impl GuiParity {
    const ALL: [GuiParity; 3] = [
        GuiParity(Parity::None),
        GuiParity(Parity::Odd),
        GuiParity(Parity::Even),
    ];
}

impl GuiDataBits {
    const ALL: [GuiDataBits; 4] = [
        GuiDataBits(DataBits::Five),
        GuiDataBits(DataBits::Six),
        GuiDataBits(DataBits::Seven),
        GuiDataBits(DataBits::Eight),
    ];
}

impl GuiStopBits {
    const ALL: [GuiStopBits; 2] = [GuiStopBits(StopBits::One), GuiStopBits(StopBits::Two)];
}

impl From<GuiParity> for Parity {
    fn from(item: GuiParity) -> Self {
        item.0
    }
}

impl From<GuiDataBits> for DataBits {
    fn from(item: GuiDataBits) -> Self {
        item.0
    }
}

impl From<GuiStopBits> for StopBits {
    fn from(item: GuiStopBits) -> Self {
        item.0
    }
}

impl std::fmt::Display for GuiParity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GuiParity(Parity::None) => "None",
                GuiParity(Parity::Odd) => "Odd",
                GuiParity(Parity::Even) => "Even",
            }
        )
    }
}

impl std::fmt::Display for GuiDataBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GuiDataBits(DataBits::Five) => "5",
                GuiDataBits(DataBits::Six) => "6",
                GuiDataBits(DataBits::Seven) => "7",
                GuiDataBits(DataBits::Eight) => "8",
            }
        )
    }
}

impl std::fmt::Display for GuiStopBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GuiStopBits(StopBits::One) => "1",
                GuiStopBits(StopBits::Two) => "2",
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SerialPortParams {
    pub port: String,
    pub baudrate: u32,
    #[serde(deserialize_with = "deserialize_parity")]
    pub parity: Parity,
    #[serde(deserialize_with = "deserialize_data_bits")]
    pub data_bits: DataBits,
    #[serde(deserialize_with = "deserialize_stop_bits")]
    pub stop_bits: StopBits,
    pub timeout: std::time::Duration,
    pub address: u8,
}

fn deserialize_parity<'de, D>(deserializer: D) -> Result<Parity, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Odd" => Ok(Parity::Odd),
        "Even" => Ok(Parity::Even),
        "None" => Ok(Parity::None),
        _ => Err(D::Error::custom("Invalid Field")),
    }
}

fn deserialize_data_bits<'de, D>(deserializer: D) -> Result<DataBits, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "5" => Ok(DataBits::Five),
        "6" => Ok(DataBits::Six),
        "7" => Ok(DataBits::Seven),
        "8" => Ok(DataBits::Eight),
        _ => Err(D::Error::custom("Invalid Field")),
    }
}

fn deserialize_stop_bits<'de, D>(deserializer: D) -> Result<StopBits, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "1" => Ok(StopBits::One),
        "2" => Ok(StopBits::Two),
        _ => Err(D::Error::custom("Invalid Field")),
    }
}

impl Serialize for SerialPortParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let parity = GuiParity(self.parity).to_string();
        let data_bits = GuiDataBits(self.data_bits).to_string();
        let stop_bits = GuiStopBits(self.stop_bits).to_string();
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("serial_port", 7)?;
        state.serialize_field("port", &self.port)?;
        state.serialize_field("baudrate", &self.baudrate)?;
        state.serialize_field("parity", &parity)?;
        state.serialize_field("data_bits", &data_bits)?;
        state.serialize_field("stop_bits", &stop_bits)?;
        state.serialize_field("timeout", &self.timeout)?;
        state.serialize_field("address", &self.address)?;
        state.end()
    }
}

impl Default for SerialPortParams {
    fn default() -> Self {
        Self {
            port: "".to_string(),
            baudrate: 115200,
            parity: Parity::None,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            timeout: std::time::Duration::from_millis(1000),
            address: 1,
        }
    }
}

pub struct SerialPortComponent<Message> {
    params: SerialPortParams,
    on_change: Box<dyn Fn(SerialPortParams) -> Message>,
}

impl<Message> SerialPortComponent<Message> {
    pub fn new(
        params: SerialPortParams,
        on_change: impl Fn(SerialPortParams) -> Message + 'static,
    ) -> Self {
        Self {
            params: params,
            on_change: Box::new(on_change),
        }
    }
}
impl<Message, Renderer> Component<Message, Renderer> for SerialPortComponent<Message>
where
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme: widget::text::StyleSheet
        + widget::text_input::StyleSheet
        + widget::pick_list::StyleSheet
        + widget::scrollable::StyleSheet
        + widget::container::StyleSheet
        + iced_native::overlay::menu::StyleSheet,
    <Renderer::Theme as iced::overlay::menu::StyleSheet>::Style:
        From<<Renderer::Theme as iced_style::pick_list::StyleSheet>::Style>,
{
    type State = ();
    type Event = SerialPortParamsMessage;

    fn update(
        &mut self,
        _state: &mut Self::State,
        event: SerialPortParamsMessage,
    ) -> Option<Message> {
        match event {
            SerialPortParamsMessage::BaudrateChanged(br) => self.params.baudrate = br,
            SerialPortParamsMessage::ParityChanged(p) => self.params.parity = p,
            SerialPortParamsMessage::DataBitsChanged(db) => self.params.data_bits = db,
            SerialPortParamsMessage::StopBitsChanged(sb) => self.params.stop_bits = sb,
            SerialPortParamsMessage::TimeoutChanged(d) => self.params.timeout = d,
            SerialPortParamsMessage::PortChanged(d) => self.params.port = d,
            SerialPortParamsMessage::AddressChanged(d) => self.params.address = d,
            _ => (),
        };
        Some(self.on_change.as_ref()(self.params.clone()))
    }

    fn view(&self, _state: &Self::State) -> Element<'static, Self::Event, Renderer> {
        let spacing = 10.0;
        row![
            column![
                "Com Port",
                text_input("", &self.params.port, |x| {
                    SerialPortParamsMessage::PortChanged(x)
                })
                .width(100)
            ]
            .align_items(Alignment::Center)
            .spacing(spacing),
            column![
                "Baudrate",
                pick_list(&BAUDRATES[..], Some(self.params.baudrate), |x| {
                    SerialPortParamsMessage::BaudrateChanged(x)
                })
                .width(80.0)
            ]
            .align_items(Alignment::Center)
            .spacing(spacing),
            column![
                "Parity",
                pick_list(
                    &GuiParity::ALL[..],
                    Some(GuiParity(self.params.parity)),
                    |x| SerialPortParamsMessage::ParityChanged(Parity::from(x)),
                )
                .width(60.0)
            ]
            .align_items(Alignment::Center)
            .spacing(spacing),
            column![
                "Data Bits",
                pick_list(
                    &GuiDataBits::ALL[..],
                    Some(GuiDataBits(self.params.data_bits)),
                    |x| SerialPortParamsMessage::DataBitsChanged(DataBits::from(x)),
                )
                .width(60.0)
            ]
            .align_items(Alignment::Center)
            .spacing(spacing),
            column![
                "Stop Bits",
                pick_list(
                    &GuiStopBits::ALL[..],
                    Some(GuiStopBits(self.params.stop_bits)),
                    |x| SerialPortParamsMessage::StopBitsChanged(StopBits::from(x)),
                )
                .width(60.0)
            ]
            .align_items(Alignment::Center)
            .spacing(spacing),
            column![
                "Timeout (ms)",
                text_input("(ms)", &self.params.timeout.as_millis().to_string(), |x| {
                    let num = x.parse::<u64>();
                    match num {
                        Ok(n) => SerialPortParamsMessage::TimeoutChanged(
                            std::time::Duration::from_millis(n),
                        ),
                        _ => SerialPortParamsMessage::None,
                    }
                })
                .width(70.0)
            ]
            .align_items(Alignment::Center)
            .spacing(spacing),
            column![
                "Address",
                text_input("1", &self.params.address.to_string(), |x| {
                    let num = x.parse::<u8>();
                    match num {
                        Ok(n) => SerialPortParamsMessage::AddressChanged(n),
                        _ => SerialPortParamsMessage::None,
                    }
                })
                .width(60.0)
            ]
            .align_items(Alignment::Center)
            .spacing(spacing)
        ]
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }
}

impl<'a, Message, Renderer> From<SerialPortComponent<Message>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: iced_native::text::Renderer + 'static,
    Renderer::Theme: widget::text::StyleSheet
        + widget::text_input::StyleSheet
        + widget::container::StyleSheet
        + widget::pick_list::StyleSheet
        + widget::scrollable::StyleSheet
        + iced_native::overlay::menu::StyleSheet,
    <Renderer::Theme as iced::overlay::menu::StyleSheet>::Style:
        From<<Renderer::Theme as iced_style::pick_list::StyleSheet>::Style>,
{
    fn from(sp: SerialPortComponent<Message>) -> Self {
        iced_lazy::component(sp)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerialPortParamsMessage {
    BaudrateChanged(u32),
    ParityChanged(Parity),
    DataBitsChanged(DataBits),
    StopBitsChanged(StopBits),
    TimeoutChanged(std::time::Duration),
    PortChanged(String),
    AddressChanged(u8),
    None,
}

impl SerialPortParams {
    pub fn new() -> Self {
        Self {
            port: "".to_string(),
            baudrate: 115200,
            parity: Parity::None,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            timeout: Duration::from_secs(1),
            address: 1,
        }
    }
}

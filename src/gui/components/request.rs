use crate::gui::components::common::ContainerStyle;
use crate::gui::protocol::Protocol;
use crate::gui::style::ButtonType;
use iced::widget::{self, button, pick_list, text_input, Container, Row, image, image::Handle};
use iced::widget::{column, row, text};
use iced::Renderer;
use iced::{Alignment, Color, Element};
use iced_aw::native::wrap::Wrap;
use iced_lazy::Component;
use iced_native;
use tokio_modbus::prelude::{Request, Response};

const PANEL_WIDTH: f32 = 500.0;

pub struct RequestComponent<Message> {
    on_change: Box<dyn Fn(RequestParams) -> Message>,
    params: RequestParams,
}

#[derive(Debug, Clone)]
pub struct RequestParams {
    pub request: Request,
    pub request_type: RequestType,
    pub response: Option<Response>,
    pub poll: std::time::Duration,
    pub polling: bool
}

#[derive(Debug, Clone)]
pub enum RequestUpdate {
    None,
    RequestType(RequestType),
    SetAddress(u16),
    Request(Request),
    UpdateVecU16(usize, u16),
    UpdateVecBool(usize, bool),
    SetPoll(std::time::Duration)
}

fn response_or_request(request_paramters: &RequestParams) -> Element<Protocol> {
    match &request_paramters.request {
        Request::ReadCoils(_addr, val) => {
            if let Some(Response::ReadCoils(v)) = &request_paramters.response {
                Wrap::with_elements(
                    v.iter()
                        .map(|x| response_data_box(text(x.to_string())).into())
                        .collect(),
                )
                .max_width(PANEL_WIDTH)
                .into()
            } else {
                Wrap::with_elements(
                    (0..*val)
                        .map(|_x| response_data_box(text("")).into())
                        .collect(),
                )
                .max_width(PANEL_WIDTH)
                .into()
            }
        }
        Request::ReadDiscreteInputs(_addr, val) => {
            if let Some(Response::ReadDiscreteInputs(v)) = &request_paramters.response {
                Wrap::with_elements(
                    v.iter()
                        .map(|x| response_data_box(text(x.to_string())).into())
                        .collect(),
                )
                .into()
            } else {
                Wrap::with_elements(
                    (0..*val)
                        .map(|_x| response_data_box(text("")).into())
                        .collect(),
                )
                .into()
            }
        }
        Request::WriteSingleCoil(_addr, _val) => text("").into(),
        Request::WriteMultipleCoils(_addr, vals) => Wrap::with_elements(
            vals.iter()
                .enumerate()
                .map(|(i, x)| {
                    text_input("", &x.to_string(), move |z| {
                        let parsed = z.parse::<bool>();
                        match parsed {
                            Ok(new_val) => {
                                Protocol::RequestUpdate(RequestUpdate::UpdateVecBool(i, new_val))
                            }
                            Err(_e) => Protocol::None,
                        }
                    })
                    .width(50.0)
                    .into()
                })
                .collect(),
        )
        .into(),
        Request::ReadInputRegisters(_addr, val) => {
            if let Some(Response::ReadInputRegisters(v)) = &request_paramters.response {
                Wrap::with_elements(
                    v.iter()
                        .map(|x| response_data_box(text(x.to_string())).into())
                        .collect(),
                )
                .into()
            } else {
                Wrap::with_elements(
                    (0..*val)
                        .map(|_x| response_data_box(text("")).into())
                        .collect(),
                )
                .into()
            }
        }
        Request::ReadHoldingRegisters(_addr, val) => {
            if let Some(Response::ReadHoldingRegisters(v)) = &request_paramters.response {
                Wrap::with_elements(
                    v.iter()
                        .map(|x| response_data_box(text(x.to_string())).into())
                        .collect(),
                )
                .into()
            } else {
                Wrap::with_elements(
                    (0..*val)
                        .map(|_x| response_data_box(text("")).into())
                        .collect(),
                )
                .into()
            }
        }
        Request::WriteSingleRegister(_addr, _val) => text("").into(),
        Request::WriteMultipleRegisters(_addr, vals) => Wrap::with_elements(
            vals.iter()
                .enumerate()
                .map(|(i, x)| {
                    text_input("", &x.to_string(), move |z| {
                        let parsed = z.parse::<u16>();
                        match parsed {
                            Ok(new_val) => {
                                Protocol::RequestUpdate(RequestUpdate::UpdateVecU16(i, new_val))
                            }
                            Err(_e) => Protocol::None,
                        }
                    })
                    .width(50.0)
                    .into()
                })
                .collect(),
        )
        .into(),
        _ => row![text("Unsupported Code")].into(),
    }
}

pub fn get_address(req: &Request) -> u16 {
    match req {
        Request::ReadCoils(addr, _val) => *addr,
        Request::ReadDiscreteInputs(addr, _val) => *addr,
        Request::WriteSingleCoil(addr, _val) => *addr,
        Request::WriteMultipleCoils(addr, _val) => *addr,
        Request::ReadInputRegisters(addr, _val) => *addr,
        Request::ReadHoldingRegisters(addr, _val) => *addr,
        Request::WriteSingleRegister(addr, _val) => *addr,
        Request::WriteMultipleRegisters(addr, _val) => *addr,
        _ => unreachable!(),
    }
}

fn get_value(req: &Request) -> Element<Protocol> {
    match req {
        Request::ReadCoils(addr, val) => column![
            "Coils",
            text_input("", &val.to_string(), |x| {
                let parsed = x.parse::<u16>();
                match parsed {
                    Ok(new_val) => Protocol::ReqChanged(Request::ReadCoils(*addr, new_val)),
                    Err(_e) => Protocol::None,
                }
            })
        ]
        .align_items(Alignment::Center)
        .width(100.0)
        .into(),
        Request::ReadDiscreteInputs(addr, val) => column![
            "Inputs",
            text_input("", &val.to_string(), |x| {
                let parsed = x.parse::<u16>();
                match parsed {
                    Ok(new_val) => {
                        Protocol::ReqChanged(Request::ReadDiscreteInputs(*addr, new_val))
                    }
                    Err(_e) => Protocol::None,
                }
            })
        ]
        .align_items(Alignment::Center)
        .width(100.0)
        .into(),
        Request::WriteSingleCoil(addr, val) => column![
            "Value",
            text_input("", &val.to_string(), |x| {
                let parsed = x.parse::<bool>();
                match parsed {
                    Ok(new_val) => Protocol::ReqChanged(Request::WriteSingleCoil(*addr, new_val)),
                    Err(_e) => Protocol::None,
                }
            })
        ]
        .align_items(Alignment::Center)
        .width(100.0)
        .into(),
        Request::WriteMultipleCoils(addr, val) => column![
            "Coils",
            text_input("", &val.len().to_string(), |x| {
                let parsed = x.parse::<u16>();
                match parsed {
                    Ok(new_val) => {
                        let mut vec_copy = val.clone();
                        vec_copy.resize(new_val.into(), false);
                        Protocol::ReqChanged(Request::WriteMultipleCoils(*addr, vec_copy))
                    }
                    Err(_e) => Protocol::None,
                }
            })
        ]
        .align_items(Alignment::Center)
        .width(100.0)
        .into(),
        Request::ReadInputRegisters(addr, val) => column![
            "Registers",
            text_input("", &val.to_string(), |x| {
                let parsed = x.parse::<u16>();
                match parsed {
                    Ok(new_val) => {
                        Protocol::ReqChanged(Request::ReadInputRegisters(*addr, new_val))
                    }
                    Err(_e) => Protocol::None,
                }
            })
        ]
        .align_items(Alignment::Center)
        .width(100.0)
        .into(),
        Request::ReadHoldingRegisters(addr, val) => column![
            "Registers",
            text_input("", &val.to_string(), |x| {
                let parsed = x.parse::<u16>();
                match parsed {
                    Ok(new_val) => {
                        Protocol::ReqChanged(Request::ReadHoldingRegisters(*addr, new_val))
                    }
                    Err(_e) => Protocol::None,
                }
            })
        ]
        .align_items(Alignment::Center)
        .width(100.0)
        .into(),
        Request::WriteSingleRegister(addr, val) => column![
            "Value",
            text_input("", &val.to_string(), |x| {
                let parsed = x.parse::<u16>();
                match parsed {
                    Ok(new_val) => {
                        Protocol::ReqChanged(Request::WriteSingleRegister(*addr, new_val))
                    }
                    Err(_e) => Protocol::None,
                }
            })
        ]
        .align_items(Alignment::Center)
        .width(100.0)
        .into(),
        Request::WriteMultipleRegisters(addr, val) => column![
            "Registers",
            text_input("", &val.len().to_string(), |x| {
                let parsed = x.parse::<u16>();
                match parsed {
                    Ok(new_val) => {
                        let mut vec_copy = val.clone();
                        vec_copy.resize(new_val.into(), 0);
                        Protocol::ReqChanged(Request::WriteMultipleRegisters(*addr, vec_copy))
                    }
                    Err(_e) => Protocol::None,
                }
            })
        ]
        .align_items(Alignment::Center)
        .width(100.0)
        .into(),
        _ => unreachable!(),
    }
}







impl RequestParams {
    pub fn update(&mut self, msg: RequestUpdate) {
        match msg {
            RequestUpdate::None => (),
            RequestUpdate::RequestType(req_type) => {
                let req = req_type.new_request();
                self.request_type = req_type;
                self.request = req;
            }
            RequestUpdate::SetAddress(new_addr) => match &mut self.request {
                Request::ReadCoils(addr, _val) => *addr = new_addr,
                Request::ReadDiscreteInputs(addr, _val) => *addr = new_addr,
                Request::WriteSingleCoil(addr, _val) => *addr = new_addr,
                Request::WriteMultipleCoils(addr, _val) => *addr = new_addr,
                Request::ReadInputRegisters(addr, _val) => *addr = new_addr,
                Request::ReadHoldingRegisters(addr, _val) => *addr = new_addr,
                Request::WriteSingleRegister(addr, _val) => *addr = new_addr,
                Request::WriteMultipleRegisters(addr, _val) => *addr = new_addr,
                _ => unreachable!(),
            },
            RequestUpdate::Request(req) => self.request = req,
            RequestUpdate::UpdateVecU16(idx, val) => {
                match &mut self.request {
                    Request::WriteMultipleRegisters(_addr, vals) => {
                        vals.get_mut(idx).map(|x| *x = val);
                    }
                    _ => (),
                };
            },
            RequestUpdate::UpdateVecBool(idx, val) => match &mut self.request {
                Request::WriteMultipleCoils(_addr, vals) => {
                    vals.get_mut(idx).map(|x| *x = val);
                }
                _ => (),
            },
            RequestUpdate::SetPoll(duration) => {
                self.poll = duration
            },
        }
    }

    pub fn view(&self) -> Element<Protocol> {
        let poll_btn = if !self.polling {
           button(image(Handle::from_path("./resources/sync.png")).width(25.0))
            .on_press(Protocol::StartPoll)
            .style(ButtonType::Image.into())
        } else {
           button(image(Handle::from_path("./resources/stop_poll.png")).width(25.0))
            .on_press(Protocol::StopPoll)
            .style(ButtonType::Image.into())
        };
        Container::new(
            column![
                row![
                    column![
                        "Request",
                        pick_list(&RequestType::ALL[..], Some(self.request_type), |val| {
                            Protocol::RequestUpdate(RequestUpdate::RequestType(val))
                        })
                    ]
                    .align_items(Alignment::Center),
                    column![
                        "Address",
                        text_input("Address", &self.get_address().to_string(), |x| {
                            let parsed = x.parse::<u16>();
                            match parsed {
                                Ok(new_addr) => {
                                    Protocol::RequestUpdate(RequestUpdate::SetAddress(new_addr))
                                }
                                Err(_e) => Protocol::None,
                            }
                        }),
                    ]
                    .align_items(Alignment::Center)
                    .width(100.0),
                    get_value(&self.request),
                    button("Execute").on_press(Protocol::ExecuteRequest),
                    column![
                        "Poll (ms)",
                        text_input("1000", &self.poll.as_millis().to_string(), |x| {
                            let num = x.parse::<u64>();
                            match num {
                                Ok(n) => Protocol::RequestUpdate(RequestUpdate::SetPoll(std::time::Duration::from_millis(n))),
                                _ => Protocol::RequestUpdate(RequestUpdate::None),
                            }
                        }),
                    ]
                    .align_items(Alignment::Center)
                    .width(100.0),
                    poll_btn
                ]
                .spacing(10.0)
                .align_items(Alignment::End),
                response_or_request(&self)
            ]
            .spacing(10.0),
        )
        .padding(10.0)
        .into()
    }

    
    pub fn get_address(&self) -> u16 {
        match &self.request {
            Request::ReadCoils(addr, _val) => *addr,
            Request::ReadDiscreteInputs(addr, _val) => *addr,
            Request::WriteSingleCoil(addr, _val) => *addr,
            Request::WriteMultipleCoils(addr, _val) => *addr,
            Request::ReadInputRegisters(addr, _val) => *addr,
            Request::ReadHoldingRegisters(addr, _val) => *addr,
            Request::WriteSingleRegister(addr, _val) => *addr,
            Request::WriteMultipleRegisters(addr, _val) => *addr,
            _ => unreachable!(),
        }
    }

    pub fn get_data(&self) -> Element<Protocol> {
        match &self.request {
            Request::ReadCoils(_addr, val) => request_history_single_data(val.to_string()),
            Request::ReadDiscreteInputs(_addr, val) => request_history_single_data(val.to_string()),
            Request::WriteSingleCoil(_addr, val) => request_history_single_data(val.to_string()),
            Request::WriteMultipleCoils(_addr, val) => Row::with_children( val.iter().map(|x| text(x.to_string()).into() ).collect() ).into(),
            Request::ReadInputRegisters(_addr, val) => request_history_single_data(val.to_string()),
            Request::ReadHoldingRegisters(_addr, val) => request_history_single_data(val.to_string()),
            Request::WriteSingleRegister(_addr, val) => request_history_single_data(val.to_string()),
            Request::WriteMultipleRegisters(_addr, val) => Row::with_children( val.iter().map(|x| text(x.to_string()).into() ).collect() ).into(),
            _ => unreachable!(),
        }
    }
}

fn request_history_single_data(data: String) -> Element<'static, Protocol> {
    text(data)
        .width(200.0)
        .into()
        
}

impl<Message> RequestComponent<Message> {
    pub fn new(
        request_params: RequestParams,
        on_change: impl Fn(RequestParams) -> Message + 'static,
    ) -> RequestComponent<Message> {
        Self {
            on_change: Box::new(on_change),
            params: request_params,
        }
    }
}
impl std::default::Default for RequestParams {
    fn default() -> RequestParams {
        Self {
            request: Request::ReadCoils(0, 0),
            request_type: RequestType::ReadCoils,
            response: None,
            poll: std::time::Duration::from_millis(1000),
            polling: false
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RequestType {
    ReadCoils,
    ReadDiscreteInputs,
    WriteSingleCoil,
    WriteMultipleCoils,
    ReadInputRegisters,
    ReadHoldingRegisters,
    WriteSingleRegister,
    WriteMultipleRegisters,
    // MaskWriteRegister,
    // ReadWriteMultipleRegisters
}

impl RequestType {
    fn new_request(&self) -> Request {
        match self {
            RequestType::ReadCoils => Request::ReadCoils(0, 0),
            RequestType::ReadDiscreteInputs => Request::ReadDiscreteInputs(0, 0),
            RequestType::WriteSingleCoil => Request::WriteSingleCoil(0, false),
            RequestType::WriteMultipleCoils => Request::WriteMultipleCoils(0, Vec::new()),
            RequestType::ReadInputRegisters => Request::ReadInputRegisters(0, 0),
            RequestType::ReadHoldingRegisters => Request::ReadHoldingRegisters(0, 0),
            RequestType::WriteSingleRegister => Request::WriteSingleRegister(0, 0),
            RequestType::WriteMultipleRegisters => Request::WriteMultipleRegisters(0, Vec::new()),
        }
    }
}

impl std::fmt::Display for RequestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RequestType::ReadCoils => "Read Coils (0x01)",
                RequestType::ReadDiscreteInputs => "Read Discrete Inputs (0x02)",
                RequestType::WriteSingleCoil => "Write Single Coil (0x05)",
                RequestType::WriteMultipleCoils => "Write Multiple Coils (0x0F)",
                RequestType::ReadInputRegisters => "Read Input Registers (0x04)",
                RequestType::ReadHoldingRegisters => "Read Holding Registers (0x03)",
                RequestType::WriteSingleRegister => "Write Single Register (0x06)",
                RequestType::WriteMultipleRegisters => "Write Multiple Registers (0x10)",
            }
        )
    }
}

impl RequestType {
    const ALL: [RequestType; 8] = [
        RequestType::ReadCoils,
        RequestType::ReadDiscreteInputs,
        RequestType::WriteSingleCoil,
        RequestType::WriteMultipleCoils,
        RequestType::ReadInputRegisters,
        RequestType::ReadHoldingRegisters,
        RequestType::WriteSingleRegister,
        RequestType::WriteMultipleRegisters,
    ];
}

fn response_data_box<'a, T, Message>(t: T) -> Container<'a, Message>
where
    T: Into<Element<'a, Message, Renderer>>,
{
    Container::new(t)
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle {
            text_color: None,
            background: None,
            border_radius: 1.0,
            border_width: 1.0,
            border_color: Color::WHITE,
        })))
        .width(50.0)
        .height(30.0)
        .center_x()
        .center_y()
}


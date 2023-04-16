use crate::gui::components::common::ContainerStyle;
use crate::gui::protocol::Protocol;
use crate::gui::style::ButtonType;
use iced::{Padding, Renderer};

use iced::{
    widget::{
        button, image, image::Handle, pick_list, row, scrollable, text, text_input, Column,
        Container,
    },
    Alignment, Background, Color, Element, Length,
};
use tokio_modbus::prelude::{Request, Response};

use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub enum TableCommand {
    AddRegister(Option<usize>),
    SetAddress(usize, u16),
    SetType(usize, RegisterType),
    SetName(usize, String),
    SetDescription(usize, String),
    SetValue(usize, u16),
    Delete(usize),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    registers: Vec<Register>,
}

#[derive(Debug, Clone)]
pub enum RegisterRequest {
    Read(Register),
    Write(Register),
}

impl From<RegisterRequest> for Request {
    fn from(register: RegisterRequest) -> Request {
        match register {
            RegisterRequest::Read(reg) => match reg.register_type {
                RegisterType::Coil => Request::ReadCoils(reg.address, 1),
                RegisterType::DiscreteInputs => Request::ReadDiscreteInputs(reg.address, 1),
                RegisterType::InputRegister => Request::ReadInputRegisters(reg.address, 1),
                RegisterType::HoldingRegister => Request::ReadHoldingRegisters(reg.address, 1),
            },
            RegisterRequest::Write(reg) => match reg.register_type {
                RegisterType::Coil => Request::WriteSingleCoil(reg.address, reg.value != 0),
                RegisterType::InputRegister => Request::WriteSingleRegister(reg.address, reg.value),
                RegisterType::HoldingRegister => {
                    Request::WriteMultipleRegisters(reg.address, vec![reg.value])
                }
                RegisterType::DiscreteInputs => unreachable!(),
            },
        }
        // Request::MaskWriteRegister(u16, u16, u16),
        // Request::ReadWriteMultipleRegisters(u16, u16, u16, Vec<u16>),
        // Request::Custom(u8, Vec<u8>),
    }
}

pub fn get_address_from_request(req: Request) -> Option<u16> {
    match req {
        Request::ReadCoils(addr, _) => Some(addr),
        Request::ReadDiscreteInputs(addr, _) => Some(addr),
        Request::WriteSingleCoil(addr, _) => Some(addr),
        Request::WriteMultipleCoils(addr, _) => Some(addr),
        Request::ReadInputRegisters(addr, _) => Some(addr),
        Request::ReadHoldingRegisters(addr, _) => Some(addr),
        Request::WriteSingleRegister(addr, _) => Some(addr),
        Request::WriteMultipleRegisters(addr, _) => Some(addr),
        _ => None
        // Request::MaskWriteRegister(addr, _, u16),
        // Request::ReadWriteMultipleRegisters(addr, u16, addr, Vec<u16>),
        // Request::Custom(u8, Vec<u8>),
        // Request::Disconnect,
    }
}

impl Table {
    pub fn handle_commmand(&mut self, tc: TableCommand) {
        match tc {
            TableCommand::AddRegister(idx) => {
                if let Some(index) = idx {
                    self.registers.insert(index+1, Register::default());
                } else {
                    self.registers.push(Register::default());
                }
            }
            TableCommand::SetAddress(idx, addr) => {
                self.registers.get_mut(idx).map(|x| {
                    x.address = addr;
                });
            }
            TableCommand::SetType(idx, rtype) => {
                self.registers.get_mut(idx).map(|x| {
                    x.register_type = rtype;
                });
            }
            TableCommand::SetName(idx, name) => {
                self.registers.get_mut(idx).map(|x| {
                    x.name = name;
                });
            }
            TableCommand::SetDescription(idx, desc) => {
                self.registers.get_mut(idx).map(|x| {
                    x.description = desc;
                });
            }
            TableCommand::SetValue(idx, val) => {
                println!("{idx} {val}");
                self.registers.get_mut(idx).map(|x| {
                    x.value = val;
                });
            }
            TableCommand::Delete(idx) => {
                self.registers.remove(idx);
            }
            TableCommand::None => (),
        }
    }

    pub fn handle_response(&mut self, req: Request, resp: Response) {
        if let Some(addr) = get_address_from_request(req) {
            self.registers.iter_mut().for_each(|x| {
                if x.address == addr {
                    x.apply_response(&resp)
                }
            })
        } else {
            println!("NO Address");
        };
    }

    pub fn load_from_file(path: &std::path::Path) -> Result<Table, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let u = serde_json::from_reader(reader)?;
        Ok(u)
    }
}

impl std::default::Default for Table {
    fn default() -> Self {
        Self {
            registers: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegisterType {
    Coil,
    DiscreteInputs,
    InputRegister,
    HoldingRegister,
}

impl std::fmt::Display for RegisterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegisterType::Coil => "COIL",
                RegisterType::DiscreteInputs => "DISCRETE",
                RegisterType::InputRegister => "INPUT",
                RegisterType::HoldingRegister => "HOLDING",
            }
        )
    }
}

impl RegisterType {
    const ALL: [RegisterType; 4] = [
        RegisterType::Coil,
        RegisterType::DiscreteInputs,
        RegisterType::InputRegister,
        RegisterType::HoldingRegister,
    ];
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    register_type: RegisterType,
    address: u16,
    length: u8,
    name: String,
    value: u16,
    description: String,
}

impl Register {
    fn apply_response(&mut self, resp: &Response) {
        match resp {
            Response::ReadCoils(vbool) => self.value = u16::from(vbool[0]),
            Response::ReadDiscreteInputs(vbool) => self.value = u16::from(vbool[0]),
            Response::WriteSingleCoil(_addr, val) => self.value = u16::from(*val),
            // Fix
            Response::ReadInputRegisters(vinputs) => self.value = vinputs[0],
            Response::ReadHoldingRegisters(vholding) => self.value = vholding[0],
            Response::WriteSingleRegister(_addr, val) => self.value = *val,
            Response::WriteMultipleRegisters(_addr, val) => self.value = *val,
            Response::WriteMultipleCoils(_addr, val) => self.value = *val,
            // Response::MaskWriteRegister(u16, u16, u16),
            // Response::ReadWriteMultipleRegisters(Vec<u16>),
            // Response::Custom(u8, Vec<u8>),
            _ => (),
        }
    }
}

impl std::default::Default for Register {
    fn default() -> Self {
        Self {
            register_type: RegisterType::Coil,
            address: 0,
            length: 1,
            name: "".to_string(),
            value: 0,
            description: "".to_string(),
        }
    }
}

pub fn row_from_register<'a>(idx: usize, register: &Register) -> Container<'a, Protocol, Renderer> {
    Container::new(
        row![
            text_input("0", &register.address.to_string(), move |x| {
                let parsed = x.parse::<u16>();
                match parsed {
                    Ok(addr) => Protocol::TableCommand(TableCommand::SetAddress(idx, addr)),
                    Err(_e) => Protocol::TableCommand(TableCommand::None),
                }
            })
            .width(100.0),
            pick_list(
                &RegisterType::ALL[..],
                Some(register.register_type),
                move |x| { Protocol::TableCommand(TableCommand::SetType(idx, x)) }
            ),
            text_input("name", &register.name, move |x| {
                Protocol::TableCommand(TableCommand::SetName(idx, x))
            })
            .width(100.0),
            text_input("description", &register.description, move |x| {
                Protocol::TableCommand(TableCommand::SetDescription(idx, x))
            })
            .width(100.0),
            text_input("value", &register.value.to_string(), move |x| {
                println!("input: {:?}", x);
                let parsed = x.parse::<u16>();
                println!("parsed: {:?}", parsed);
                match parsed {
                    Ok(addr) => Protocol::TableCommand(TableCommand::SetValue(idx, addr)),
                    Err(_e) => Protocol::TableCommand(TableCommand::None),
                }
            })
            .width(100.0),
            actions(idx, register)
        ]
        .align_items(Alignment::Center)
        .spacing(5.0),
    )
    .padding(5.0)
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
}

pub fn actions<'a>(idx: usize, reg: &Register) -> Container<'a, Protocol> {
    let image_size = 24.0;
    Container::new(
        row![
            button(image(Handle::from_path("./resources/read.png")).width(image_size))
                .on_press(Protocol::ModbusRequest(Request::from(
                    RegisterRequest::Read(reg.clone())
                )))
                .style(ButtonType::Image.into()),
            button(image(Handle::from_path("./resources/write.png")).width(image_size))
                .on_press(Protocol::ModbusRequest(Request::from(
                    RegisterRequest::Write(reg.clone())
                )))
                .style(ButtonType::Image.into()),
            button(image(Handle::from_path("./resources/garbage.png")).width(image_size))
                .on_press(Protocol::TableCommand(TableCommand::Delete(idx)))
                .style(ButtonType::Image.into()),
            button(image(Handle::from_path("./resources/plus.png")).width(image_size))
                .on_press(Protocol::TableCommand(TableCommand::AddRegister(Some(idx))))
                .style(ButtonType::Image.into()),
        ]
        .align_items(Alignment::Center)
        .spacing(10.0),
    )
}

pub fn header_cell<'a, T, Message>(t: T) -> Container<'a, Message, Renderer>
where
    T: Into<Element<'a, Message, Renderer>>,
{
    Container::new(t)
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(30.0))
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle {
            text_color: Some(Color::WHITE),
            background: Some(Background::from(Color::BLACK)),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Color::WHITE,
        })))
}

pub fn table_cell<'a, T, Message>(t: T) -> Container<'a, Message, Renderer>
where
    T: Into<Element<'a, Message, Renderer>>,
{
    Container::new(t)
        .width(Length::Fixed(100.0))
        .height(Length::Fixed(30.0))
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(ContainerStyle {
            text_color: None,
            background: None,
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Color::WHITE,
        })))
}

pub fn table<'a>(table: &Table) -> Column<'a, Protocol, Renderer> {
    let addr_width = 100.0;
    let type_width = 100.0;
    let name_width = 100.0;
    let value_width = 100.0;
    let description_width = 100.0;
    let actions_width = 100.0;
    let headers = row![
        text("Address").width(addr_width),
        text("Type").width(type_width),
        text("Name").width(name_width),
        text("Description").width(value_width),
        text("Value").width(description_width),
        text("Actions").width(actions_width)
    ]
    .spacing(5)
    .padding(Padding::from([10, 5]));
    let mut c = Column::new()
        .spacing(0)
        .align_items(iced::Alignment::Center);
    c = c.push(text("Register Mapping"));
    c = c.push(headers);
    let rc = Column::with_children(
        table
            .registers
            .iter()
            .enumerate()
            .map(|(idx, reg)| row_from_register(idx, reg).into())
            .collect(),
    )
    .align_items(iced::Alignment::Center);
    c = c.push(
        scrollable(rc).height(Length::Fill).vertical_scroll(
            iced::widget::scrollable::Properties::new()
                .width(1.0)
                .margin(1.0)
                .scroller_width(1.0),
        ),
    );
    c.push(
        Container::new(
            button("Add Row").on_press(Protocol::TableCommand(TableCommand::AddRegister(None))),
        )
        .padding(10.0),
    )
    .padding(10.0)
}

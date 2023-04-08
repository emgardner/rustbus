use iced::theme::{Palette, Theme};
use iced::time::Duration;
use iced::widget::{button, column, text, Container};
use iced::{executor, Alignment};
use iced::{Application, Element};
use iced::{Color, Command, Length, Settings, Subscription};
use iced_aw::native::Modal;
use rfd::AsyncFileDialog;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use tokio::sync::mpsc::UnboundedSender;

use serde::{Deserialize, Serialize};

use crate::controller::{connect, Commands, WorkerEvent};
use crate::gui::components::connection::ConnectionParams;
use crate::gui::components::request::RequestParams;
use crate::gui::components::table::Table;
use crate::gui::pages::home_page::home_page;
use crate::gui::protocol::Protocol;

pub fn run_app() -> iced::Result {
    let mut settings = Settings::default();
    settings.default_text_size = 15.0;
    App::run(settings)
}

#[derive(Debug, Clone)]
pub enum AppState {
    HomePage,
    ControlPage,
}

impl std::default::Default for AppState {
    fn default() -> Self {
        AppState::HomePage
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct App {
    #[serde(skip_deserializing, skip_serializing)]
    pub state: AppState,
    pub connection: ConnectionParams,
    #[serde(skip_deserializing, skip_serializing)]
    pub connected: bool,
    #[serde(skip_deserializing, skip_serializing)]
    pub tx_handle: Option<UnboundedSender<Commands>>,
    pub table: Table,
    #[serde(skip_deserializing, skip_serializing)]
    pub config_file: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub request_params: RequestParams,
    #[serde(skip_deserializing, skip_serializing)]
    is_error: bool,
    #[serde(skip_deserializing, skip_serializing)]
    error_text: String,
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
         .field("state", &self.state)
         .field("connection", &self.connection)
         .finish()
    }
}

impl App {
    fn send_message(&mut self, cmd: Commands) {
        if let Some(tx_handle) = &self.tx_handle {
            let _ = tx_handle.send(cmd);
        }
    }

    pub fn load_from_file(
        &mut self,
        path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let u: Result<App, serde_json::Error> = serde_json::from_reader(reader);
        match u {
            Ok(app) => {
                self.table = app.table;
                self.connection = app.connection;
            }
            Err(e) => {
                self.is_error = true;
                self.error_text = e.to_string();
            }
        };
        Ok(())
    }

    pub fn load_settings(
        path: &std::path::Path,
    ) -> Result<App, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn load(&mut self) {
        let _ = self.load_from_file(std::path::Path::new("./settings.json"));
    }

    pub fn save_to_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(filename) = &self.config_file {
            let file = File::create(filename)?;
            let writer = BufWriter::new(file);
            Ok(serde_json::to_writer_pretty(writer, self)?)
        } else {
            let file = File::create("./settings.json")?;
            let writer = BufWriter::new(file);
            Ok(serde_json::to_writer_pretty(writer, self)?)
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Protocol;
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<Protocol>) {
        let mut app = App {
            state: AppState::HomePage,
            connection: ConnectionParams::new(),
            connected: false,
            tx_handle: None,
            table: Table::default(),
            config_file: None,
            request_params: RequestParams::default(),
            is_error: false,
            error_text: String::new(),
        };
        app.load();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Rust Bus")
    }

    fn update(&mut self, message: Protocol) -> Command<Protocol> {
        match message {
            Protocol::ConnectionChanged(params) => {
                self.connection = params;
            }
            Protocol::Connect(params) => {
                self.send_message(Commands::Connect(params));
            }
            Protocol::TableCommand(tc) => {
                self.table.handle_commmand(tc);
            }
            Protocol::WorkerEvent(ev) => {
                // println!("{:?}", ev);
                match ev {
                    WorkerEvent::WorkerHandle(tx_handle) => {
                        self.tx_handle = Some(tx_handle);
                    },
                    WorkerEvent::DeviceResponse(req, res) => {
                        println!("{:?} {:?}", req, res);
                        self.table.handle_response(req, res);
                    },
                    WorkerEvent::RequestResponse(req, res) => {
                        println!("{:?} {:?}", req, res);
                        self.request_params.response = Some(res);
                    },
                    WorkerEvent::Error(e) => {
                        self.is_error = true;
                        self.error_text = e;
                    },
                    WorkerEvent::Connected => {
                        self.connected = true;
                    },
                    _ => ()

                    // WorkerEvent::Disconnected,
                    // WorkerEvent::Idle,
                }
            },
            Protocol::Disconnect => {
                self.send_message(Commands::Disconnect);
                self.connected = false;
            }
            Protocol::ModbusRequest(req) => {
                self.send_message(Commands::DeviceCommand(req));
            }
            Protocol::ExecuteRequest => {
                println!("Execute Request Pressed");
                self.send_message(Commands::RequestCommand(self.request_params.request.clone()));
            }
            Protocol::RequestChanged(params) => {
                self.request_params = params;
            }
            Protocol::ReqChanged(params) => {
                self.request_params.request = params;
            }
            Protocol::RequestUpdate(msg) => {
                self.request_params.update(msg);
            }
            Protocol::SaveFile => {
                self.save_to_file();
            }
            Protocol::CloseModal => {
                self.is_error = false;
            }
            Protocol::OpenFileDialog => {
                let future = async {
                    AsyncFileDialog::new()
                        .add_filter("json", &["json"])
                        .set_directory("/")
                        .pick_file()
                        .await
                };
                return Command::perform(future, |file| {
                    match file {
                        Some(f) => {
                            match App::load_settings(f.path()) {
                                Ok(mut app) => {
                                    app.config_file = Some(f.file_name());
                                    Protocol::ApplyApp(app)
                                },
                                Err(e) => Protocol::Error(e.to_string())
                            }
                        },
                        None => Protocol::None
                    }
                });
            }
            Protocol::SaveFileDialog => {
                let future = async {
                    AsyncFileDialog::new()
                        .add_filter("json", &["json"])
                        .set_directory("/")
                        .save_file()
                        .await
                };
                return Command::perform(future, |file| {
                    match file {
                        Some(fname) => Protocol::SaveFileWithPath(fname.file_name()),
                        None => Protocol::None
                    }
                });
            }
            Protocol::SaveFileWithPath(filename) => {
                self.config_file = Some(filename);
                self.save_to_file();
            }
            Protocol::Error(e) => {
                self.is_error = true;
                self.error_text = e.to_string();
            }
            Protocol::ApplyApp(app) => {
                self.connection = app.connection;
                self.table = app.table;
                self.config_file = app.config_file;
            }
            _ => (),
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Protocol> {
        iced::Subscription::batch([
            connect().map(Protocol::WorkerEvent),
            iced::time::every(Duration::from_secs(5)).map(|_x| Protocol::SaveFile),
        ])
    }

    fn view(&self) -> Element<Protocol> {
        let c = match self.state {
            AppState::HomePage => home_page(&self),
            AppState::ControlPage => home_page(&self),
        };
        let content = Container::new(c)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();
        Modal::new(self.is_error, content, || {
            Container::new(
                column![
                    text(&self.error_text),
                    button("CLOSE").on_press(Protocol::CloseModal)
                ]
                .align_items(Alignment::Center)
                .spacing(20.0),
            )
            .width(200.0)
            .height(200.0)
            .center_x()
            .center_y()
            .into()
        })
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::custom(Palette {
            background: Color::from_rgb8(34, 34, 34),
            text: Color::from_rgb8(242, 242, 242),
            primary: Color::from_rgb8(198, 151, 75),
            success: Color::from_rgb8(250, 204, 21),
            danger: Color::from_rgb8(248, 113, 113),
        })
    }
}

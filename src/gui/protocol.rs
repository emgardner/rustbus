use crate::controller::WorkerEvent;
use crate::gui::components::connection::ConnectionParams;
use crate::gui::app::App;
use crate::gui::components::request::{RequestParams, RequestUpdate};
use crate::gui::components::table::TableCommand;
use tokio_modbus::prelude::Request;

#[derive(Debug, Clone)]
pub enum Protocol {
    Debug,
    ConnectionChanged(ConnectionParams),
    Connect(ConnectionParams),
    Disconnect,
    WorkerEvent(WorkerEvent),
    TableCommand(TableCommand),
    ModbusRequest(Request),
    ExecuteRequest,
    RequestChanged(RequestParams),
    ReqChanged(Request),
    RequestUpdate(RequestUpdate),
    SaveFile,
    LoadFile,
    OpenFileDialog,
    SaveFileDialog,
    CloseModal,
    ApplyApp(App),
    SaveFileWithPath(String), 
    None,
    Error(String)
}

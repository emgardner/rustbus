use crate::gui::app::App;
use crate::gui::components::common::{header, screen};
use crate::gui::components::connection::connection;
use crate::gui::components::table::table;
use crate::gui::components::request::RequestParams;
use crate::gui::protocol::Protocol;

use iced::widget::{column, row, scrollable, text, Column, Container, tooltip};
use iced::{Alignment, Element, Color};
use iced::{Length, Padding, Renderer};
use crate::gui::style::ContainerStyle;

fn raw_data_viewer<'a>() -> Element<'a, Protocol> {
    let column_widths = 200;
    let tx_data =
        Column::with_children((0..100).map(|idx| text(idx).into()).collect()).width(column_widths);
    let rx_data =
        Column::with_children((0..100).map(|idx| text(idx).into()).collect()).width(column_widths);
    Container::new(column![
        row![
            text("Tx Data").width(column_widths),
            text("Rx Data").width(column_widths),
        ],
        scrollable(row![tx_data, rx_data])
            .height(Length::Fill)
            .vertical_scroll(
                iced::widget::scrollable::Properties::new()
                    .width(1.0)
                    .margin(1.0)
                    .scroller_width(1.0),
            )
    ])
    .padding(Padding::from([0, 20]))
    .into()
    // scrollable(rc).height(Length::Fixed(400.0)).vertical_scroll(
    //     iced::widget::scrollable::Properties::new()
    //         .width(1.0)
    //         .margin(1.0)
    //         .scroller_width(1.0),
    // ),
    // .style(iced::theme::Container::Custom(Box::new(ContainerStyle {
    //     text_color: None,
    //     background: None,
    //     border_radius: 0.0,
    //     border_width: 2.0,
    //     border_color: Color::WHITE,
    // })))
}

pub fn request_history_row(request: &RequestParams) -> Container<Protocol>
{
    let mut req_type = text(request.request_type.to_string()).width(150.0);
    let mut addr = text(request.get_address()).width(75.0);
    let mut data = request.get_data();
    Container::new(row![
                   req_type,
                   addr,
                   data
        ])
        .padding(5.0)
        .center_x()
        .center_y()
        // .style(iced::theme::Container::Custom(Box::new(ContainerStyle {
        //     text_color: None,
        //     background: None,
        //     border_radius: 0.0,
        //     border_width: 1.0,
        //     border_color: Color::WHITE,
        // })))
}

pub fn request_history(app: &App) -> Element<Protocol> {
    let requests = Column::with_children(
        app.request_history.iter().map(|x| {
            tooltip(
                request_history_row(x),
                "Click to Load",
                iced_native::widget::tooltip::Position::FollowCursor
            ).into()

        }).collect()
    )
    .align_items(Alignment::Center);
    column![
        "Request History",
        row![
            text("Request Type").width(150.0),
            text("Address").width(75.0),
            text("Data").width(200.0)
        ],
        scrollable(requests).height(Length::Fill).vertical_scroll(
            iced::widget::scrollable::Properties::new()
                .width(1.0)
                .margin(1.0)
                .scroller_width(1.0),
        )
    ]
    .spacing(10.0)
    .height(200)
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .into()
}

pub fn home_page(app: &App) -> Element<Protocol> {
    let mut c = Column::new()
        .width(Length::Fill)
        .align_items(Alignment::Center);
    c = c.push(connection(app));
    if app.connected {
        c = c.push(app.request_params.view());
        c = c.push(request_history(&app));
    };
    c = c.push(table(&app.table));
    screen(
        column![
            header(),
            row![
                c
            ]
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center),
    )
    .into()
}

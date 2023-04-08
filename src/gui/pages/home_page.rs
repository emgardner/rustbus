use crate::gui::app::App;
use crate::gui::components::common::{header, screen};
use crate::gui::components::connection::connection;
use crate::gui::components::table::table;

use crate::gui::protocol::Protocol;

use iced::widget::{column, row, scrollable, text, Column, Container};
use iced::{Alignment, Element};
use iced::{Length, Padding};

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

pub fn home_page(app: &App) -> Element<Protocol> {
    let mut c = Column::new();
    c = c.push(connection(app));
    if app.connected {
        c = c.push(app.request_params.view());
    };
    c = c.push(table(&app.table));
    screen(
        column![
            header(),
            row![
                c
                // column![
                //     connection(app),
                //     app.request_params.view(),
                //     table(&app.table)
                // ],
                // raw_data_viewer()
            ]
        ]
        .width(Length::Fill)
        .align_items(Alignment::Center),
    )
    .into()
}

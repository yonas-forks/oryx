use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Row, Table, TableState},
    Frame,
};

use crate::packet::direction::TrafficDirection;

#[derive(Debug)]
pub struct TrafficDirectionFilter {
    pub state: TableState,
    pub selected_direction: Vec<TrafficDirection>,
    pub applied_direction: Vec<TrafficDirection>,
    pub terminate_ingress: Arc<AtomicBool>,
    pub terminate_egress: Arc<AtomicBool>,
}

impl TrafficDirectionFilter {
    pub fn new(direction: Vec<TrafficDirection>) -> Self {
        TrafficDirectionFilter {
            state: TableState::default(),
            selected_direction: direction,
            applied_direction: Vec::new(),
            terminate_ingress: Arc::new(AtomicBool::new(false)),
            terminate_egress: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn terminate(&mut self, direction: TrafficDirection) {
        match direction {
            TrafficDirection::Ingress => self.terminate_ingress.store(true, Ordering::Relaxed),
            TrafficDirection::Egress => self.terminate_egress.store(true, Ordering::Relaxed),
        }
    }

    pub fn select(&mut self) {
        if let Some(i) = self.state.selected() {
            let traffic_direction = match i {
                0 => TrafficDirection::Ingress,
                _ => TrafficDirection::Egress,
            };

            if self.selected_direction.contains(&traffic_direction) {
                self.selected_direction
                    .retain(|&direction| direction != traffic_direction);
            } else {
                self.selected_direction.push(traffic_direction);
            }
        }
    }

    pub fn apply(&mut self) {
        self.applied_direction = self.selected_direction.clone();
        self.selected_direction.clear();
    }

    pub fn render(&mut self, frame: &mut Frame, block: Rect, is_focused: bool) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(55),
                Constraint::Fill(1),
            ])
            .flex(Flex::Center)
            .split(block);

        let area = layout[1];

        let widths = [Constraint::Length(2), Constraint::Fill(1)];
        let filters = vec![
            Row::new(vec![
                {
                    if self.selected_direction.contains(&TrafficDirection::Ingress) {
                        " "
                    } else {
                        ""
                    }
                },
                "Ingress",
            ]),
            Row::new(vec![
                {
                    if self.selected_direction.contains(&TrafficDirection::Egress) {
                        " "
                    } else {
                        ""
                    }
                },
                "Egress",
            ]),
        ];

        let table = Table::new(filters, widths)
            .row_highlight_style(Style::new().bg(ratatui::style::Color::DarkGray));

        frame.render_widget(
            Block::new()
                .title(" Traffic Direction 󰞁 ")
                .title_style(Style::default().bold().fg(Color::Green))
                .title_alignment(Alignment::Center)
                .borders(Borders::LEFT)
                .border_type(if is_focused {
                    BorderType::Thick
                } else {
                    BorderType::default()
                })
                .border_style(Style::default().fg(Color::Green)),
            area,
        );

        frame.render_stateful_widget(
            table,
            area.inner(ratatui::layout::Margin {
                horizontal: 6,
                vertical: 2,
            }),
            &mut self.state,
        );
    }
}

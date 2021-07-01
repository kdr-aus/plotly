//! Candlestick plot

use crate::common::color::NamedColor;
use crate::common::{Calendar, Dim, Direction, HoverInfo, Label, Line, PlotType};
use crate::private;
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct Candlestick<T, O>
{
    r#type: PlotType,
    x: Vec<T>,
    open: Vec<O>,
    high: Vec<O>,
    low: Vec<O>,
    close: Vec<O>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis")]
    y_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "whiskerwidth")]
    whisker_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increasing: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    decreasing: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
}

impl<T, O> Candlestick<T, O>
{
    pub fn new(
        x: Vec<T>,
        open: Vec<O>,
        high: Vec<O>,
        low: Vec<O>,
        close: Vec<O>,
    ) -> Box<Self> 
    where
        T: Serialize + Default,
        O: Serialize + Default,
    {
        let iline = Line::new().width(1.0).color(NamedColor::Green);
        let dline = Line::new().width(1.0).color(NamedColor::Red);
        Box::new(Candlestick {
            r#type: PlotType::Candlestick,
            x,
            open,
            high,
            low,
            close,
            increasing: Some(Direction::Increasing { line: iline }),
            decreasing: Some(Direction::Decreasing { line: dline }),
            ..Default::default()
        })
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn visible(&mut self, visible: bool) -> &mut Self {
        self.visible = Some(visible);
        self
    }

    pub fn show_legend(&mut self, show_legend: bool) -> &mut Self {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend_group(&mut self, legend_group: &str) -> &mut Self {
        self.legend_group = Some(legend_group.to_owned());
        self
    }

    pub fn opacity(&mut self, opacity: f64) -> &mut Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = Some(Dim::Scalar(text.to_owned()));
        self
    }

    pub fn text_array<S: AsRef<str>>(&mut self, text: Vec<S>) -> &mut Self {
        let text = private::owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        self
    }

    pub fn hover_text(&mut self, hover_text: &str) -> &mut Self {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        self
    }

    pub fn hover_text_array<S: AsRef<str>>(&mut self, hover_text: Vec<S>) -> &mut Self {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        self
    }

    pub fn hover_info(&mut self, hover_info: HoverInfo) -> &mut Self {
        self.hover_info = Some(hover_info);
        self
    }

    pub fn x_axis(&mut self, axis: &str) -> &mut Self {
        self.x_axis = Some(axis.to_owned());
        self
    }

    pub fn y_axis(&mut self, axis: &str) -> &mut Self {
        self.y_axis = Some(axis.to_owned());
        self
    }

    pub fn line(&mut self, line: Line) -> &mut Self {
        self.line = Some(line);
        self
    }

    pub fn whisker_width(&mut self, whisker_width: f64) -> &mut Self {
        self.whisker_width = Some(whisker_width);
        self
    }

    pub fn increasing(&mut self, increasing: Direction) -> &mut Self {
        self.increasing = Some(increasing);
        self
    }

    pub fn decreasing(&mut self, decreasing: Direction) -> &mut Self {
        self.decreasing = Some(decreasing);
        self
    }

    pub fn hover_label(&mut self, hover_label: Label) -> &mut Self {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn x_calendar(&mut self, x_calendar: Calendar) -> &mut Self {
        self.x_calendar = Some(x_calendar);
        self
    }
}

impl<X, Y> Trace for Candlestick<X, Y>
where
    X: Serialize,
    Y: Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

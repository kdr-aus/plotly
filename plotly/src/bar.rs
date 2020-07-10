//! Bar plot

use crate::common::{
    Calendar, ConstrainText, Dim, ErrorData, Font, HoverInfo, Label, Marker, Orientation, PlotType,
    TextAnchor, TextPosition,
};
use crate::Trace;
use serde::Serialize;

use crate::private;

#[derive(Serialize, Debug, Default)]
pub struct Bar<X, Y> {
    x: Vec<X>,
    y: Vec<Y>,
    r#type: PlotType,
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
    ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Dim<usize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textposition")]
    text_position: Option<Dim<TextPosition>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "yaxis")]
    y_axis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "offsetgroup")]
    offset_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<Marker>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textangle")]
    text_angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "textfont")]
    text_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_x: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_y: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "constraintext")]
    constrain_text: Option<ConstrainText>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "insidetextanchor")]
    inside_text_anchor: Option<TextAnchor>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "insidetextfont")]
    inside_text_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "outsidetextfont")]
    outside_text_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y> Bar<X, Y> {
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self>
    where
        X: Serialize + Default,
        Y: Serialize + Default,
    {
        Box::new(Bar {
            x,
            y,
            r#type: PlotType::Bar,
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

    pub fn ids<S: AsRef<str>>(&mut self, ids: Vec<S>) -> &mut Self {
        let ids = private::owned_string_vector(ids);
        self.ids = Some(ids);
        self
    }

    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }

    pub fn offset(&mut self, offset: usize) -> &mut Self {
        self.offset = Some(Dim::Scalar(offset));
        self
    }

    pub fn offset_array(&mut self, offset: Vec<usize>) -> &mut Self {
        self.offset = Some(Dim::Vector(offset));
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

    pub fn text_position(&mut self, text_position: TextPosition) -> &mut Self {
        self.text_position = Some(Dim::Scalar(text_position));
        self
    }

    pub fn text_position_array(&mut self, text_position: Vec<TextPosition>) -> &mut Self {
        self.text_position = Some(Dim::Vector(text_position));
        self
    }

    pub fn text_template(&mut self, text_template: &str) -> &mut Self {
        self.text_template = Some(Dim::Scalar(text_template.to_owned()));
        self
    }

    pub fn text_template_array<S: AsRef<str>>(&mut self, text_template: Vec<S>) -> &mut Self {
        let text_template = private::owned_string_vector(text_template);
        self.text_template = Some(Dim::Vector(text_template));
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

    pub fn hover_template(&mut self, hover_template: &str) -> &mut Self {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
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

    pub fn hover_template_array<S: AsRef<str>>(&mut self, hover_template: Vec<S>) -> &mut Self {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        self
    }

    pub fn orientation(&mut self, orientation: Orientation) -> &mut Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn alignment_group(&mut self, alignment_group: &str) -> &mut Self {
        self.alignment_group = Some(alignment_group.to_owned());
        self
    }

    pub fn offset_group(&mut self, offset_group: &str) -> &mut Self {
        self.offset_group = Some(offset_group.to_owned());
        self
    }

    pub fn marker(&mut self, marker: Marker) -> &mut Self {
        self.marker = Some(marker);
        self
    }

    pub fn text_angle(&mut self, text_angle: f64) -> &mut Self {
        self.text_angle = Some(text_angle);
        self
    }

    pub fn text_font(&mut self, text_font: Font) -> &mut Self {
        self.text_font = Some(text_font);
        self
    }

    pub fn error_x(&mut self, error_x: ErrorData) -> &mut Self {
        self.error_x = Some(error_x);
        self
    }

    pub fn error_y(&mut self, error_y: ErrorData) -> &mut Self {
        self.error_y = Some(error_y);
        self
    }

    pub fn clip_on_axis(&mut self, clip_on_axis: bool) -> &mut Self {
        self.clip_on_axis = Some(clip_on_axis);
        self
    }

    pub fn constrain_text(&mut self, constrain_text: ConstrainText) -> &mut Self {
        self.constrain_text = Some(constrain_text);
        self
    }

    pub fn hover_label(&mut self, hover_label: Label) -> &mut Self {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn inside_text_anchor(&mut self, inside_text_anchor: TextAnchor) -> &mut Self {
        self.inside_text_anchor = Some(inside_text_anchor);
        self
    }

    pub fn inside_text_font(&mut self, inside_text_font: Font) -> &mut Self {
        self.inside_text_font = Some(inside_text_font);
        self
    }

    pub fn outside_text_font(&mut self, outside_text_font: Font) -> &mut Self {
        self.outside_text_font = Some(outside_text_font);
        self
    }

    pub fn x_calendar(&mut self, x_calendar: Calendar) -> &mut Self {
        self.x_calendar = Some(x_calendar);
        self
    }

    pub fn y_calendar(&mut self, y_calendar: Calendar) -> &mut Self {
        self.y_calendar = Some(y_calendar);
        self
    }
}

impl<X, Y> Trace for Bar<X, Y>
where
    X: Serialize,
    Y: Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

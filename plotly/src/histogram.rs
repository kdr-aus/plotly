//! Histogram plot

use crate::common::{Calendar, Dim, ErrorData, HoverInfo, Label, Marker, Orientation, PlotType};
use crate::Trace;
use serde::Serialize;

use crate::private;
use crate::private::copy_iterable_to_vec;

#[cfg(feature = "plotly_ndarray")]
use crate::ndarray::ArrayTraces;
#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix1, Ix2};

#[derive(Serialize, Clone, Debug)]
pub struct Bins {
    start: f64,
    end: f64,
    size: f64,
}

impl Bins {
    pub fn new(start: f64, end: f64, size: f64) -> Bins {
        Bins { start, end, size }
    }
}

#[derive(Serialize, Clone, Debug)]
pub enum HistFunc {
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "sum")]
    Sum,
    #[serde(rename = "avg")]
    Average,
    #[serde(rename = "min")]
    Minimum,
    #[serde(rename = "max")]
    Maximum,
}

#[derive(Serialize, Clone, Debug)]
pub enum HistNorm {
    #[serde(rename = "")]
    Default,
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "probability")]
    Probability,
    #[serde(rename = "density")]
    Density,
    #[serde(rename = "probability density")]
    ProbabilityDensity,
}

#[derive(Serialize, Clone, Debug)]
pub enum HistDirection {
    #[serde(rename = "increasing")]
    Increasing,
    #[serde(rename = "decreasing")]
    Decreasing,
}

#[derive(Serialize, Clone, Debug)]
pub enum CurrentBin {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
    #[serde(rename = "half")]
    Half,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct Cumulative {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    direction: Option<HistDirection>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "currentbin")]
    current_bin: Option<CurrentBin>,
}

impl Cumulative {
    pub fn new() -> Cumulative {
        Cumulative {
            enabled: None,
            direction: None,
            current_bin: None,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Cumulative {
        self.enabled = Some(enabled);
        self
    }

    pub fn direction(mut self, direction: HistDirection) -> Cumulative {
        self.direction = Some(direction);
        self
    }

    pub fn current_bin(mut self, current_bin: CurrentBin) -> Cumulative {
        self.current_bin = Some(current_bin);
        self
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Histogram<H> {
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
    x: Option<Vec<H>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<H>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
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
    orientation: Option<Orientation>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "histfunc")]
    hist_func: Option<HistFunc>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "histnorm")]
    hist_norm: Option<HistNorm>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "alignmentgroup")]
    alignment_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "offsetgroup")]
    offset_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nbinsx")]
    n_bins_x: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "nbinsy")]
    n_bins_y: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autobinx")]
    auto_bin_x: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autobiny")]
    auto_bin_y: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "bingroup")]
    bin_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xbins")]
    x_bins: Option<Bins>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ybins")]
    y_bins: Option<Bins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<Marker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_x: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_y: Option<ErrorData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cumulative: Option<Cumulative>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<H> Default for Histogram<H> {
    fn default() -> Self {
        Histogram {
            r#type: PlotType::Histogram,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            x: None,
            y: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            x_axis: None,
            y_axis: None,
            orientation: None,
            hist_func: None,
            hist_norm: None,
            alignment_group: None,
            offset_group: None,
            n_bins_x: None,
            n_bins_y: None,
            auto_bin_x: None,
            auto_bin_y: None,
            bin_group: None,
            x_bins: None,
            y_bins: None,
            marker: None,
            error_x: None,
            error_y: None,
            cumulative: None,
            hover_label: None,
            x_calendar: None,
            y_calendar: None,
        }
    }
}

impl<H> Histogram<H> {
    pub fn new(x: Vec<H>) -> Box<Self> {
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: Some(x),
            ..Default::default()
        })
    }

    pub fn new_xy(x: Vec<H>, y: Vec<H>) -> Box<Self> {
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }

    pub fn new_vertical(y: Vec<H>) -> Box<Self> {
        let y = copy_iterable_to_vec(y);
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            y: Some(y),
            ..Default::default()
        })
    }

    /// Produces `Histogram` traces from a 2 dimensional tensor (`traces_matrix`) indexed by `x`. This
    /// function requires the `ndarray` feature.
    ///
    /// # Arguments
    /// * `x`             - One dimensional array (or view) that represents the `x` axis coordinates.
    /// * `traces_matrix` - Two dimensional array (or view) containing the `y` axis coordinates of
    /// the traces.
    /// * `array_traces`  - Determines whether the traces are arranged in the matrix over the
    /// columns (`ArrayTraces::OverColumns`) or over the rows (`ArrayTraces::OverRows`).
    ///
    /// # Examples
    ///
    /// ```
    /// use plotly::common::Mode;
    /// use plotly::{Plot, Histogram, ArrayTraces};
    /// use ndarray::{Array, Ix1, Ix2};
    /// use rand_distr::{Distribution, Normal};
    /// use plotly::Layout;
    /// use plotly::layout::BarMode;
    ///
    /// fn ndarray_to_traces() {
    ///     let n: usize = 1_250;
    ///     let rng = rand::thread_rng();
    ///     let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    ///     let mut ys: Array<f64, Ix2> = Array::zeros((n, 4));
    ///     let mut count = 0.;
    ///     for mut row in ys.gencolumns_mut() {
    ///         let tmp: Vec<f64> = Normal::new(4. * count, 1.).unwrap().sample_iter(rng).take(n).collect();
    ///         for i in 0..row.len() {
    ///             row[i] = tmp[i];
    ///         }
    ///         count += 1.;
    ///     }
    ///
    ///     let traces = Histogram::default()
    ///         .opacity(0.5)
    ///         .auto_bin_x(true)
    ///         .to_traces(ys, ArrayTraces::OverColumns);
    ///
    ///     let layout = Layout::new().bar_mode(BarMode::Overlay);
    ///
    ///     let mut plot = Plot::new();
    ///     plot.set_layout(layout);
    ///     plot.add_traces(traces);
    ///     plot.show();
    /// }
    /// fn main() -> std::io::Result<()> {
    ///     ndarray_to_traces();
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "plotly_ndarray")]
    pub fn to_traces(
        &self,
        traces_matrix: Array<H, Ix2>,
        array_traces: ArrayTraces,
    ) -> Vec<Box<dyn Trace>> {
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let mut trace_vectors = private::trace_vectors_from(traces_matrix, array_traces);
        trace_vectors.reverse();
        while !trace_vectors.is_empty() {
            let mut sc = Box::new(self.clone());
            let data = trace_vectors.pop();
            if let Some(d) = data {
                sc.x = Some(d);
                traces.push(sc);
            }
        }

        traces
    }

    #[cfg(feature = "plotly_ndarray")]
    pub fn from_array(x: Array<H, Ix1>) -> Box<Self> {
        Box::new(Histogram {
            r#type: PlotType::Histogram,
            x: Some(x.to_vec()),
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

    pub fn hist_func(&mut self, hist_func: HistFunc) -> &mut Self {
        self.hist_func = Some(hist_func);
        self
    }

    pub fn hist_norm(&mut self, hist_norm: HistNorm) -> &mut Self {
        self.hist_norm = Some(hist_norm);
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

    pub fn n_bins_x(&mut self, n_bins_x: usize) -> &mut Self {
        self.n_bins_x = Some(n_bins_x);
        self
    }

    pub fn n_bins_y(&mut self, n_bins_y: usize) -> &mut Self {
        self.n_bins_y = Some(n_bins_y);
        self
    }

    pub fn auto_bin_x(&mut self, auto_bin_x: bool) -> &mut Self {
        self.auto_bin_x = Some(auto_bin_x);
        self
    }

    pub fn auto_bin_y(&mut self, auto_bin_y: bool) -> &mut Self {
        self.auto_bin_y = Some(auto_bin_y);
        self
    }

    pub fn bin_group(&mut self, bin_group: &str) -> &mut Self {
        self.bin_group = Some(bin_group.to_owned());
        self
    }

    pub fn x_bins(&mut self, x_bins: Bins) -> &mut Self {
        self.x_bins = Some(x_bins);
        self
    }

    pub fn y_bins(&mut self, y_bins: Bins) -> &mut Self {
        self.y_bins = Some(y_bins);
        self
    }

    pub fn marker(&mut self, marker: Marker) -> &mut Self {
        self.marker = Some(marker);
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

    pub fn cumulative(&mut self, cumulative: Cumulative) -> &mut Self {
        self.cumulative = Some(cumulative);
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

    pub fn y_calendar(&mut self, y_calendar: Calendar) -> &mut Self {
        self.y_calendar = Some(y_calendar);
        self
    }
}

impl<H> Trace for Histogram<H>
where
    H: Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

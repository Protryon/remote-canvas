use crate::result::*;
use crate::websocket::*;
use async_std::sync;
pub use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(Clone)]
pub struct CanvasGenerator {
    pub(crate) sender: sync::Sender<mpsc::Receiver<WSMessage>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WebglPowerPreference {
    #[serde(rename = "default")]
    DefaultPower,
    #[serde(rename = "high-performance")]
    HighPerformance,
    #[serde(rename = "low-power")]
    LowPower,
}

impl Default for WebglPowerPreference {
    fn default() -> WebglPowerPreference {
        WebglPowerPreference::DefaultPower
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ContextDataWebgl {
    #[serde(default)]
    pub alpha: bool,
    #[serde(default)]
    pub desynchronized: bool,
    #[serde(default)]
    pub antialias: bool,
    #[serde(default)]
    pub depth: bool,
    #[serde(default)]
    #[serde(rename = "failIfMajorPerformanceCaveat")]
    pub fail_if_major_performance_caveat: bool,
    #[serde(default)]
    #[serde(rename = "powerPreference")]
    pub power_preference: WebglPowerPreference,
    #[serde(default)]
    #[serde(rename = "premultipliedAlpha")]
    pub premultiplied_alpha: bool,
    #[serde(default)]
    #[serde(rename = "preserveDrawingBuffer")]
    pub preserve_drawing_buffer: bool,
    #[serde(default)]
    pub stencil: bool,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ContextData2D {
    #[serde(default)]
    pub alpha: bool,
    #[serde(default)]
    pub desynchronized: bool,
}

fn map_js_error(result: Result<WsMessageResponse>) -> Result<()> {
    match result {
        Ok(WsMessageResponse::JsError { message }) => {
            return Err(canvas_error!("js error: {}", message));
        }
        Ok(_) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn map_js_error_data(result: Result<WsMessageResponse>) -> Result<WsMessageResponse> {
    match result {
        Ok(WsMessageResponse::JsError { message }) => {
            return Err(canvas_error!("js error: {}", message));
        }
        Ok(a) => {
            return Ok(a);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

impl CanvasGenerator {
    pub async fn make_2d_canvas(
        &self,
        data: ContextData2D,
        width: u32,
        height: u32,
    ) -> Result<Canvas2D> {
        let (sender, receiver) = mpsc::channel::<WSMessage>(1024);
        self.sender.send(receiver).await;
        let mut canvas = Canvas2D {
            uuid: Uuid::new_v4(),
            sender,
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 10.0,
            line_dash_offset: 0.0,
            font: "10px sans-serif".to_string(),
            text_align: TextAlign::Start,
            text_baseline: TextBaseline::Alphabetic,
            text_direction: TextDirection::Inherit,
            fill_style: "#000".to_string(),
            stroke_style: "#000".to_string(),
            shadow_blur: 0.0,
            shadow_color: "#000".to_string(),
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            global_alpha: 1.0,
            global_composite_operation: "source-over".to_string(),
            image_smoothing_enabled: true,
            image_smoothing_quality: Some(ImageSmoothingQuality::Low),
            filter: None,
            width,
            height,
        };
        canvas.initialize(data).await?;
        return Ok(canvas);
    }
}

pub struct Canvas2D {
    uuid: Uuid,
    sender: mpsc::Sender<WSMessage>,
    line_width: f64,
    line_cap: LineCap,
    line_join: LineJoin,
    miter_limit: f64,
    line_dash_offset: f64,
    font: String,
    text_align: TextAlign,
    text_baseline: TextBaseline,
    text_direction: TextDirection,
    fill_style: String,
    stroke_style: String,
    shadow_blur: f64,
    shadow_color: String,
    shadow_offset_x: f64,
    shadow_offset_y: f64,
    global_alpha: f64,
    global_composite_operation: String,
    image_smoothing_enabled: bool,
    image_smoothing_quality: Option<ImageSmoothingQuality>,
    filter: Option<String>,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextMetrics {
    pub width: f64,
    #[serde(rename = "actualBoundingBoxLeft")]
    pub actual_bounding_box_left: f64,
    #[serde(rename = "actualBoundingBoxRight")]
    pub actual_bounding_box_right: f64,
    #[serde(rename = "fontBoundingBoxAscent")]
    pub font_bounding_box_ascent: f64,
    #[serde(rename = "fontBoundingBoxDescent")]
    pub font_bounding_box_descent: f64,
    #[serde(rename = "actualBoundingBoxAscent")]
    pub actual_bounding_box_ascent: f64,
    #[serde(rename = "actualBoundingBoxDescent")]
    pub actual_bounding_box_descent: f64,
    #[serde(rename = "emHeightAscent")]
    pub em_height_ascent: f64,
    #[serde(rename = "emHeightDescent")]
    pub em_height_descent: f64,
    #[serde(rename = "hangingBaseline")]
    pub hanging_baseline: f64,
    #[serde(rename = "alphabeticBaseline")]
    pub alphabetic_baseline: f64,
    #[serde(rename = "ideographicBaseline")]
    pub ideographic_baseline: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LineJoin {
    Round,
    Bevel,
    Miter,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TextBaseline {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideographic,
    Bottom,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TextDirection {
    Ltr,
    Rtl,
    Inherit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ImageSmoothingQuality {
    Low,
    Medium,
    High,
}

pub type DomMatrix = [f64; 6];

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PatternRepitition {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageData {
    pub data: Vec<u32>, // (y*width + x) -> RGBA
    pub width: u32,
    pub height: u32,
}

impl ImageData {
    pub fn get_pixel(&self, x: u32, y: u32) -> u32 {
        if x > self.width || y > self.height {
            return 0;
        }
        return self.data[(y * self.width + x) as usize];
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: u32) {
        if x > self.width || y > self.height {
            return;
        }
        self.data[(y * self.width + x) as usize] = pixel;
    }
}

impl Canvas2D {
    async fn send(&mut self, data: WSMessageData) -> Result<WsMessageResponse> {
        let (sender, receiver) = oneshot::channel::<WsMessageResponse>();
        self.sender
            .send(WSMessage {
                context: self.uuid,
                data,
                response: Some(sender),
            })
            .await?;
        receiver
            .await
            .map_err(|_| canvas_error!("failed to receiver response from client") as Error)
    }

    async fn initialize(&mut self, data: ContextData2D) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::Init2DCanvas {
                data,
                width: self.width,
                height: self.height,
            })
            .await,
        )
    }

    pub async fn measure_text(&mut self, text: String) -> Result<TextMetrics> {
        let response = map_js_error_data(self.send(WSMessageData::MeasureText { text }).await)?;
        match response {
            WsMessageResponse::MeasureText { text_metrics } => Ok(text_metrics),
            _ => Err(canvas_error!(
                "bad packet type received for measure_text: {:?}",
                response
            )),
        }
    }

    pub fn get_line_width(&self) -> f64 {
        self.line_width
    }

    pub async fn set_line_width(&mut self, line_width: f64) -> Result<()> {
        let result = map_js_error(self.send(WSMessageData::SetLineWidth { line_width }).await);
        if result.is_ok() {
            self.line_width = line_width;
        }
        result
    }

    pub fn get_line_cap(&self) -> LineCap {
        self.line_cap
    }

    pub async fn set_line_cap(&mut self, line_cap: LineCap) -> Result<()> {
        let result = map_js_error(self.send(WSMessageData::SetLineCap { line_cap }).await);
        if result.is_ok() {
            self.line_cap = line_cap;
        }
        result
    }

    pub fn get_line_join(&self) -> LineJoin {
        self.line_join
    }

    pub async fn set_line_join(&mut self, line_join: LineJoin) -> Result<()> {
        let result = map_js_error(self.send(WSMessageData::SetLineJoin { line_join }).await);
        if result.is_ok() {
            self.line_join = line_join;
        }
        result
    }

    pub fn get_miter_limit(&self) -> f64 {
        self.miter_limit
    }

    pub async fn set_miter_limit(&mut self, miter_limit: f64) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetMiterLimit { miter_limit })
                .await,
        );
        if result.is_ok() {
            self.miter_limit = miter_limit;
        }
        result
    }

    pub async fn get_line_dash(&mut self) -> Result<Vec<f64>> {
        let response = map_js_error_data(self.send(WSMessageData::GetLineDash {}).await)?;
        match response {
            WsMessageResponse::LineDash { line_dash } => Ok(line_dash),
            _ => Err(canvas_error!(
                "bad packet type received for get_line_dash: {:?}",
                response
            )),
        }
    }

    pub async fn set_line_dash(&mut self, line_dash: Vec<f64>) -> Result<Vec<f64>> {
        let response =
            map_js_error_data(self.send(WSMessageData::SetLineDash { line_dash }).await)?;
        match response {
            WsMessageResponse::LineDash { line_dash } => Ok(line_dash),
            _ => Err(canvas_error!(
                "bad packet type received for set_line_dash: {:?}",
                response
            )),
        }
    }

    pub fn get_line_dash_offset(&self) -> f64 {
        self.line_dash_offset
    }

    pub async fn set_line_dash_offset(&mut self, line_dash_offset: f64) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetLineDashOffset { line_dash_offset })
                .await,
        );
        if result.is_ok() {
            self.line_dash_offset = line_dash_offset;
        }
        result
    }

    pub fn get_font(&self) -> &str {
        &self.font
    }

    pub async fn set_font(&mut self, font: &str) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetFont {
                font: font.to_string(),
            })
            .await,
        );
        if result.is_ok() {
            self.font = font.to_string();
        }
        result
    }

    pub fn get_text_align(&self) -> TextAlign {
        self.text_align
    }

    pub async fn set_text_align(&mut self, text_align: TextAlign) -> Result<()> {
        let result = map_js_error(self.send(WSMessageData::SetTextAlign { text_align }).await);
        if result.is_ok() {
            self.text_align = text_align;
        }
        result
    }

    pub fn get_text_baseline(&self) -> TextBaseline {
        self.text_baseline
    }

    pub async fn set_text_baseline(&mut self, text_baseline: TextBaseline) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetTextBaseline { text_baseline })
                .await,
        );
        if result.is_ok() {
            self.text_baseline = text_baseline;
        }
        result
    }

    pub fn get_text_direction(&self) -> TextDirection {
        self.text_direction
    }

    pub async fn set_text_direction(&mut self, text_direction: TextDirection) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetTextDirection { text_direction })
                .await,
        );
        if result.is_ok() {
            self.text_direction = text_direction;
        }
        result
    }

    pub fn get_fill_style(&self) -> &str {
        &self.fill_style
    }

    pub async fn set_fill_style(&mut self, fill_style: &str) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetFillStyle {
                fill_style: fill_style.to_string(),
            })
            .await,
        );
        if result.is_ok() {
            self.fill_style = fill_style.to_string();
        }
        result
    }

    pub fn get_stroke_style(&self) -> &str {
        &self.stroke_style
    }

    pub async fn set_stroke_style(&mut self, stroke_style: &str) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetStrokeStyle {
                stroke_style: stroke_style.to_string(),
            })
            .await,
        );
        if result.is_ok() {
            self.stroke_style = stroke_style.to_string();
        }
        result
    }

    pub fn get_shadow_blur(&self) -> f64 {
        self.shadow_blur
    }

    pub async fn set_shadow_blur(&mut self, shadow_blur: f64) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetShadowBlur { shadow_blur })
                .await,
        );
        if result.is_ok() {
            self.shadow_blur = shadow_blur;
        }
        result
    }

    pub fn get_shadow_color(&self) -> &str {
        &self.shadow_color
    }

    pub async fn set_shadow_color(&mut self, shadow_color: &str) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetShadowColor {
                shadow_color: shadow_color.to_string(),
            })
            .await,
        );
        if result.is_ok() {
            self.shadow_color = shadow_color.to_string();
        }
        result
    }

    pub fn get_shadow_offset_x(&self) -> f64 {
        self.shadow_offset_x
    }

    pub async fn set_shadow_offset_x(&mut self, shadow_offset_x: f64) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetShadowOffsetX { shadow_offset_x })
                .await,
        );
        if result.is_ok() {
            self.shadow_offset_x = shadow_offset_x;
        }
        result
    }

    pub fn get_shadow_offset_y(&self) -> f64 {
        self.shadow_offset_y
    }

    pub async fn set_shadow_offset_y(&mut self, shadow_offset_y: f64) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetShadowOffsetY { shadow_offset_y })
                .await,
        );
        if result.is_ok() {
            self.shadow_offset_y = shadow_offset_y;
        }
        result
    }

    pub fn get_global_alpha(&self) -> f64 {
        self.global_alpha
    }

    pub async fn set_global_alpha(&mut self, global_alpha: f64) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetGlobalAlpha { global_alpha })
                .await,
        );
        if result.is_ok() {
            self.global_alpha = global_alpha;
        }
        result
    }

    pub fn get_global_composite_operation(&self) -> &str {
        &self.global_composite_operation
    }

    pub async fn set_global_composite_operation(
        &mut self,
        global_composite_operation: &str,
    ) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetGlobalCompositeOperation {
                global_composite_operation: global_composite_operation.to_string(),
            })
            .await,
        );
        if result.is_ok() {
            self.global_composite_operation = global_composite_operation.to_string();
        }
        result
    }

    pub fn get_image_smoothing_enabled(&self) -> bool {
        self.image_smoothing_enabled
    }

    pub async fn set_image_smoothing_enabled(
        &mut self,
        image_smoothing_enabled: bool,
    ) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetImageSmoothingEnabled {
                image_smoothing_enabled,
            })
            .await,
        );
        if result.is_ok() {
            self.image_smoothing_enabled = image_smoothing_enabled;
        }
        result
    }

    pub fn get_image_smoothing_quality(&self) -> Option<ImageSmoothingQuality> {
        self.image_smoothing_quality
    }

    pub async fn set_image_smoothing_quality(
        &mut self,
        image_smoothing_quality: Option<ImageSmoothingQuality>,
    ) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetImageSmoothingQuality {
                image_smoothing_quality,
            })
            .await,
        );
        if result.is_ok() {
            self.image_smoothing_quality = image_smoothing_quality;
        }
        result
    }

    pub fn get_filter(&self) -> Option<&str> {
        self.filter.as_ref().map(|o| &**o)
    }

    pub async fn set_filter(&mut self, filter: Option<&str>) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetFilter {
                filter: filter.map(|o| o.to_string()),
            })
            .await,
        );
        if result.is_ok() {
            self.filter = filter.map(|o| o.to_string());
        }
        result
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub async fn set_width(&mut self, width: u32) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetWidth {
                width: width as i32,
            })
            .await,
        );
        if result.is_ok() {
            self.width = width;
        }
        result
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub async fn set_height(&mut self, height: u32) -> Result<()> {
        let result = map_js_error(
            self.send(WSMessageData::SetHeight {
                height: height as i32,
            })
            .await,
        );
        if result.is_ok() {
            self.height = height;
        }
        result
    }

    pub async fn clear_rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::ClearRect {
                x,
                y,
                width,
                height,
            })
            .await,
        )
    }

    pub async fn fill_rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::FillRect {
                x,
                y,
                width,
                height,
            })
            .await,
        )
    }

    pub async fn stroke_rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::StrokeRect {
                x,
                y,
                width,
                height,
            })
            .await,
        )
    }

    pub async fn fill_text(
        &mut self,
        text: &str,
        x: f64,
        y: f64,
        max_width: Option<f64>,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::FillText {
                text: text.to_string(),
                x,
                y,
                max_width,
            })
            .await,
        )
    }

    pub async fn stroke_text(
        &mut self,
        text: &str,
        x: f64,
        y: f64,
        max_width: Option<f64>,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::StrokeText {
                text: text.to_string(),
                x,
                y,
                max_width,
            })
            .await,
        )
    }

    pub async fn create_linear_gradient(
        &mut self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        stops: Vec<(f64, String)>,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::CreateLinearGradient {
                x0,
                y0,
                x1,
                y1,
                stops,
            })
            .await,
        )
    }

    pub async fn create_radial_gradient(
        &mut self,
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
        stops: Vec<(f64, String)>,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::CreateRadialGradient {
                x0,
                y0,
                r0,
                x1,
                y1,
                r1,
                stops,
            })
            .await,
        )
    }

    pub async fn create_pattern(&mut self, url: &str, repitition: PatternRepitition) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::CreatePattern {
                url: url.to_string(),
                repitition,
            })
            .await,
        )
    }

    pub async fn begin_path(&mut self, path_uuid: Option<Uuid>) -> Result<()> {
        map_js_error(self.send(WSMessageData::BeginPath { path_uuid }).await)
    }

    pub async fn close_path(&mut self, path_uuid: Option<Uuid>) -> Result<()> {
        map_js_error(self.send(WSMessageData::ClosePath { path_uuid }).await)
    }

    pub async fn move_to(&mut self, path_uuid: Option<Uuid>, x: f64, y: f64) -> Result<()> {
        map_js_error(self.send(WSMessageData::MoveTo { path_uuid, x, y }).await)
    }

    pub async fn line_to(&mut self, path_uuid: Option<Uuid>, x: f64, y: f64) -> Result<()> {
        map_js_error(self.send(WSMessageData::LineTo { path_uuid, x, y }).await)
    }

    pub async fn bezier_curve_to(
        &mut self,
        path_uuid: Option<Uuid>,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::BezierCurveTo {
                path_uuid,
                cp1x,
                cp1y,
                cp2x,
                cp2y,
                x,
                y,
            })
            .await,
        )
    }

    pub async fn quadratic_curve_to(
        &mut self,
        path_uuid: Option<Uuid>,
        cpx: f64,
        cpy: f64,
        x: f64,
        y: f64,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::QuadraticCurveTo {
                path_uuid,
                cpx,
                cpy,
                x,
                y,
            })
            .await,
        )
    }

    pub async fn arc(
        &mut self,
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::Arc {
                path_uuid,
                x,
                y,
                radius,
                start_angle,
                end_angle,
                anticlockwise,
            })
            .await,
        )
    }

    pub async fn arc_to(
        &mut self,
        path_uuid: Option<Uuid>,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::ArcTo {
                path_uuid,
                x1,
                y1,
                x2,
                y2,
                radius,
            })
            .await,
        )
    }

    pub async fn ellipse(
        &mut self,
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::Ellipse {
                path_uuid,
                x,
                y,
                radius_x,
                radius_y,
                rotation,
                start_angle,
                end_angle,
                anticlockwise,
            })
            .await,
        )
    }

    pub async fn rect(
        &mut self,
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::Rect {
                path_uuid,
                x,
                y,
                width,
                height,
            })
            .await,
        )
    }

    pub async fn fill(&mut self, path_uuid: Option<Uuid>, is_even_odd: bool) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::Fill {
                path_uuid,
                is_even_odd,
            })
            .await,
        )
    }

    pub async fn stroke(&mut self, path_uuid: Option<Uuid>) -> Result<()> {
        map_js_error(self.send(WSMessageData::Stroke { path_uuid }).await)
    }

    pub async fn clip(&mut self, path_uuid: Option<Uuid>, is_even_odd: bool) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::Clip {
                path_uuid,
                is_even_odd,
            })
            .await,
        )
    }

    pub async fn is_point_in_path(
        &mut self,
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
        is_even_odd: bool,
    ) -> Result<bool> {
        let response = map_js_error_data(
            self.send(WSMessageData::IsPointInPath {
                path_uuid,
                x,
                y,
                is_even_odd,
            })
            .await,
        )?;
        match response {
            WsMessageResponse::IsPointIn { is_in } => Ok(is_in),
            _ => Err(canvas_error!(
                "bad packet type received for is_point_in_path: {:?}",
                response
            )),
        }
    }

    pub async fn is_point_in_stroke(
        &mut self,
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
    ) -> Result<bool> {
        let response = map_js_error_data(
            self.send(WSMessageData::IsPointInStroke { path_uuid, x, y })
                .await,
        )?;
        match response {
            WsMessageResponse::IsPointIn { is_in } => Ok(is_in),
            _ => Err(canvas_error!(
                "bad packet type received for is_point_in_stroke: {:?}",
                response
            )),
        }
    }

    pub async fn get_transform(&mut self) -> Result<DomMatrix> {
        let response = map_js_error_data(self.send(WSMessageData::GetTransform {}).await)?;
        match response {
            WsMessageResponse::Transform { matrix } => Ok(matrix),
            _ => Err(canvas_error!(
                "bad packet type received for get_transform: {:?}",
                response
            )),
        }
    }

    pub async fn rotate(&mut self, angle: f64) -> Result<()> {
        map_js_error(self.send(WSMessageData::Rotate { angle }).await)
    }

    pub async fn scale(&mut self, x: f64, y: f64) -> Result<()> {
        map_js_error(self.send(WSMessageData::Scale { x, y }).await)
    }

    pub async fn translate(&mut self, x: f64, y: f64) -> Result<()> {
        map_js_error(self.send(WSMessageData::Translate { x, y }).await)
    }

    pub async fn transform(&mut self, matrix: DomMatrix) -> Result<()> {
        map_js_error(self.send(WSMessageData::Transform { matrix }).await)
    }

    pub async fn set_transform(&mut self, matrix: DomMatrix) -> Result<()> {
        map_js_error(self.send(WSMessageData::SetTransform { matrix }).await)
    }

    pub async fn draw_image(
        &mut self,
        url: &str,
        source: Option<(f64, f64, f64, f64)>,
        dx: f64,
        dy: f64,
        d_dims: Option<(f64, f64)>,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::DrawImage {
                url: url.to_string(),
                source,
                dx,
                dy,
                d_dims,
            })
            .await,
        )
    }

    pub async fn get_image_data(
        &mut self,
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
    ) -> Result<ImageData> {
        let response = map_js_error_data(
            self.send(WSMessageData::GetImageData { sx, sy, sw, sh })
                .await,
        )?;
        match response {
            WsMessageResponse::ImageData { image_data } => Ok(image_data),
            _ => Err(canvas_error!(
                "bad packet type received for get_image_data: {:?}",
                response
            )),
        }
    }

    pub async fn set_image_data(
        &mut self,
        image_data: ImageData,
        dx: f64,
        dy: f64,
        dirty_pos: Option<(f64, f64)>,
        dirty_dims: Option<(f64, f64)>,
    ) -> Result<()> {
        map_js_error(
            self.send(WSMessageData::SetImageData {
                image_data,
                dx,
                dy,
                dirty_pos,
                dirty_dims,
            })
            .await,
        )
    }

    pub async fn save(&mut self) -> Result<()> {
        map_js_error(self.send(WSMessageData::Save {}).await)
    }

    pub async fn restore(&mut self) -> Result<()> {
        map_js_error(self.send(WSMessageData::Restore {}).await)
    }

    pub async fn to_data_url(&mut self, mime_type: &str, quality: Option<f64>) -> Result<String> {
        let response = map_js_error_data(
            self.send(WSMessageData::ToDataUrl {
                mime_type: mime_type.to_string(),
                quality,
            })
            .await,
        )?;
        match response {
            WsMessageResponse::DataUrl { data_url } => Ok(data_url),
            _ => Err(canvas_error!(
                "bad packet type received for to_data_url: {:?}",
                response
            )),
        }
    }
}

async fn send_out_of_band(mut sender: mpsc::Sender<WSMessage>, context: Uuid, data: WSMessageData) {
    sender
        .send(WSMessage {
            context,
            data,
            response: None,
        })
        .await
        .unwrap_or(());
}

impl Drop for Canvas2D {
    fn drop(&mut self) {
        tokio::spawn(send_out_of_band(
            self.sender.clone(),
            self.uuid,
            WSMessageData::Destroy2DCanvas {},
        ));
    }
}

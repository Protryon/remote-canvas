use crate::canvas::*;
use crate::result::*;
use async_std::sync;
use futures::future::FutureExt;
use futures::select;
use futures::stream::SelectAll;
use futures::SinkExt;
use futures::StreamExt;
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::delay_for;
use tungstenite::protocol::Message;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum WSMessageData {
    Init2DCanvas {
        data: ContextData2D,
        width: u32,
        height: u32,
    },
    Destroy2DCanvas {},
    MeasureText {
        text: String,
    },
    SetLineWidth {
        line_width: f64,
    },
    SetLineCap {
        line_cap: LineCap,
    },
    SetLineJoin {
        line_join: LineJoin,
    },
    SetMiterLimit {
        miter_limit: f64,
    },
    GetLineDash {},
    SetLineDash {
        line_dash: Vec<f64>,
    },
    SetLineDashOffset {
        line_dash_offset: f64,
    },
    SetFont {
        font: String,
    },
    SetTextAlign {
        text_align: TextAlign,
    },
    SetTextBaseline {
        text_baseline: TextBaseline,
    },
    SetTextDirection {
        text_direction: TextDirection,
    },
    SetFillStyle {
        fill_style: String,
    },
    SetStrokeStyle {
        stroke_style: String,
    },
    SetShadowBlur {
        shadow_blur: f64,
    },
    SetShadowColor {
        shadow_color: String,
    },
    SetShadowOffsetX {
        shadow_offset_x: f64,
    },
    SetShadowOffsetY {
        shadow_offset_y: f64,
    },
    SetGlobalAlpha {
        global_alpha: f64,
    },
    SetGlobalCompositeOperation {
        global_composite_operation: String,
    },
    SetImageSmoothingEnabled {
        image_smoothing_enabled: bool,
    },
    SetImageSmoothingQuality {
        image_smoothing_quality: Option<ImageSmoothingQuality>,
    },
    SetFilter {
        filter: Option<String>,
    },
    SetWidth {
        width: i32,
    },
    SetHeight {
        height: i32,
    },
    ClearRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    FillRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    StrokeRect {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    FillText {
        text: String,
        x: f64,
        y: f64,
        max_width: Option<f64>,
    },
    StrokeText {
        text: String,
        x: f64,
        y: f64,
        max_width: Option<f64>,
    },
    CreateLinearGradient {
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        stops: Vec<(f64, String)>,
    },
    CreateRadialGradient {
        x0: f64,
        y0: f64,
        r0: f64,
        x1: f64,
        y1: f64,
        r1: f64,
        stops: Vec<(f64, String)>,
    },
    CreatePattern {
        url: String,
        repitition: PatternRepitition,
    },
    BeginPath {
        path_uuid: Option<Uuid>,
    },
    ClosePath {
        path_uuid: Option<Uuid>,
    },
    MoveTo {
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
    },
    LineTo {
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
    },
    BezierCurveTo {
        path_uuid: Option<Uuid>,
        cp1x: f64,
        cp1y: f64,
        cp2x: f64,
        cp2y: f64,
        x: f64,
        y: f64,
    },
    QuadraticCurveTo {
        path_uuid: Option<Uuid>,
        cpx: f64,
        cpy: f64,
        x: f64,
        y: f64,
    },
    Arc {
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    },
    ArcTo {
        path_uuid: Option<Uuid>,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        radius: f64,
    },
    Ellipse {
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
        radius_x: f64,
        radius_y: f64,
        rotation: f64,
        start_angle: f64,
        end_angle: f64,
        anticlockwise: bool,
    },
    Rect {
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    Fill {
        path_uuid: Option<Uuid>,
        is_even_odd: bool,
    },
    Stroke {
        path_uuid: Option<Uuid>,
    },
    Clip {
        path_uuid: Option<Uuid>,
        is_even_odd: bool,
    },
    IsPointInPath {
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
        is_even_odd: bool,
    },
    IsPointInStroke {
        path_uuid: Option<Uuid>,
        x: f64,
        y: f64,
    },
    GetTransform {},
    Rotate {
        angle: f64,
    },
    Scale {
        x: f64,
        y: f64,
    },
    Translate {
        x: f64,
        y: f64,
    },
    Transform {
        matrix: DomMatrix,
    },
    SetTransform {
        matrix: DomMatrix,
    },
    DrawImage {
        url: String,
        source: Option<(f64, f64, f64, f64)>, // sx, sy, s_width, s_height
        dx: f64,
        dy: f64,
        d_dims: Option<(f64, f64)>, // d_width, d_height
    },
    GetImageData {
        sx: f64,
        sy: f64,
        sw: f64,
        sh: f64,
    },
    SetImageData {
        image_data: ImageData,
        dx: f64,
        dy: f64,
        dirty_pos: Option<(f64, f64)>,
        dirty_dims: Option<(f64, f64)>,
    },
    Save {},
    Restore {},
    ToDataUrl {
        mime_type: String,
        quality: Option<f64>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum WsMessageResponse {
    JsError { message: String },
    Success {},
    MeasureText { text_metrics: TextMetrics },
    LineDash { line_dash: Vec<f64> },
    IsPointIn { is_in: bool },
    Transform { matrix: DomMatrix },
    ImageData { image_data: ImageData },
    DataUrl { data_url: String },
}

#[derive(Debug)]
pub(crate) struct WSMessage {
    pub data: WSMessageData,
    pub context: Uuid,
    pub response: Option<sync::Sender<WsMessageResponse>>,
}

#[derive(Serialize, Deserialize)]
struct WSNetMessage {
    data: WSMessageData,
    context: Uuid,
    txn_uuid: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
struct WSNetResponse {
    data: WsMessageResponse,
    txn_uuid: Uuid,
}

async fn handle_connection(
    raw_stream: TcpStream,
    address: SocketAddr,
    mut receiver: sync::Receiver<mpsc::Receiver<WSMessage>>,
) -> Result<()> {
    debug!("Incoming WebSocket connection from: {}", address);

    let mut ws_stream = tokio_tungstenite::accept_async(raw_stream).await?;
    debug!("WebSocket connection established: {}", address);
    let mut living_jobs: HashMap<Uuid, sync::Sender<WsMessageResponse>> = HashMap::new();
    let mut contexts = SelectAll::new();
    loop {
        select! {
            ctx = receiver.next().fuse() => {
                if let Some(ctx) = ctx {
                    contexts.push(ctx);
                } else {
                    break;
                }
            },
            job = contexts.next() => {
                if let Some(job) = job {
                    let txn = Uuid::new_v4();
                    let net_msg = WSNetMessage { data: job.data, txn_uuid: txn, context: job.context };
                    ws_stream.send(Message::Text(serde_json::to_string(&net_msg).unwrap())).await?;
                    if let Some(response) = job.response {
                        living_jobs.insert(txn, response);
                    }
                }
            },
            msg = ws_stream.next().fuse() => {
                if let Some(Ok(Message::Text(msg))) = msg {
                    let msg: WSNetResponse = serde_json::from_str(&msg)?;
                    let job = living_jobs.remove(&msg.txn_uuid);
                    match job {
                        Some(job) => {
                            job.send(msg.data).await;
                        },
                        None => {
                            return Err(canvas_error!("invalid txn id in response: {:?}", &msg));
                        }
                    }
                } else {
                    break;
                }
            },
        }
    }

    debug!("{} disconnected", &address);
    Ok(())
}

// deals with non-Send Errors
async fn handle_connection_wrapper(
    raw_stream: TcpStream,
    address: SocketAddr,
    receiver: sync::Receiver<mpsc::Receiver<WSMessage>>,
) -> Result<()> {
    let result = handle_connection(raw_stream, address, receiver).await;
    match result {
        Err(e) => {
            warn!("Error in websocket connection: {:?}", e);
            Err(e)
        }
        _ => Ok(()),
    }
}

pub(crate) async fn start_server(
    address: String,
    receiver: sync::Receiver<mpsc::Receiver<WSMessage>>,
) -> Result<()> {
    let mut listener = TcpListener::bind(&address).await?;
    info!("websocket server listening on: {}", address);

    loop {
        let result = listener.accept().await;
        if result.is_err() {
            error!(
                "error accepting connection: {:?}, sleeping 1 sec",
                result.err().unwrap()
            );
            delay_for(Duration::from_secs(1)).await;
            continue;
        }
        let (stream, addr) = result.unwrap();
        tokio::spawn(handle_connection_wrapper(stream, addr, receiver.clone()));
    }
}

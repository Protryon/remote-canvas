const cookies = Object.fromEntries(document.cookie.split('; ')
    .map(c => [c.slice(0, c.indexOf('=')), c.slice(c.indexOf('=') + 1)]));

const panic = message => {
    document.write(message);
    console.error(message);
    throw new Error(message);
}

const websocketAddress = cookies["websocket_addr"];
if (websocketAddress == null) {
    panic("didn't find websocket address");
}

let websocket = null;

const connectionOpened = () => {

}

/*
    JsError { message: String },
    Success {},
    MeasureText { text_metrics: TextMetrics },
    LineDash { line_dash: Vec<f64> },
    IsPointIn { is_in: bool },
    Transform { matrix: DomMatrix },
    ImageData { image_data: ImageData },
    DataUrl { data_url: String },


#[derive(Serialize, Deserialize)]
struct WSNetResponse {
    data: WSMessageData,
    txn_uuid: Uuid,
}
*/

const send = (uuid, id, packet) => {
    websocket.send(JSON.stringify({ data: { [id]: packet }, txn_uuid: uuid }));
}

const genericSuccess = (uuid) => {
    send(uuid, 'Success', {});
}

const sendError = (uuid, e) => {
    send(uuid, 'JsError', { message: e.toString() });
}

const canvii = {};
window.canvii = canvii;

const getTarget = (path_uuid, canvi) => {
    if (path_uuid != null) {
        target = canvi.paths[target.path_uuid];
    } else {
        target = canvi.ctx;
    }
    return target;
};

const getTargetImplied = (path_uuid, canvi) => {
    if (path_uuid != null) {
        target = canvi.paths[target.path_uuid];
    } else {
        target = void 0;
    }
    return target;
};

const receiveMessage = message => {
    const parsed = JSON.parse(message.data);
    console.log(message.data);
    let { data, txn_uuid, context } = parsed;
    try {
        const [id, packet] = Object.entries(data)[0];
        const canvi = canvii[context];
        const canvas = canvi == null ? null : canvi.canvas;
        const ctx = canvi == null ? null : canvi.ctx;
        if (id == "Init2DCanvas") {
            const canvas = document.createElement('canvas');
            canvas.width = packet.width;
            canvas.height = packet.height;
            const ctx = canvas.getContext('2d', packet.data);
            canvii[context] = {
                canvas,
                ctx,
                paths: {},
            };
            genericSuccess(txn_uuid);
        } else if (id == "Destroy2DCanvas") {
            delete canvii[context];
            // nobody is listening
            //genericSuccess(txn_uuid);
        } else if (id == "MeasureText") {
            const metrics = ctx.measureText(packet.text);
            send(txn_uuid, 'MeasureText', { text_metrics: metrics });
        } else if (id == "SetLineWidth") {
            ctx.lineWidth = packet.line_width;
            genericSuccess(txn_uuid);
        } else if (id == "SetLineCap") {
            ctx.lineCap = packet.line_cap.toLowerCase();
            genericSuccess(txn_uuid);
        } else if (id == "SetLineJoin") {
            ctx.lineJoin = packet.line_join.toLowerCase();
            genericSuccess(txn_uuid);
        } else if (id == "SetMiterLimit") {
            ctx.miterLimit = packet.miter_limit;
            genericSuccess(txn_uuid);
        } else if (id == "GetLineDash") {
            const line_dash = ctx.getLineDash();
            send(txn_uuid, 'LineDash', { line_dash });
        } else if (id == "SetLineDash") {
            ctx.setLineDash(packet.line_dash);
            const line_dash = ctx.getLineDash();
            send(txn_uuid, 'LineDash', { line_dash });
        } else if (id == "SetLineDashOffset") {
            ctx.lineDashOffset = packet.line_dash_offset;
            genericSuccess(txn_uuid);
        } else if (id == "SetFont") {
            ctx.font = packet.font;
            genericSuccess(txn_uuid);
        } else if (id == "SetTextAlign") {
            ctx.textAlign = packet.text_align.toLowerCase();
            genericSuccess(txn_uuid);
        } else if (id == "SetTextBaseline") {
            ctx.textBaseline = packet.text_baseline.toLowerCase();
            genericSuccess(txn_uuid);
        } else if (id == "SetTextDirection") {
            ctx.direction = packet.text_direction.toLowerCase();
            genericSuccess(txn_uuid);
        } else if (id == "SetFillStyle") {
            ctx.fillStyle = packet.fill_style;
            genericSuccess(txn_uuid);
        } else if (id == "SetStrokeStyle") {
            ctx.strokeStyle = packet.stroke_style;
            genericSuccess(txn_uuid);
        } else if (id == "SetShadowBlur") {
            ctx.shadowBlur = packet.shadow_blur;
            genericSuccess(txn_uuid);
        } else if (id == "SetShadowColor") {
            ctx.shadowColor = packet.shadow_color;
            genericSuccess(txn_uuid);
        } else if (id == "SetShadowOffsetX") {
            ctx.shadowOffsetX = packet.shadow_offset_x;
            genericSuccess(txn_uuid);
        } else if (id == "SetShadowOffsetY") {
            ctx.shadowOffsetY = packet.shadow_offset_y;
            genericSuccess(txn_uuid);
        } else if (id == "SetGlobalAlpha") {
            ctx.globalAlpha = packet.global_alpha;
            genericSuccess(txn_uuid);
        } else if (id == "SetGlobalCompositeOperation") {
            ctx.globalCompositeOperation = packet.global_composite_operation;
            genericSuccess(txn_uuid);
        } else if (id == "SetImageSmoothingEnabled") {
            ctx.imageSmoothingEnabled = packet.image_smoothing_enabled;
            genericSuccess(txn_uuid);
        } else if (id == "SetImageSmoothingQuality") {
            ctx.imageSmoothingQuality = packet.image_smoothing_quality ? packet.image_smoothing_quality.toLowerCase() : null;
            genericSuccess(txn_uuid);
        } else if (id == "SetFilter") {
            ctx.filter = packet.filter ? packet.filter : null;
            genericSuccess(txn_uuid);
        } else if (id == "SetWidth") {
            canvas.width = packet.width;
            genericSuccess(txn_uuid);
        } else if (id == "SetHeight") {
            canvas.height = packet.height;
            genericSuccess(txn_uuid);
        } else if (id == "ClearRect") {
            ctx.clearRect(packet.x, packet.y, packet.width, packet.height);
            genericSuccess(txn_uuid);
        } else if (id == "FillRect") {
            ctx.fillRect(packet.x, packet.y, packet.width, packet.height);
            genericSuccess(txn_uuid);
        } else if (id == "StrokeRect") {
            ctx.strokeRect(packet.x, packet.y, packet.width, packet.height);
            genericSuccess(txn_uuid);
        } else if (id == "FillText") {
            ctx.fillText(packet.text, packet.x, packet.y, ...(packet.max_width == null ? [] : [packet.max_width]));
            genericSuccess(txn_uuid);
        } else if (id == "StrokeText") {
            ctx.strokeText(packet.text, packet.x, packet.y, ...(packet.max_width == null ? [] : [packet.max_width]));
            genericSuccess(txn_uuid);
        } else if (id == "CreateLinearGradient") {
            const gradient = ctx.createLinearGradient(packet.x0, packet.y0, packet.x1, packet.y1);
            for ([offset, color] of packet.stops) {
                gradient.addColorStop(offset, color);
            }
            genericSuccess(txn_uuid);
        } else if (id == "CreateRadialGradient") {
            const gradient = ctx.createRadialGradient(packet.x0, packet.y0, packet.r0, packet.x1, packet.y1, packet.r1);
            for ([offset, color] of packet.stops) {
                gradient.addColorStop(offset, color);
            }
            genericSuccess(txn_uuid);
        } else if (id == "CreatePattern") {
            const img = document.createElement('img');
            img.src = packet.url;
            let repitition = null;
            if (packet.repitition == 'Repeat') {
                repitition = 'repeat';
            } else if (packet.repitition == 'RepeatX') {
                repitition = 'repeat-x';
            } else if (packet.repitition == 'RepeatY') {
                repitition = 'repeat-y';
            } else if (packet.repitition == 'NoRepeat') {
                repitition = 'no-repeat';
            } else {
                throw new Error(`invalid repitition: ${packet.repitition}`);
            }
            ctx.createPattern(img, repitition);
            genericSuccess(txn_uuid);
        } else if (id == "BeginPath") {
            let target;
            if (packet.path_uuid != null) {
                target = canvi.paths[packet.path_uuid] = new Path2D();
            } else {
                target = ctx;
            }
            target.beginPath();
            genericSuccess(txn_uuid);
        } else if (id == "ClosePath") {
            const target = getTarget(packet.path_uuid, canvi);
            target.closePath();
            genericSuccess(txn_uuid);
        } else if (id == "MoveTo") {
            const target = getTarget(packet.path_uuid, canvi);
            target.moveTo(packet.x, packet.y);
            genericSuccess(txn_uuid);
        } else if (id == "LineTo") {
            const target = getTarget(packet.path_uuid, canvi);
            target.lineTo(packet.x, packet.y);
            genericSuccess(txn_uuid);
        } else if (id == "BezierCurveTo") {
            const target = getTarget(packet.path_uuid, canvi);
            target.bezierCurveTo(packet.cp1x, packet.cp1y, packet.cp2x, packet.cp2y, packet.x, packet.y);
            genericSuccess(txn_uuid);
        } else if (id == "QuadraticCurveTo") {
            const target = getTarget(packet.path_uuid, canvi);
            target.quadraticCurveTo(packet.cpx, packet.cpy, packet.x, packet.y);
            genericSuccess(txn_uuid);
        } else if (id == "Arc") {
            const target = getTarget(packet.path_uuid, canvi);
            target.arc(packet.x, packet.y, packet.radius, packet.start_angle, packet.end_angle, packet.anticlockwise);
            genericSuccess(txn_uuid);
        } else if (id == "ArcTo") {
            const target = getTarget(packet.path_uuid, canvi);
            target.arcTo(packet.x1, packet.y1, packet.x2, packet.y2, packet.radius);
            genericSuccess(txn_uuid);
        } else if (id == "Ellipse") {
            const target = getTarget(packet.path_uuid, canvi);
            target.ellipse(packet.x, packet.y, packet.radius_x, packet.radius_y, packet.rotation, packet.start_angle, packet.end_angle, packet.anticlockwise);
            genericSuccess(txn_uuid);
        } else if (id == "Rect") {
            const target = getTarget(packet.path_uuid, canvi);
            target.ellipse(packet.x, packet.y, packet.width, packet.height);
            genericSuccess(txn_uuid);
        } else if (id == "Fill") {
            const target = getTargetImplied(packet.path_uuid, canvi);
            if (target) {
                ctx.fill(target, packet.is_even_odd ? "evenodd" : "nonzero");
            } else {
                ctx.fill(packet.is_even_odd ? "evenodd" : "nonzero");
            }
            genericSuccess(txn_uuid);
        } else if (id == "Stroke") {
            const target = getTargetImplied(packet.path_uuid, canvi);
            if (target) {
                ctx.stroke(target);
            } else {
                ctx.stroke();
            }
            genericSuccess(txn_uuid);
        } else if (id == "Clip") {
            const target = getTargetImplied(packet.path_uuid, canvi);
            if (target) {
                ctx.clip(target, packet.is_even_odd ? "evenodd" : "nonzero");
            } else {
                ctx.clip(packet.is_even_odd ? "evenodd" : "nonzero");
            }
            genericSuccess(txn_uuid);
        } else if (id == "IsPointInPath") {
            const target = getTargetImplied(packet.path_uuid, canvi);
            let is_in = false;
            if (target) {
                is_in = ctx.isPointInPath(target, packet.x, packet.y, packet.is_even_odd ? "evenodd" : "nonzero");
            } else {
                is_in = ctx.isPointInPath(packet.x, packet.y, packet.is_even_odd ? "evenodd" : "nonzero");
            }
            send(txn_uuid, 'IsPointIn', { is_in });
        } else if (id == "IsPointInStroke") {
            const target = getTargetImplied(packet.path_uuid, canvi);
            let is_in = false;
            if (target) {
                is_in = ctx.isPointInPath(target, packet.x, packet.y);
            } else {
                is_in = ctx.isPointInPath(packet.x, packet.y);
            }
            send(txn_uuid, 'IsPointIn', { is_in });
        } else if (id == "GetTransform") {
            const [a, b, c, d, e, f] = ctx.getTransform();
            send(txn_uuid, 'Transform', { matrix: [a, b, c, d, e, f] });
        } else if (id == "Rotate") {
            ctx.rotate(packet.angle);
            genericSuccess(txn_uuid);
        } else if (id == "Scale") {
            ctx.scale(packet.x, packet.y);
            genericSuccess(txn_uuid);
        } else if (id == "Translate") {
            ctx.translate(packet.x, packet.y);
            genericSuccess(txn_uuid);
        } else if (id == "Transform") {
            ctx.transform.apply(ctx, packet.matrix);
            genericSuccess(txn_uuid);
        } else if (id == "SetTransform") {
            ctx.setTransform.apply(ctx, packet.matrix);
            genericSuccess(txn_uuid);
        } else if (id == "DrawImage") {
            const img = document.createElement('img');
            img.src = packet.url;
            const { source, d_dims } = packet;
            const args = [img];
            if (source != null) {
                args.push(...source);
            }
            args.push(packet.dx, packet.dy);
            if (d_dims != null) {
                args.push(...d_dims);
            } else if (source != null) {
                args.push(void 0, void 0);
            }
            ctx.drawImage(...args);
            genericSuccess(txn_uuid);
        } else if (id == "GetImageData") {
            const data = ctx.getImageData(packet.sx, packet.sy, packet.sw, packet.sh);
            const image_data = {
                data: new Uint32Array(data.data.buffer),
                width: data.width,
                height: data.height,
            };
            send(txn_uuid, 'ImageData', { image_data: image_data });
        } else if (id == "SetImageData") {
            const image_data = new ImageData(new Uint8ClampedArray(new Uint32Array(packet.image_data.data).buffer), packet.image_data.width, packet.image_data.height);
            const args = [image_data, packet.dx, packet.dy];
            if (packet.dirty_pos != null) {
                args.push(...packet.dirty_pos);
            } else if (packet.dirty_dims != null) {
                args.push(0, 0);
            }
            if (packet.dirty_dims != null) {
                args.push(...packet.dirty_dims);
            } else if (packet.dirty_pos != null) {
                args.push(image_data.width, image_data.height);
            }
            ctx.putImageData(...args);
            genericSuccess(txn_uuid);
        } else if (id == "Save") {
            ctx.save();
            genericSuccess(txn_uuid);
        } else if (id == "Restore") {
            ctx.restore();
            genericSuccess(txn_uuid);
        } else if (id == "ToDataUrl") {
            send(txn_uuid, 'DataUrl', { data_url: canvas.toDataURL(data.mime_type, packet.quality == null ? void 0 : packet.quality) });
        } else {
            throw new Error(`invalid packet id: ${id}`);
        }
    } catch (e) {
        console.error(e);
        sendError(txn_uuid, e);
    }
}

function reconnect() {
    if (websocket != null) {
        websocket.close();
    }
    websocket = new WebSocket(`ws://${websocketAddress}`);
    websocket.onopen = connectionOpened;
    websocket.onclose = () => {
        console.log('connection closed, reconnecting...');
        setTimeout(reconnect, 1000);
    };
    websocket.onmessage = receiveMessage;
};

reconnect();
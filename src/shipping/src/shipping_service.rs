// Copyright The OpenTelemetry Authors
// SPDX-License-Identifier: Apache-2.0

use actix_web::{post, web, HttpResponse, Responder};
use opentelemetry::{trace::FutureExt, TraceId};
use tracing::info;

mod quote;
use quote::create_quote_from_count;

mod tracking;
use tracking::create_tracking_id;

mod shipping_types;
pub use shipping_types::*;

const NANOS_MULTIPLE: u32 = 10000000u32;

#[post("/get-quote")]
pub async fn get_quote(req: web::Json<GetQuoteRequest>) -> impl Responder {
    let itemct: u32 = req.items.iter().map(|item| item.quantity as u32).sum();

    let quote = match create_quote_from_count(itemct).await {
        Ok(q) => q,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Failed to get quote: {}", e));
        }
    };

    let reply = GetQuoteResponse {
        cost_usd: Some(Money {
            currency_code: "USD".into(),
            units: quote.dollars,
            nanos: quote.cents * NANOS_MULTIPLE,
        }),
    };

    info!(
        name = "SendingQuoteValue",
        quote.dollars = quote.dollars,
        quote.cents = quote.cents,
        message = "Sending Quote"
    );

    HttpResponse::Ok().json(reply)
}

///  Fetch an opentelemetry::trace::TraceId as hex through the full tracing stack
// pub fn get_trace_id() -> TraceId {
//     use opentelemetry::trace::TraceContextExt as _; // opentelemetry::Context -> opentelemetry::trace::Span
//     use tracing_opentelemetry::OpenTelemetrySpanExt as _; // tracing::Span to opentelemetry::Context
//     tracing::Span::current()
//         .context()
//         .span()
//         .span_context()
//         .trace_id()
// }

#[post("/ship-order")]
pub async fn ship_order(
    req: actix_web::HttpRequest,
    payload: web::Json<ShipOrderRequest>,
) -> impl Responder {
    // 创建一个新的跟踪上下文
    // let span = tracing::info_span!("create_tracking_id");
    // let _guard = span.enter();

    // 从请求头中提取跟踪上下文
    // let parent_cx = opentelemetry::global::get_text_map_propagator(|propagator| {
    //     propagator.extract(&HeaderExtractor(req.headers()))
    // });

    // 创建一个子 span
    // let span = tracing::info_span!("ship_order").with_context(parent_cx);
    // let _guard = span.enter();

    let tid = create_tracking_id();
    info!(
        name = "CreatingTrackingId",
        tracking_id = tid.as_str(),
        message = "Tracking ID Created"
    );

    // let trace_id = get_trace_id();
    // println!("trace_id: {trace_id}");

    HttpResponse::Ok().json(ShipOrderResponse { tracking_id: tid })
}

// 辅助提取器
struct HeaderExtractor<'a>(&'a actix_web::http::header::HeaderMap);

impl<'a> opentelemetry::propagation::Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_ship_order() {
        let app = test::init_service(App::new().service(ship_order)).await;
        let req = test::TestRequest::post()
            .uri("/ship-order")
            .insert_header(ContentType::json())
            .set_json(&ShipOrderRequest {})
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let order: ShipOrderResponse = test::read_body_json(resp).await;
        assert!(!order.tracking_id.is_empty());
    }
}

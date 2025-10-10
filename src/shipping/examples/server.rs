use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use opentelemetry::{global, KeyValue};
use opentelemetry_instrumentation_actix_web::{RequestMetrics, RequestTracing};
use opentelemetry_sdk::{
    metrics::{Aggregation, Instrument, SdkMeterProvider, Stream},
    propagation::TraceContextPropagator,
    trace::SdkTracerProvider,
    Resource,
};
use opentelemetry_stdout::{MetricExporter, SpanExporter};
use serde::{Deserialize, Serialize};
use tracing::info;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start a new OTLP trace pipeline
    global::set_text_map_propagator(TraceContextPropagator::new());

    let service_name_resource = Resource::builder_empty()
        .with_attribute(KeyValue::new("service.name", "actix_server"))
        .build();

    let tracer = SdkTracerProvider::builder()
        .with_simple_exporter(SpanExporter::default())
        .with_resource(service_name_resource)
        .build();

    global::set_tracer_provider(tracer.clone());

    // Setup a OTLP metrics exporter if --features metrics is used
    #[cfg(feature = "metrics")]
    let meter_provider = {
        let provider = SdkMeterProvider::builder()
            .with_periodic_exporter(MetricExporter::default())
            .with_resource(
                Resource::builder_empty()
                    .with_attribute(KeyValue::new("service.name", "my_app"))
                    .build(),
            )
            .with_view(|i: &Instrument| {
                if i.name() == "http.server.duration" {
                    Some(
                        Stream::builder()
                            .with_aggregation(Aggregation::ExplicitBucketHistogram {
                                boundaries: vec![
                                    0.0, 0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75,
                                    1.0, 2.5, 5.0, 7.5, 10.0,
                                ],
                                record_min_max: true,
                            })
                            .build()
                            .unwrap(),
                    )
                } else {
                    None
                }
            })
            .build();
        global::set_meter_provider(provider.clone());

        provider
    };

    HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .wrap(RequestMetrics::default())
            .service(ship_order)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8813")?
    .run()
    .await?;

    println!("http server ended");

    // Ensure all spans have been reported
    tracer.shutdown()?;

    #[cfg(feature = "metrics")]
    meter_provider.shutdown()?;

    Ok(())
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct ShipOrderRequest {}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShipOrderResponse {
    pub tracking_id: String,
}

use uuid::Uuid;

/// returns a tracking ID
pub fn create_tracking_id() -> String {
    Uuid::new_v4().to_string()
}

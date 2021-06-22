use crate::actors::supervisor::Supervisor;
use anyhow::Error;
use async_trait::async_trait;
use meio::{Actor, Consumer, Context, InterruptedBy, StartedBy};
use rill_protocol::flow::core::{ActionEnvelope, Activity};
use rillrate_logs_protocol::plain_logs::tracer::{
    PlainLogsState, PlainLogsTracer, PlainLogsWatcher,
};
use tokio_stream::wrappers::{errors::BroadcastStreamRecvError, BroadcastStream};

pub struct Forwarder {
    tracer: PlainLogsTracer,
    watcher: Option<PlainLogsWatcher>,
}

impl Forwarder {
    pub fn new() -> Self {
        // TODO: Use a parameter here
        let path = "my.logs".parse().unwrap();
        let (tracer, watcher) = PlainLogsTracer::new(path);
        Self {
            tracer,
            watcher: Some(watcher),
        }
    }
}

impl Actor for Forwarder {
    type GroupBy = ();
}

#[async_trait]
impl StartedBy<Supervisor> for Forwarder {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        let rx = self
            .watcher
            .take()
            .ok_or_else(|| Error::msg("watcher already taken"))?;
        let stream = BroadcastStream::new(rx);
        ctx.attach(stream, (), ());

        Ok(())
    }
}

#[async_trait]
impl InterruptedBy<Supervisor> for Forwarder {
    async fn handle(&mut self, ctx: &mut Context<Self>) -> Result<(), Error> {
        ctx.shutdown();
        Ok(())
    }
}

#[async_trait]
impl Consumer<Result<ActionEnvelope<PlainLogsState>, BroadcastStreamRecvError>> for Forwarder {
    async fn handle(
        &mut self,
        event: Result<ActionEnvelope<PlainLogsState>, BroadcastStreamRecvError>,
        _ctx: &mut Context<Self>,
    ) -> Result<(), Error> {
        let envelope = event?;
        match envelope.activity {
            Activity::Action(action) => {}
            _ => {}
        }
        Ok(())
    }
}
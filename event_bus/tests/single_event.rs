use event_bus::{async_trait, Event, Subscribe};

use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Event)]
struct CountEvent;

struct CountUpV1;

#[async_trait]
impl Subscribe for CountUpV1 {
    type InputEvent = CountEvent;
    type Output = u32;
    async fn handle_event<'event>(&self, _: &'event Self::InputEvent) -> Self::Output {
        COUNTER.fetch_add(1, Ordering::SeqCst);
        1
    }
}

async fn count_up_v2(_: &CountEvent) -> u32 {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    2
}

#[tokio::test]
async fn test_count() {
    use event_bus::EventBus;
    use futures::future::join_all;

    let mut bus = EventBus::<u32>::new();

    bus.subscribe(CountUpV1);
    bus.subscribe_pinned_fn(|event| Box::pin(count_up_v2(event)));
    bus.subscribe_pinned_fn(|_: &CountEvent| {
        Box::pin(async move {
            COUNTER.fetch_add(1, Ordering::SeqCst);
            3
        })
    });

    let tasks = bus.dispatch_event(CountEvent);

    let returns = join_all(tasks).await;
    assert_eq!(vec![1, 2, 3], returns);

    assert_eq!(3, COUNTER.load(Ordering::SeqCst));
}

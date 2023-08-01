use event_bus::async_trait;
use event_bus::{Event, Subscribe};

/// 気象情報を表すイベント
#[derive(Clone, Event)]
struct Weather {
    /// 気温(セ氏)
    temperature: f64,
    /// 気圧
    pressure: f64,
}

#[derive(Debug)]
enum WeatherError {}

struct JpShowWeather;

#[async_trait]
impl Subscribe for JpShowWeather {
    type InputEvent = Weather;
    type Output = Result<(), WeatherError>;

    async fn handle_event<'event>(&self, event: &'event Weather) -> Self::Output {
        println!(
            "気温は{:>3.0}度で，気圧は{:>4.0}Hpaです．",
            event.temperature, event.pressure
        );

        Ok(())
    }
}

async fn usa_show_weather(event: &Weather) -> Result<(), WeatherError> {
    println!(
        "Temperature: {:>3.0}℉, pressure: {:>3.0}Hpa",
        event.temperature * 1.8 + 32.0,
        event.pressure
    );

    Ok(())
}

fn main() {
    use event_bus::{event_bus_from_subscriber_pinned_fns, event_bus_from_subscribes, EventBus};
    use std::thread::sleep;
    use std::time::Duration;

    let mut event_bus = EventBus::<Result<(), WeatherError>>::new();

    event_bus.subscribe(JpShowWeather);
    event_bus.subscribe_pinned_fn(|event| Box::pin(usa_show_weather(event)));

    let tasks = event_bus.dispatch_event(Weather {
        temperature: 25.0,
        pressure: 1014.0,
    });

    while !tasks.iter().all(|task| task.is_finished()) {
        sleep(Duration::from_millis(100));
    }
    println!("All event handler finished");

    // その他の作成方法
    let _: EventBus<Result<(), WeatherError>> =
        event_bus_from_subscribes![JpShowWeather, JpShowWeather];

    let _: EventBus<Result<(), WeatherError>> =
        event_bus_from_subscriber_pinned_fns![|event| Box::pin(usa_show_weather(event)), |event| {
            Box::pin(usa_show_weather(event))
        }];
}

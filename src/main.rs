#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt::info;
use defmt_rtt as _;
use device_envoy_rp::{
    Result, led,
    led::{Led as _, LedLevel, OnLevel},
};
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use panic_probe as _;

const DOT_MS: u64 = 200;
const DASH_MS: u64 = DOT_MS * 3;
const SYMBOL_GAP_MS: u64 = DOT_MS;
const LETTER_GAP_MS: u64 = DOT_MS * 3;
const WORD_GAP_MS: u64 = DOT_MS * 7;

const DOT_DURATION: Duration = Duration::from_millis(DOT_MS);
const DASH_DURATION: Duration = Duration::from_millis(DASH_MS);
const SYMBOL_GAP_DURATION: Duration = Duration::from_millis(SYMBOL_GAP_MS);
const LETTER_GAP_DURATION: Duration = Duration::from_millis(LETTER_GAP_MS);
const WORD_GAP_DURATION: Duration = Duration::from_millis(WORD_GAP_MS);

const SOS_PATTERN: [(LedLevel, Duration); 18] = [
    (LedLevel::On, DOT_DURATION),
    (LedLevel::Off, SYMBOL_GAP_DURATION),
    (LedLevel::On, DOT_DURATION),
    (LedLevel::Off, SYMBOL_GAP_DURATION),
    (LedLevel::On, DOT_DURATION),
    (LedLevel::Off, LETTER_GAP_DURATION),
    (LedLevel::On, DASH_DURATION),
    (LedLevel::Off, SYMBOL_GAP_DURATION),
    (LedLevel::On, DASH_DURATION),
    (LedLevel::Off, SYMBOL_GAP_DURATION),
    (LedLevel::On, DASH_DURATION),
    (LedLevel::Off, LETTER_GAP_DURATION),
    (LedLevel::On, DOT_DURATION),
    (LedLevel::Off, SYMBOL_GAP_DURATION),
    (LedLevel::On, DOT_DURATION),
    (LedLevel::Off, SYMBOL_GAP_DURATION),
    (LedLevel::On, DOT_DURATION),
    (LedLevel::Off, WORD_GAP_DURATION),
];

#[cfg(feature = "wifi")]
led!(LedBlinky {
    pin: PIN_1,
    max_steps: 32
});

#[cfg(not(feature = "wifi"))]
led!(LedBlinky {
    pin: PIN_25,
    max_steps: 32
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let err = inner_main(spawner).await.unwrap_err();
    core::panic!("{err}");
}

async fn inner_main(spawner: Spawner) -> Result<Infallible> {
    info!("device-envoy-rp-blinky boot");
    let p = embassy_rp::init(Default::default());

    #[cfg(feature = "wifi")]
    let led_blinky = LedBlinky::new(p.PIN_1, OnLevel::High, spawner)?;
    #[cfg(not(feature = "wifi"))]
    let led_blinky = LedBlinky::new(p.PIN_25, OnLevel::High, spawner)?;

    #[cfg(feature = "wifi")]
    info!("LED ready on PIN_1");
    #[cfg(not(feature = "wifi"))]
    info!("LED ready on internal PIN_25");

    led_blinky.animate(SOS_PATTERN);
    info!("SOS animation started");

    // Delayed log helps confirm RTT output even if very-early boot logs are missed.
    Timer::after_secs(2).await;
    info!("device-envoy-rp-blinky alive");

    core::future::pending().await
}

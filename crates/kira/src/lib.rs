/*!
# Kira

Kira is a backend-agnostic library to create expressive audio for games. It provides
[tweens](tween) for smoothly adjusting properties of sounds, a flexible [mixer](track)
for applying effects to audio, a [clock] system for precisely timing audio events,
and [spatial audio](spatial) support.

To get started, create an [`AudioManager`](crate::manager::AudioManager) and use it to
[play](crate::manager::AudioManager::play) a
[`StaticSoundData`](crate::sound::static_sound::StaticSoundData) or
[`StreamingSoundData`](crate::sound::streaming::StreamingSoundData).

## Examples

Playing a sound multiple times simultaneously:

```rust, no_run
# extern crate kira;
#
use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

// Create an audio manager. This plays sounds and manages resources.
let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
let sound_data = StaticSoundData::from_file("sound.ogg", StaticSoundSettings::default())?;
manager.play(sound_data.clone())?;
// After a couple seconds...
manager.play(sound_data.clone())?;
// Cloning the sound data will not use any extra memory.
# Result::<(), Box<dyn std::error::Error>>::Ok(())
```

Gradually speeding up a sound over time:

```rust, no_run
# extern crate kira;
#
use std::time::Duration;

use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	tween::Tween,
};

let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
let sound_data = StaticSoundData::from_file("sound.ogg", StaticSoundSettings::new())?;
let mut sound = manager.play(sound_data)?;
// Start smoothly adjusting the playback rate parameter.
sound.set_playback_rate(
	2.0,
	Tween {
		duration: Duration::from_secs(3),
		..Default::default()
	},
);
# Result::<(), Box<dyn std::error::Error>>::Ok(())
```

Playing a sound with a low-pass filter applied (this makes the
audio sound muffled):

```rust, no_run
# extern crate kira;
#
use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	track::{
		TrackBuilder,
		effect::filter::FilterBuilder,
	},
};

let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
// Create a mixer sub-track with a filter.
let track = manager.add_sub_track({
	let mut builder = TrackBuilder::new();
	builder.add_effect(FilterBuilder::new().cutoff(1000.0));
	builder
})?;
// Play the sound on the track.
let sound_data = StaticSoundData::from_file(
	"sound.ogg",
	StaticSoundSettings::new().output_destination(&track),
)?;
manager.play(sound_data)?;
# Result::<(), Box<dyn std::error::Error>>::Ok(())
```

Playing sounds in time with a musical beat:

```rust, no_run
# extern crate kira;
#
use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	clock::ClockSpeed,
};

const TEMPO: f64 = 120.0;

let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
// Create a clock that ticks 120 times per second. In this case,
// each tick is one musical beat. We can use a tick to represent any
// arbitrary amount of time.
let mut clock = manager.add_clock(ClockSpeed::TicksPerMinute(TEMPO))?;
// Play a sound 2 ticks (beats) from now.
let sound_data_1 = StaticSoundData::from_file(
	"sound1.ogg",
	StaticSoundSettings::new().start_time(clock.time() + 2),
)?;
manager.play(sound_data_1)?;
// Play a different sound 4 ticks (beats) from now.
let sound_data_2 = StaticSoundData::from_file(
	"sound2.ogg",
	StaticSoundSettings::new().start_time(clock.time() + 4),
)?;
manager.play(sound_data_2)?;
// Start the clock.
clock.start()?;
# Result::<(), Box<dyn std::error::Error>>::Ok(())
```

## Features

The Kira crate has the following feature flags, all of which are enabled by
default:

- `cpal` - enables the cpal backend and makes it the default for audio managers.
This allows Kira to talk to the operating system to output audio. Most users should
leave this enabled.
- `symphonia` - allows loading and streaming audio from common audio formats,
like MP3 and WAV.
	- `mp3` - enables support for loading and streaming MP3 audio (enables the
	`symphonia` feature automatically)
	- `ogg` - enables support for loading and streaming OGG audio (enables the
	`symphonia` feature automatically)
	- `flac` - enables support for loading and streaming FLAC audio (enables the
	`symphonia` feature automatically)
	- `wav` - enables support for loading and streaming WAV audio (enables the
	`symphonia` feature automatically)

## Loading other audio file formats

Kira will be able to load any audio format that Symphonia supports with its
current enabled features. For example, to add support for AAC files, you can
add `symphonia` to your Cargo.toml with the `aac` feature:

```toml
symphonia = { version = "0.5.2", features = ["aac"] }
```

Kira's `mp3`, `ogg`, `flac`, and `wav` feature flags are provided for convenience.

See the [symphonia documentation](https://github.com/pdeljanov/Symphonia#formats-demuxers)
for a list of supported container formats and codecs.

## Performance using the `dev` profile

By default, Rust programs run with the `dev` profile are not optimized. This can
lead to poor performance of audio playback and long loading times for audio
files. You can alleviate this by building Kira and its audio-related
dependencies with a higher optimization level. Add the following to your
Cargo.toml:

```toml
[profile.dev.package.kira]
opt-level = 3

[profile.dev.package.cpal]
opt-level = 3

[profile.dev.package.symphonia]
opt-level = 3

[profile.dev.package.symphonia-bundle-mp3]
opt-level = 3

[profile.dev.package.symphonia-format-ogg]
opt-level = 3

[profile.dev.package.symphonia-codec-vorbis]
opt-level = 3

[profile.dev.package.symphonia-bundle-flac]
opt-level = 3

[profile.dev.package.symphonia-format-wav]
opt-level = 3

[profile.dev.package.symphonia-codec-pcm]
opt-level = 3
```

You can also build all of your projects with a higher optimization level by
using this snippet instead:

```toml
[profile.dev.package."*"]
opt-level = 3
```

Building dependencies with a higher optimization level does increase compile
times, but only when compiling your project from scratch. If you only make
changes to your crate, you're not recompiling the dependencies, so you don't
suffer from the longer compilation step in that case. Building dependencies
optimized and the main crate unoptimized can be a good balance of performance
and compile times for games.
*/

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_else_if)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::enum_variant_names)]
#![warn(clippy::todo)]
#![warn(missing_docs)]
#![allow(clippy::tabs_in_doc_comments)]

pub mod clock;
pub mod dsp;
mod error;
pub mod manager;
pub mod modulator;
mod output_destination;
pub mod sound;
pub mod spatial;
mod start_time;
pub mod track;
pub mod tween;
mod volume;

pub use error::*;
pub use output_destination::*;
pub use start_time::*;
pub use volume::*;

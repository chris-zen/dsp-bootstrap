use std::sync::{Arc, RwLock};

use failure;
use failure::{Error, Fail};
use failure_derive;

use portaudio;

mod audio;
use crate::audio::{audio_close, audio_start};

mod dsp;

fn main() -> Result<(), Error> {
  let pa_ctx = portaudio::PortAudio::new()?;

  let mut stream = audio_start(&pa_ctx)?;

  // Loop while the non-blocking stream is active.
  while let Ok(true) = stream.is_active() {
    pa_ctx.sleep(1000);
  }

  audio_close(&mut stream)?;

  Ok(())
}

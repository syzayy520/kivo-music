use crate::playback::audio_bridge::PcmSourceChunk;
use crate::playback::decoder::DecodedAudioFrame;
use crate::playback::output::AudioOutputFrame;

pub enum AudioRouteFrameInput<'a> {
    PcmSourceChunk(PcmSourceChunk<'a>),
    DecodedFrame(&'a DecodedAudioFrame),
    OutputFrame(&'a AudioOutputFrame),
}

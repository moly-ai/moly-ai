/// Build WAV audio data from f32 samples.
///
/// Returns a Vec<u8> containing the complete WAV file data.
pub(crate) fn build_wav(samples: &[f32], sample_rate: u32, channels: u16) -> Vec<u8> {
    let header_len = 44;
    let data_len = samples.len() * 2; // 2 bytes per sample
    let total_len = header_len + data_len;

    let mut wav_bytes = Vec::with_capacity(total_len);

    // RIFF header
    wav_bytes.extend_from_slice(b"RIFF");
    wav_bytes.extend_from_slice(&((total_len as u32 - 8).to_le_bytes()));
    wav_bytes.extend_from_slice(b"WAVE");

    // fmt chunk
    wav_bytes.extend_from_slice(b"fmt ");
    wav_bytes.extend_from_slice(&(16u32.to_le_bytes())); // chunk size
    wav_bytes.extend_from_slice(&(1u16.to_le_bytes())); // PCM format
    wav_bytes.extend_from_slice(&(channels).to_le_bytes());
    wav_bytes.extend_from_slice(&(sample_rate).to_le_bytes());
    wav_bytes.extend_from_slice(&((sample_rate * (channels as u32) * 2) as u32).to_le_bytes()); // byte rate
    wav_bytes.extend_from_slice(&((channels * 2) as u16).to_le_bytes()); // block align
    wav_bytes.extend_from_slice(&(16u16.to_le_bytes())); // bits per sample

    // data chunk
    wav_bytes.extend_from_slice(b"data");
    wav_bytes.extend_from_slice(&(data_len as u32).to_le_bytes());

    for sample in samples {
        let clamped = sample.max(-1.0).min(1.0);
        let val = (clamped * 32767.0) as i16;
        wav_bytes.extend_from_slice(&val.to_le_bytes());
    }

    wav_bytes
}

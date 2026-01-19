/// Errors that can occur when generating WAV data.
#[derive(Debug)]
pub enum WavError {
    /// The generated data exceeds the size limit (4GB).
    SizeLimitExceeded,
}

impl std::fmt::Display for WavError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WavError::SizeLimitExceeded => write!(f, "WAV file size exceeds 4GB limit"),
        }
    }
}

impl std::error::Error for WavError {}

/// Build WAV audio data from f32 samples.
///
/// Returns a `Vec<u8>` containing the complete WAV file data.
///
/// # Errors
///
/// Returns [`WavError::SizeLimitExceeded`] if the generated WAV file would exceed
/// the 4GB size limit imposed by the format.
pub(crate) fn build_wav(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
) -> Result<Vec<u8>, WavError> {
    let header_len = 44;
    let data_len = samples.len() * 2; // 2 bytes per sample
    let total_len = header_len + data_len;

    if total_len > u32::MAX as usize {
        return Err(WavError::SizeLimitExceeded);
    }

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
        // Note: AI suggested round(), but trunc() works better for me.
        let val = (sample.clamp(-1.0, 1.0) * 32767.0).trunc() as i16;
        wav_bytes.extend_from_slice(&val.to_le_bytes());
    }

    Ok(wav_bytes)
}

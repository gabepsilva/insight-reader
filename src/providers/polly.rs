//! AWS Polly TTS provider implementation.
//!
//! Uses the AWS SDK for Rust to synthesize speech and plays it using rodio.

use aws_config::BehaviorVersion;
use aws_sdk_polly::types::{Engine, OutputFormat, VoiceId};
use tracing::{debug, info};

use super::audio_player::AudioPlayer;
use super::{TTSError, TTSProvider};
use crate::voices::aws;

const CREDENTIALS_ERROR_MSG: &str = "AWS credentials not found. Please configure credentials via:\n  - Environment variables: AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY\n  - Or credentials file: ~/.aws/credentials";

/// AWS Polly TTS provider using the official AWS SDK.
pub struct PollyTTSProvider {
    /// AWS Polly client
    client: aws_sdk_polly::Client,
    /// Shared audio playback engine
    player: AudioPlayer,
    /// Tokio runtime for async AWS calls
    runtime: tokio::runtime::Runtime,
    /// Selected voice ID (e.g., "Matthew", "Joanna")
    voice_id: String,
    /// Selected engine type (e.g., "Standard", "Neural", "Generative", "LongForm")
    engine: Engine,
}

impl PollyTTSProvider {
    /// Create a new AWS Polly TTS provider.
    ///
    /// Loads credentials from `~/.aws/credentials` or environment variables.
    pub fn new(voice_id: Option<String>) -> Result<Self, TTSError> {
        info!("Initializing AWS Polly TTS provider");

        // Create a tokio runtime for async AWS SDK calls
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| TTSError::ProcessError(format!("Failed to create tokio runtime: {e}")))?;

        // Determine region: check ~/.aws/config, env vars, or default to us-east-1
        let region = aws::detect_aws_region();
        debug!(region = %region, "Using AWS region");

        // Load AWS config (credentials from ~/.aws/credentials or env vars)
        let config = runtime.block_on(async {
            aws_config::defaults(BehaviorVersion::latest())
                .region(aws_config::Region::new(region.clone()))
                .load()
                .await
        });

        let client = aws_sdk_polly::Client::new(&config);
        debug!("AWS Polly client created");

        // Parse voice_id and engine from the voice key (format: "VoiceId:Engine" or just "VoiceId")
        let (voice_id_str, engine) = if let Some(voice_key) = voice_id {
            if let Some((vid, eng_str)) = voice_key.split_once(':') {
                let engine = match eng_str {
                    "Standard" => Engine::Standard,
                    "Neural" => Engine::Neural,
                    "Generative" => Engine::Generative,
                    "LongForm" => Engine::LongForm,
                    _ => {
                        debug!(engine = %eng_str, "Unknown engine type, defaulting to Neural");
                        Engine::Neural
                    }
                };
                (vid.to_string(), engine)
            } else {
                // No engine specified, default to Neural
                (voice_key, Engine::Neural)
            }
        } else {
            ("Matthew".to_string(), Engine::Neural)
        };

        debug!(voice_id = %voice_id_str, engine = ?engine, "Using voice and engine");

        // Polly neural voices use 16kHz sample rate
        let player = AudioPlayer::new(16000)?;

        Ok(Self {
            client,
            player,
            runtime,
            voice_id: voice_id_str,
            engine,
        })
    }


    /// Check if AWS credentials are available.
    ///
    /// Returns `Ok(())` if credentials are found, or an error message if not.
    pub fn check_credentials() -> Result<(), String> {
        // Check environment variables first (AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY)
        if std::env::var("AWS_ACCESS_KEY_ID").is_ok() && std::env::var("AWS_SECRET_ACCESS_KEY").is_ok() {
            return Ok(());
        }

        // Check for credentials file
        if let Some(home) = dirs::home_dir() {
            let credentials_path = home.join(".aws").join("credentials");
            if credentials_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&credentials_path) {
                    let profile = std::env::var("AWS_PROFILE").unwrap_or_else(|_| "default".to_string());
                    let section_header = if profile == "default" {
                        "[default]".to_string()
                    } else {
                        format!("[profile {}]", profile)
                    };

                    if Self::parse_credentials_from_section(&content, &section_header) {
                        return Ok(());
                    }
                }
            }
        }

        Err(CREDENTIALS_ERROR_MSG.to_string())
    }

    /// Parse credentials from a specific section in the credentials file.
    /// Returns true if both access key and secret key are found and non-empty.
    fn parse_credentials_from_section(content: &str, section_header: &str) -> bool {
        let mut in_section = false;
        let mut has_access_key = false;
        let mut has_secret_key = false;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                in_section = line.eq_ignore_ascii_case(section_header);
                continue;
            }
            if in_section {
                if line.starts_with("aws_access_key_id") {
                    if let Some(value) = line.split('=').nth(1) {
                        if !value.trim().is_empty() {
                            has_access_key = true;
                        }
                    }
                } else if line.starts_with("aws_secret_access_key") {
                    if let Some(value) = line.split('=').nth(1) {
                        if !value.trim().is_empty() {
                            has_secret_key = true;
                        }
                    }
                }
            }
        }

        has_access_key && has_secret_key
    }
}

impl TTSProvider for PollyTTSProvider {
    fn speak(&mut self, text: &str) -> Result<(), TTSError> {
        debug!(chars = text.len(), "Polly: synthesizing speech");

        // Stop any current playback
        self.player.stop()?;

        // Call AWS Polly to synthesize speech
        let audio_bytes = self.runtime.block_on(async {
            let response = self
                .client
                .synthesize_speech()
                .text(text)
                .output_format(OutputFormat::Pcm)
                .voice_id(VoiceId::from(self.voice_id.as_str()))
                .engine(self.engine.clone())
                .sample_rate("16000")
                .send()
                .await
                .map_err(|e| TTSError::ProcessError(format!("AWS Polly API error: {e}")))?;

            let audio_stream = response.audio_stream;
            let bytes = audio_stream
                .collect()
                .await
                .map_err(|e| TTSError::ProcessError(format!("Failed to read audio stream: {e}")))?;

            Ok::<_, TTSError>(bytes.into_bytes().to_vec())
        })?;

        if audio_bytes.is_empty() {
            return Err(TTSError::ProcessError(
                "No audio data generated by AWS Polly".into(),
            ));
        }

        // Convert PCM to f32 and play
        let audio_data = AudioPlayer::pcm_to_f32(&audio_bytes);
        let duration_sec = audio_data.len() as f32 / 16000.0;
        info!(
            bytes = audio_bytes.len(),
            duration_sec = format!("{:.1}", duration_sec),
            "Polly: audio received"
        );

        self.player.play_audio(audio_data)
    }

    fn pause(&mut self) -> Result<(), TTSError> {
        self.player.pause()
    }

    fn resume(&mut self) -> Result<(), TTSError> {
        self.player.resume()
    }

    fn stop(&mut self) -> Result<(), TTSError> {
        self.player.stop()
    }

    fn is_playing(&self) -> bool {
        self.player.is_playing()
    }

    fn is_paused(&self) -> bool {
        self.player.is_paused()
    }

    fn skip_forward(&mut self, seconds: f32) {
        self.player.skip_forward(seconds);
    }

    fn skip_backward(&mut self, seconds: f32) {
        self.player.skip_backward(seconds);
    }

    fn get_progress(&self) -> f32 {
        self.player.get_progress()
    }

    fn get_frequency_bands(&self, num_bands: usize) -> Vec<f32> {
        self.player.get_frequency_bands(num_bands)
    }
}

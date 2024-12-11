use std::process::{Command, ExitStatus};
use std::path::Path;
use clap::{Arg, Command as ClapCommand};
use thiserror::Error;
use log::{info, error};

#[derive(Debug, Error)]
pub enum FFustError {
    #[error("Error executing command: {0}")]
    CommandExecutingError(String),

    #[error("Input or output file does not have a valid extension.")]
    InvalidFileExtension,

    #[error("Unsupported audio codec for the given file extension.")]
    UnsupportedAudioCodec,
}

pub struct FFust {
    input_file: String,
    output_file: String,
}

impl FFust {
    /// Creates a new instance of `FFust`.
    /// 
    /// # Arguments
    /// 
    /// - `input_file`: The path to the input file.
    /// - `output_file`: The path to the output file.
    /// 
    /// # Returns
    /// 
    /// A new instance of `FFust`.
    pub fn new(input_file: &str, output_file: &str) -> Self {
        Self {
            input_file: input_file.to_string(),
            output_file: output_file.to_string(),
        }
    }

    /// Checks if a filename has an extension.
    /// 
    /// # Arguments
    /// 
    /// - `filename`: The filename to check.
    /// 
    /// # Returns
    /// 
    /// `true` if the filename has an extension, `false` otherwise.
    pub fn has_extension(filename: &str) -> bool {
        Path::new(filename).extension().is_some()
    }

    /// Gets the appropriate audio codec based on the file extension.
    /// 
    /// # Arguments
    /// 
    /// - `filename`: The filename to extract the extension from.
    /// 
    /// # Returns
    /// 
    /// The corresponding audio codec, or an error if the extension is unsupported.
    pub fn get_audio_codec(filename: &str) -> Result<String, FFustError> {
        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "mp3" => Ok("libmp3lame".to_string()),
            "aac" => Ok("aac".to_string()),
            "wav" => Ok("pcm_s16le".to_string()),
            "flac" => Ok("flac".to_string()),
            "ogg" => Ok("libvorbis".to_string()),
            _ => Err(FFustError::UnsupportedAudioCodec),
        }
    }

    /// Executes a given command and returns its exit status.
    /// 
    /// # Arguments
    /// 
    /// - `command`: The command to be executed.
    /// 
    /// # Returns
    /// 
    /// `Ok(ExitStatus)` if the command executes successfully, or an error of type `FFustError` if it fails.
    pub fn execute_command(&self, command: &mut Command) -> Result<ExitStatus, FFustError> {
        match command.status() {
            Ok(status) if status.success() => Ok(status),
            Ok(status) => Err(FFustError::CommandExecutingError(format!(
                "Command failed with exit status: {:?}",
                status.code()
            ))),
            Err(e) => Err(FFustError::CommandExecutingError(format!(
                "Error during execution: {}",
                e
            ))),
        }
    }
    
    /// Converts a video file to an audio file using the appropriate codec.
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the conversion is successful, or an error of type `FFustError` if it fails.
    pub fn convert_video_to_audio(&self) -> Result<(), FFustError> {
        let audio_codec = Self::get_audio_codec(&self.output_file)?;
        
        let mut command = Command::new("ffmpeg");
        command.args([
            "-i", &self.input_file, 
            "-vn",
            "-acodec", &audio_codec,
            &self.output_file
        ]);
        
        self.execute_command(&mut command).map(|_| ())
    }

    /// Compresses a video file using H.265 codec.
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the compression is successful, or an error of type `FfustError` if it fails.
    pub fn compress_video(&self) -> Result<(), FFustError> {
        let mut command = Command::new("ffmpeg");
        command.args(["-i", &self.input_file, "-c:v", "libx265", "-crf", "23", "-preset", "medium", &self.output_file]);
        self.execute_command(&mut command).map(|_| ())
    }

    /// Converts a video file to another format.
    /// 
    /// # Returns
    /// 
    /// `Ok(())` if the conversion is successful, or an error of type `FfustError` if it fails.
    pub fn convert_to(&self) -> Result<(), FFustError> {
        let mut command = Command::new("ffmpeg");
        command.args(["-i", &self.input_file, &self.output_file]);
        self.execute_command(&mut command).map(|_| ())
    }
}

fn main() {
    env_logger::init();

    let matches = ClapCommand::new("ffust")
        .about("🦀 ffust is a simple FFmpeg wrapper.")
        .arg(Arg::new("operation")
            .help("The operation to perform.")
            .required(true)
            .value_parser(["get-audio", "compress", "convert"]))
        .arg(Arg::new("input-file")
            .help("Path to the input file.")
            .required(true))
        .arg(Arg::new("output-file")
            .help("Path to the output file.")
            .required(true))
        .get_matches();

    let operation = matches.get_one::<String>("operation").expect("Operation is required.");
    let input_file = matches.get_one::<String>("input-file").expect("Input file is required.");
    let output_file = matches.get_one::<String>("output-file").expect("Output file is required.");

    if !FFust::has_extension(input_file) || !FFust::has_extension(output_file) {
        error!("Input and output files must have a valid extension.");
        std::process::exit(1);
    }

    let ffust = FFust::new(&input_file, &output_file);

    let result = match operation.as_str() {
        "get-audio" => ffust.convert_video_to_audio(),
        "compress" => ffust.compress_video(),
        "convert" => ffust.convert_to(),
        _ => unreachable!(),
    };

    match result {
        Ok(()) => info!("Conversion completed successfully."),
        Err(e) => error!("Error during conversion: {}", e),
    }
}
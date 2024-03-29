use std::path::{Path, PathBuf};

use clap::builder::TypedValueParser as _;
use clap::Parser;

use ffms2::error::Result;
use ffms2::index::{Index, IndexErrorHandling, Indexer};
use ffms2::track::{Track, TrackType};
use ffms2::{Log, LogLevel, FFMS2};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Force overwriting of an existing index file whether is present.
    force: bool,
    /// Set `ffms2` verbosity level.
    #[arg(default_value_t = LogLevel::Debug,
        value_parser = clap::builder::PossibleValuesParser::new(LogLevel::all())
            .map(|s| s.parse::<LogLevel>().unwrap()))]
    verbose: LogLevel,
    /// Enable progress reporting.
    progress: bool,
    /// Write timecodes for all indexed video tracks into a file.
    timecodes: bool,
    /// Write key frames for all indexed video tracks into a file.
    keyframes: bool,
    /// Set the audio indexing mask to N.
    ///
    /// (-1 means index all tracks, 0 means index none)
    #[arg(short = 't', long = "index")]
    audio_index_mask: Option<u64>,
    /// Set audio decoding error handling.
    #[arg(short = 's', long = "audio-decoding", default_value_t = IndexErrorHandling::Abort,
        value_parser = clap::builder::PossibleValuesParser::new(IndexErrorHandling::all())
            .map(|s| s.parse::<IndexErrorHandling>().unwrap()))]
    ignore_errors: IndexErrorHandling,
    /// The file to be indexed
    input_file: PathBuf,
    /// The output file.
    ///
    ///If no output filename is specified, `input_filename.ffindex` will be used
    output_file: Option<PathBuf>,
}

#[inline(always)]
fn print_progress(args_progress: bool, error: &str) {
    if args_progress {
        println!("{error}");
    }
}

fn update_progress(
    current: usize,
    total: usize,
    private: Option<&mut usize>,
) -> usize {
    let percentage = ((current as f32 / total as f32) * 100.0) as usize;

    if let Some(percent) = private {
        if percentage <= *percent {
            return 0;
        }
        *percent = percentage;
    }

    println!("Indexing, please wait... {}%", percentage);
    0
}

fn write_frame_info<W>(
    index: &Index,
    cache_file: &Path,
    write_info: W,
) -> Result<()>
where
    W: Fn(Track, String) -> Result<()>,
{
    for track_id in 0..index.tracks_count() {
        let track = Track::from_index(&index, track_id)?;

        if track.frames_count() == 0 {
            println!("Track does not have frames. Skipping it.");
            continue;
        }

        if let TrackType::Video = track.track_type() {
            let filename = format!(
                "{}_track{:02}.tc.txt",
                cache_file.to_string_lossy(),
                track_id
            );
            write_info(track, filename)?;
        }
    }
    Ok(())
}

fn do_indexing(args: Args, cache_file: PathBuf) -> Result<()> {
    let mut progress = 0;

    let mut indexer = Indexer::new(&args.input_file)?;

    if args.progress {
        update_progress(0, 100, None);
        indexer.progress_callback(update_progress, &mut progress);
    }

    if let Some(audio_index_mask) = args.audio_index_mask {
        if audio_index_mask > 0 {
            for index_id in 0..64 {
                if ((audio_index_mask >> index_id) & 1) != 0 {
                    indexer.enable_track(index_id)?;
                }
            }
        }
    } else {
        indexer.enable_track_type(TrackType::Audio)?
    }

    let index = indexer.do_indexing(args.ignore_errors)?;

    if args.timecodes {
        print_progress(args.progress, "Writing timecodes...");

        write_frame_info(&index, &cache_file, |track, filename| {
            track.write_timecodes(Path::new(&filename))
        })?;

        print_progress(args.progress, "Done.");
    }

    if args.keyframes {
        print_progress(args.progress, "Writing keyframes...");

        for t in 0..index.tracks_count() {
            let track = Track::from_index(&index, t).unwrap();
            if track.frames_count() == 0 {
                println!("Track does not have frames. Skipping it.");
                continue;
            }

            if let TrackType::Video = track.track_type() {
                let filename = format!(
                    "{}_track{:02}.kf.txt",
                    cache_file.to_string_lossy(),
                    t
                );
                track.write_key_frames(Path::new(&filename))?;
            }
        }

        print_progress(args.progress, "Done.");
    }

    print_progress(args.progress, "Writing index...");

    index.write_to_file(&cache_file).unwrap();

    print_progress(args.progress, "Done.");

    Ok(())
}

fn main() {
    let args = Args::parse();

    let cache_file = if let Some(ref out) = args.output_file {
        out.clone()
    } else {
        let file_stem = args
            .input_file
            .as_path()
            .file_stem()
            .expect("Error in extracting the file stem")
            .to_string_lossy();
        PathBuf::from(format!("{file_stem}.ffindex"))
    };

    if cache_file.is_file() && !args.force {
        panic!("Error: index file already exists, use -f to overwrite it.");
    }

    FFMS2::init();

    Log::set_log_level(args.verbose);

    do_indexing(args, cache_file).unwrap();
}

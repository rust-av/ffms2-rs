extern crate ffms2;
extern crate structopt;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use ffms2::index::*;
use ffms2::track::*;
use ffms2::*;

macro_rules! print_progress {
    ($cond:expr, $error:expr) => {
        if $cond {
            println!($error);
        }
    };
}

#[derive(Debug, StructOpt)]
struct CliArgs {
    /// Force overwriting of existing index file, if any
    #[structopt(short = "f", long = "force")]
    force: bool,
    /// Set FFmpeg verbosity level
    #[structopt(short = "v", long = "verbose", default_value = "0")]
    verbose: usize,
    /// Disable progress reporting
    #[structopt(short = "p", long = "progress")]
    progress: bool,
    /// Write timecodes for all video tracks to outputfile_track00.tc.txt
    #[structopt(short = "c", long = "timecodes")]
    timecodes: bool,
    /// Write keyframes for all video tracks to outputfile_track00.kf.txt
    #[structopt(short = "k", long = "keyframes")]
    keyframes: bool,
    /// Set the audio indexing mask to N
    /// (-1 means index all tracks, 0 means index none)
    #[structopt(short = "t", long = "index", default_value = "0")]
    index_mask: i64,
    /// Set audio decoding error handling
    #[structopt(short = "s", long = "audio-decoding", default_value = "0")]
    ignore_errors: usize,
    /// The file to be indexed
    #[structopt(parse(from_os_str))]
    input_file: PathBuf,
    /// The output file.
    /// If no output filename is specified, input_file.ffindex will be used
    #[structopt(parse(from_os_str))]
    output_file: Option<PathBuf>,
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

#[inline]
fn dump_filename(
    track: &Track,
    track_num: usize,
    cache_file: &PathBuf,
    suffix: &str,
) -> PathBuf {
    if track.NumFrames() == 0 {
        return PathBuf::new();
    }

    if let TrackType::TYPE_VIDEO = track.TrackType() {
        let start = cache_file.to_str().unwrap();
        let filename = format!("{}_track{:02}{}", start, track_num, suffix);
        PathBuf::from(filename)
    } else {
        PathBuf::new()
    }
}

fn do_indexing(
    args: &CliArgs,
    cache_file: &PathBuf,
    ignore_errors: IndexErrorHandling,
) -> std::io::Result<()> {
    let mut progress = 0;

    if cache_file.as_path().exists() && !args.force {
        panic!(
            "Error: index file already exists, \
             use -f if you are sure you want to overwrite it."
        );
    }

    let indexer = Indexer::new(&args.input_file).unwrap();

    if args.progress {
        update_progress(0, 100, None);
        indexer.ProgressCallback(update_progress, &mut progress);
    }

    if args.index_mask == -1 {
        indexer.TrackTypeIndexSettings(TrackType::TYPE_AUDIO, 1);
    }

    for i in 0..64 {
        if ((args.index_mask >> i) & 1) != 0 {
            indexer.TrackIndexSettings(i, 1);
        }
    }

    let index = indexer.DoIndexing2(ignore_errors).unwrap();

    if args.timecodes {
        print_progress!(args.progress, "Writing timecodes...");
        let num_tracks = index.NumTracks();
        for t in 0..num_tracks {
            let track = Track::TrackFromIndex(&index, t);
            let filename = dump_filename(&track, t, &cache_file, ".tc.txt");
            if !filename.to_str().unwrap().is_empty()
                && track.WriteTimecodes(&filename).is_err()
            {
                println!(
                    "Failed to write timecodes file {}",
                    filename.to_str().unwrap()
                );
            }
        }
        print_progress!(args.progress, "Done.");
    }

    if args.keyframes {
        print_progress!(args.progress, "Writing keyframes...");
        let num_tracks = index.NumTracks();
        for t in 0..num_tracks {
            let track = Track::TrackFromIndex(&index, t);
            let filename = dump_filename(&track, t, &cache_file, ".kf.txt");
            if !filename.to_str().unwrap().is_empty() {
                let mut file = File::create(filename)?;
                write!(file, "# keyframe format v1\nfps 0\n")?;
                let frame_count = track.NumFrames();
                for f in 0..frame_count {
                    if track.FrameInfo(f).KeyFrame() != 0 {
                        writeln!(file, "{}", f)?;
                    }
                }
            }
        }
        print_progress!(args.progress, "Done.");
    }

    print_progress!(args.progress, "Writing index...");

    index.WriteIndex(&cache_file).unwrap();

    print_progress!(args.progress, "Done.");

    Ok(())
}

fn main() {
    let args = CliArgs::from_args();

    if args.ignore_errors > 3 {
        panic!("Error: invalid audio decoding error handling mode");
    }

    let cache_file = if let Some(out) = &args.output_file {
        out.to_path_buf()
    } else {
        let file_stem = args
            .input_file
            .as_path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let filename = format!("{}.ffindex", file_stem);
        Path::new(&filename).to_path_buf()
    };

    FFMS2::Init();

    let level = match args.verbose {
        0 => LogLevels::LOG_QUIET,
        1 => LogLevels::LOG_WARNING,
        2 => LogLevels::LOG_INFO,
        3 => LogLevels::LOG_VERBOSE,
        _ => LogLevels::LOG_DEBUG,
    };

    Log::SetLogLevel(level);

    let ignore_errors = match args.ignore_errors {
        0 => IndexErrorHandling::IEH_IGNORE,
        1 => IndexErrorHandling::IEH_STOP_TRACK,
        2 => IndexErrorHandling::IEH_CLEAR_TRACK,
        _ => IndexErrorHandling::IEH_ABORT,
    };

    do_indexing(&args, &cache_file, ignore_errors).unwrap();
}

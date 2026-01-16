use indexmap::IndexMap;
use matroska_demuxer::{Frame, MatroskaFile, TrackType};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Accept MKV file from command line
    let path = std::env::args()
        .nth(1)
        .expect("Usage: mkv_sizes <file.mkv>");
    let file = File::open(&path)?;

    // Increase buffer size for better I/O performance (default is 8KB, we use 1MB)
    let mut reader = BufReader::with_capacity(1024 * 1024, file);

    // Open MKV using MatroskaFile
    let mut mkv = MatroskaFile::open(&mut reader)?;

    // Pre-allocate HashMap with known capacity
    let track_count = mkv.tracks().len();
    
    // Prepare map: TrackNumber -> (TrackType, total bytes)
    let mut sizes: IndexMap<u64, (TrackType, u64)> = IndexMap::with_capacity(track_count);

    // Initialize tracks
    for track in mkv.tracks().iter() {
        let number = track.track_number().get();
        let ttype = track.track_type();
        sizes.insert(number, (ttype, 0));
    }

    let mut frame = Frame::default();

    // Iterate frames until end
    while mkv.next_frame(&mut frame)? {
        let id = frame.track;
        if let Some(entry) = sizes.get_mut(&id) {
            entry.1 += frame.data.len() as u64;
        }
    }

    println!("Track sizes for : {}", path);

    let mut total_bytes = 0u64;
    // Print results
    for (track_num, (t_type, bytes)) in sizes {
        let mb = bytes as f64 / (1024.0 * 1024.0);
        println!(
            "Track {:>2}  | {:<8} | {:>9.2} MB ({:>} bytes)",
            track_num,
            format!("{:#?}", t_type),
            mb,
            bytes
        );
        total_bytes += bytes;
    }

    // Print total
    let total_mb = total_bytes as f64 / (1024.0 * 1024.0);
    println!("{:-<50}", "");
    println!(
        "Total     | {:<8} | {:>9.2} MB ({:>} bytes)",
        "", total_mb, total_bytes
    );

    Ok(())
}

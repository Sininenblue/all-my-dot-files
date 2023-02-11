use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = vec![
        "--yes-playlist",
        "-o",
        "%(title)s.%(ext)s",
        "-x",
        "--embed-thumbnail",
        "--embed-metadata",
        "--audio-format",
        "mp3",
    ];

    let index = &args[1];
    let link = &args[2];
    
    options.push("-I");
    options.push(index);
    options.push(link);

    let output = Command::new("yt-dlp")
        .args(options)
        .output()
        .expect("something fucked up");

    let output = String::from_utf8_lossy(&output.stdout).into_owned();

    println!("{}", output);
}

// import some sort of yt-dlp helper
//
// get args
// if there are more than 1 arguments
// we can assume that thre is a playlist
// then we check if there are more than 2
// if there are, we can assume that htey specified an index
// so audio -p 10:20 [link]
//
// has more than 1
// has more than 2
// download playlist iwth -I 10:20
//
// I think it would be better if we do
// if args[] more than 1
// for argument in extra_arguments
//     match argument to different possible options

import sys
import subprocess

arguments = sys.argv
default_options = [
    "yt-dlp",
    "-o",
    "%(title)s.%(ext)s",
    "-x",
    "--embed-thumbnail",
    "--embed-metadata",
    "--audio-format",
    "mp3",
    ]

if len(arguments) == 2:
    default_options.append("--no-playlist")
    default_options.append(arguments[1])

    print(default_options)
    # make yt-dlp run a test run first
    # subprocess.run(default_options)
else:
    for option in arguments:
        match option:
            case '-P':
                default_options.append("--yes-playlist")
            case '-I':
                default_options.append("-I")
                playlist_index = str(arguments[arguments.index(option) + 1])
                default_options.append(playlist_index)

    default_options.append(arguments.pop(len(arguments) - 1))

    print(default_options)

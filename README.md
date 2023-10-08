# Sonify
A video sonification software built with rust.

Warning: this project is not polished and may produce some static noise in the sound file.

Note:
  This program only runs on linux and mac (if some dependencies are configured right).

requirements:
- ffmpeg
- cargo

Used rust packages:
- image (0.24.7): https://crates.io/crates/image

usage:
```bash
chmod +x sonify.sh
./sonify.sh videoname.mp4
```


Note:
 This program may not sync the audio with the video and might leave a few seconds blank. You can trim the blank part by using ffmpeg. Trimming:
 ```bash
 # Replace output.mp4 with the file that was made by the program.
 # also replace the timestamps. (start, finish)
 ffmpeg -i output.mp4 -ss 00:00:00 -t 00:00:36 -c:v copy -c:a copy trimmed_output.mp4
```

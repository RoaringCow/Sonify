use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
extern crate image;

//use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use image::GenericImageView;
use std::path::Path;
use std::fs;
//use std::path::Path;

const PI:f64 = 3.1415926535;
const BITDEPTH:u16 = 16;
const SAMPLERATE:u32 = 48000;
const CHANNELS:u16 = 1;
const BLOCKALIGN:u16 = BITDEPTH/2;
const BYTERATE:u32 = SAMPLERATE*BITDEPTH as u32/8;
const FORMAT:u16 = 1; // WAVE_FORMAT_PCM
const CHUNKSIZE:u32 = 16;





fn main() -> std::io::Result<()> {
    
    
    // open file
    let mut output_file = File::create("ses.wav")?;
    
    // Header
    // - RIFF
    output_file.write_all(&[0x52,0x49,0x46,0x46])?;
    // - ---- place holder
    let pos_cksize = output_file.stream_position()?;
    output_file.write_all("----".as_bytes())?;
    output_file.write_all("WAVE".as_bytes())?;
    
    //  Format
    output_file.write_all("fmt ".as_bytes())?;
    output_file.write_all(&CHUNKSIZE.to_le_bytes())?;
    output_file.write_all(&FORMAT.to_le_bytes())?;
    output_file.write_all(&CHANNELS.to_le_bytes())?;
    output_file.write_all(&SAMPLERATE.to_le_bytes())?;
    output_file.write_all(&BYTERATE.to_le_bytes())?;
    output_file.write_all(&BLOCKALIGN.to_le_bytes())?;
    output_file.write_all(&BITDEPTH.to_le_bytes())?;
    
    // Data
    output_file.write_all("data".as_bytes())?;
    let pos_data_placeholder = output_file.stream_position()?;
    output_file.write_all("----".as_bytes())?;
    let pos_data_start = output_file.stream_position()?;
    // BOILERPLATE DIŞI
    
    
   
    
    let dir_path = "frames";
    
    // Read the contents of the directory
    let entries = fs::read_dir(dir_path).unwrap();
    
    println!("Files: {}", entries.count());

    let entries: Vec<_> = fs::read_dir(dir_path)?.collect();

    println!("Files: {}", entries.len());

    // Collect and sort the file names alphabetically
    let mut file_names: Vec<String> = entries
        .iter()
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    Some(file_name.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    file_names.sort(); // Sort the file names alphabetically

    // Process the files in alphabetical order
    for file_name in file_names {
        let file_path = Path::new(dir_path).join(&file_name);
        //println!("Processing file: {:?}", file_path);

        let img = image::open(file_path).unwrap();

        let mut row_value: f32 = 0.0;
        for y in 0..img.height() {
            for x in 0..img.width() {
                // Get pixel from coordinates
                let rgba = img.get_pixel(x, y); 
                let pixel_brightness = (rgba[0] as f32 * 0.21) + (rgba[1] as f32 * 0.72) + (rgba[2] as f32 * 0.07);
                // This value can be changed to adjust the frequency (Higher = Higher frequency)
                row_value += pixel_brightness * 10.0;
            }
        }
        //println!("{}    {}", row_value, file_name);
        // duration argument can be changed in order to use a different video speed.(Other than 30fps)
        write_note(&mut output_file, 30.0, (row_value / (img.width() * img.height()) as f32) as f64);
    }




    
    // BOILERPLATE DEVAM
    let mut pos_end = output_file.stream_position()?;
    
    let chunk_size_data:u32 = (pos_end - pos_data_start) as u32;
    if chunk_size_data % 2 != 0 {
        output_file.write_all(&[0x00])?;
        pos_end = output_file.stream_position()?;
    }
    output_file.seek(SeekFrom::Start(pos_data_placeholder))?;
	
	output_file.write_all(&chunk_size_data.to_le_bytes())?;
	
    output_file.seek(SeekFrom::Start(pos_cksize))?;
    let chunk_size_header:u32 = (pos_end - 8) as u32;
    output_file.write_all(&chunk_size_header.to_le_bytes())?;
    
    output_file.sync_all()?;
    Ok(())
}


fn write_note(output_file: &mut File, duration: f32, frequency: f64){
    let amplitude:f64 = 0.5; 
    let offset:f64 = 2.0*PI*(frequency as i64)as f64/(SAMPLERATE as f64);
    let mut angle:f64 = 0.0;
    let samples_required:u64 = (SAMPLERATE as f32 * (duration / 1000.0)) as u64;
    //println!("{}", ((SAMPLERATE as f32 * (duration / 1000.0)) as u64));
    
    //println!("{}", frequency);
    let mut sample:f64;
    let mut sample_to_write:i16;
    let max_amplitude:f64 = 2.0f64.powi((BITDEPTH-1).into()) - 1.0;
    
    for _ in 1..samples_required
    {
        sample = amplitude * angle.sin();
        angle += offset;
        sample_to_write = (sample * max_amplitude) as i16;
        let _ = output_file.write_all(&sample_to_write.to_le_bytes());
    }/*
    for x in 1..5{
        let blank: i16 = (sample_to_write / x) as i16 ;
        let _ = output_file.write_all(&blank.to_le_bytes());
    }
    */
    

    
}


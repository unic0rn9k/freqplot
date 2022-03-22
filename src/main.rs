use minimp3::{Decoder, Error, Frame};
use sfft::*;
use std::{env::var, fs::File, mem::transmute};

const LEN: usize = 2usize.pow(14);
const PLOT_X_LEN: usize = 1000;

fn main() {
    let input_file =
        var("AUDIO_FILE").expect("Expected AUDIO_FILE to contain path to audio file to plot");

    let mut decoder = Decoder::new(File::open(input_file.clone()).unwrap());

    let mut row_nr = 0;
    let mut samples = 0;

    let mut buffer = vec![re(0f32); LEN * 2];
    let mut sound_map: Vec<Vec<f32>> = Vec::new();

    'outer: loop {
        match decoder.next_frame() {
            Ok(Frame {
                data,
                sample_rate,
                channels,
                ..
            }) => {
                assert_eq!(channels, 1);

                for data_idx in 0..data.len() {
                    buffer[samples % LEN] = re(data[data_idx] as f32);
                    buffer[samples % LEN + LEN] = re(data[data_idx] as f32);

                    if samples / LEN >= 2 && samples % 10 == 0 {
                        println!("Generating row {row_nr}");

                        let sample: &[Complex<f32>; LEN] =
                            unsafe { transmute(&buffer[samples % LEN]) };

                        let mut buffer2 = fft(sample);

                        let mut row_buffer = [0.; PLOT_X_LEN];

                        for n in 0..LEN {
                            row_buffer[(n * sample_rate as usize / LEN).min(PLOT_X_LEN - 1)] +=
                                buffer2[n].re / LEN as f32;
                        }

                        sound_map.push(row_buffer.to_vec());
                        row_nr += 1;
                    }

                    samples += 1;
                    if row_nr > 4000 {
                        break 'outer;
                    }
                }
            }
            Err(Error::Eof) => break 'outer,
            Err(e) => panic!("{:?}", e),
        }
    }

    println!("Generating plot");

    use plotly::common::{ColorScale, ColorScalePalette, Title};
    use plotly::contour::Contours;
    use plotly::{Contour, HeatMap, Layout, Plot};

    let trace = HeatMap::new_z(sound_map);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    if var("HTML_PLOT").is_ok() {
        println!("Showing plot");
        plot.show()
    } else {
        println!("Saving plot");
        plot.save(
            format!("plots/{input_file}.png"),
            plotly::ImageFormat::PNG,
            1920,
            1080,
            1.0,
        );
    }
}

use minimp3::{Decoder, Error, Frame};
use sfft::*;
use std::io::Write;
use std::{env::var, fs::File, mem::transmute};

const SAMPLE_LEN: usize = 2usize.pow(14);
const PLOT_WIDTH: usize = 3000;

/*
struct RingBuffer<T, const LEN: usize> {
    buffer: [T; LEN],
    index: usize,
}

impl<T, const LEN: usize> RingBuffer<T, LEN> {
    fn new(buffer: [T; LEN]) -> Self {
        RingBuffer { buffer, index: 0 }
    }

    fn push(&mut self, v: T) {
        if self.index > LEN {
            self.index = 0
        }
        self.buffer[self.index] = v;
        self.index += 1
    }

    fn pop(&mut self) -> T {
        if self.index == 0 {
            self.index = LEN
        }
        let res = self.buffer[self.index];
        self.index -= 1;
        res
    }
}*/

fn main() {
    let input_file =
        var("AUDIO_FILE").expect("Expected AUDIO_FILE to contain path to audio file to plot");

    let mut decoder = Decoder::new(File::open(input_file.clone()).unwrap());

    let mut samples = 0;

    let mut buffer = vec![re(0f32); SAMPLE_LEN * 2];
    let mut sound_map: Vec<Vec<f32>> = Vec::new();
    let mut peak_freq = vec![];

    println!("Transforming audio data");

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
                    buffer[samples % SAMPLE_LEN] = re(data[data_idx] as f32);
                    buffer[samples % SAMPLE_LEN + SAMPLE_LEN] = re(data[data_idx] as f32);

                    if samples / SAMPLE_LEN >= 2 && samples % 40 == 0 {
                        let sample: &[Complex<f32>; SAMPLE_LEN] =
                            unsafe { transmute(&buffer[samples % SAMPLE_LEN]) };

                        let buffer2 = fft(sample);

                        let mut row_buffer = [0.; PLOT_WIDTH];
                        let mut row_max = 0;

                        for i in 0..SAMPLE_LEN {
                            let j = (i * sample_rate as usize / SAMPLE_LEN).min(PLOT_WIDTH - 1);

                            row_buffer[j] += buffer2[i].re;
                            //row_buffer[(j + 1).min(PLOT_WIDTH - 1)] -= buffer2[i].re;

                            if j != PLOT_WIDTH - 1 && row_buffer[j] > row_buffer[row_max] {
                                row_max = j
                            }
                        }

                        peak_freq.push(row_max);
                        sound_map.push(row_buffer.to_vec());
                    }

                    samples += 1;
                }
            }
            Err(Error::Eof) => break 'outer,
            Err(e) => panic!("Error while reading audio: {:?}", e),
        }
    }

    println!("Saving peak frequencies");
    writeln!(
        &mut File::create(format!("{input_file}.csv")).unwrap(),
        "freq,\n{}",
        peak_freq
            .iter()
            .map(|f| format!("{}", f))
            .collect::<Vec<_>>()
            .join(",\n")
    )
    .unwrap();

    println!("Generating plot");

    use plotly::{HeatMap, Plot};

    let plot_height = sound_map.len() + 50;
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
            2160,
            plot_height,
            1.0,
        );
    }
    println!("Done!");
}

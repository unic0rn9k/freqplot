use minimp3::{Decoder, Error, Frame};
use sfft::*;
use std::{fs::File, mem::transmute};

const LEN: usize = 2usize.pow(14);

fn plot_vector(v: &[[f32; 2]; LEN], sample_rate: f64, name: &str) {
    use itertools_num::*;
    use plotly::*;

    let t: Vec<f64> = linspace(0., sample_rate, LEN).collect();
    let len = 500;

    let trace_re = Scatter::new(t.clone(), v[0..len].iter().map(|n| n[0] / LEN as f32)).name("re");
    let trace_im = Scatter::new(t, v[0..len].iter().map(|n| n[1] / LEN as f32)).name("im");

    let mut plot = Plot::new();
    plot.add_trace(trace_re);
    plot.add_trace(trace_im);
    let layout = Layout::new().height(300);
    plot.set_layout(layout);
    plot.save(name, ImageFormat::PNG, 1024, 680, 1.0);
    //plot.show();
}

fn main() {
    let mut decoder = Decoder::new(File::open("data/stemmegaffel/440hz_ekstra.mp3").unwrap());

    let mut plot_nr = 0;
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

                    if samples / LEN >= 2 {
                        println!("Generating row {plot_nr}...");

                        let sample: &[Complex<f32>; LEN] =
                            unsafe { transmute(&buffer[samples % LEN]) };

                        let mut buffer2 = fft(sample);

                        //unsafe {
                        //    plot_vector(
                        //        transmute(&buffer2),
                        //        sample_rate as f64,
                        //        &format!("plots/{}", plot_nr),
                        //    )
                        //};

                        let mut row_buffer = [0.; 1000];

                        for n in 0..LEN {
                            row_buffer[(n * sample_rate as usize / LEN).min(999)] +=
                                buffer2[n].re / LEN as f32;
                        }

                        sound_map.push(row_buffer.to_vec());
                        plot_nr += 1;
                    }

                    samples += 1;
                    if plot_nr > 1000 {
                        break 'outer;
                    }
                }
            }
            Err(Error::Eof) => break 'outer,
            Err(e) => panic!("{:?}", e),
        }
    }

    use plotly::common::{ColorScale, ColorScalePalette, Title};
    use plotly::contour::Contours;
    use plotly::{Contour, HeatMap, Layout, Plot};

    let trace = HeatMap::new_z(sound_map);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.save("soundmap.png", ImageFormat::PNG, 1024, 680, 1.0);
}

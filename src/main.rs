use minimp3::{Decoder, Error, Frame};
use sfft::*;
use std::{fs::File, mem::transmute};

const LEN: usize = 2usize.pow(14);

fn plot_vector(v: &[[f32; 2]; LEN], sample_rate: f64, name: &str) {
    use itertools_num::*;
    use plotly::*;

    let t: Vec<f64> = linspace(0., sample_rate, LEN).collect();
    let len = 500;

    let trace_re = Scatter::new(t.clone(), v[0..len].iter().map(|n| n[0])).name("re");
    let trace_im = Scatter::new(t, v[0..len].iter().map(|n| n[1])).name("im");

    let mut plot = Plot::new();
    plot.add_trace(trace_re);
    plot.add_trace(trace_im);
    let layout = Layout::new().height(300);
    plot.set_layout(layout);
    plot.save(name, ImageFormat::PNG, 1024, 680, 1.0);
}

fn main() {
    let mut decoder = Decoder::new(File::open("440ishhz.mp3").unwrap());

    let mut plot_nr = 0;
    let mut samples = 0;

    let mut buffer = [re(0f32); LEN];

    loop {
        match decoder.next_frame() {
            Ok(Frame {
                data,
                sample_rate,
                channels,
                ..
            }) => {
                assert_eq!(channels, 1);
                //println!("{} samples, with sample rate {}", data.len(), sample_rate);

                for n in 0..data.len() {
                    buffer[samples] = re(data[n] as f32);
                    samples += 1;
                    if samples >= LEN {
                        println!("Generating new plot...");
                        unsafe {
                            buffer = fft::<LEN>(&buffer);
                            plot_vector(
                                transmute(&buffer),
                                sample_rate as f64,
                                &format!("plots/{}", plot_nr),
                            )
                        };
                        plot_nr += 1;
                        samples = 0;
                        buffer = [re(0f32); LEN];
                    }
                }
            }
            Err(Error::Eof) => return (),
            Err(e) => panic!("{:?}", e),
        }
    }
}

use ::core::f32::consts::TAU;
pub fn bark_freq(samplerate: f32) -> [f32; 24] {
  [ 
    TAU * 100.0   / samplerate,
    TAU * 200.0   / samplerate,
    TAU * 300.0   / samplerate,
    TAU * 400.0   / samplerate,
    TAU * 510.0   / samplerate,
    TAU * 630.0   / samplerate,
    TAU * 770.0   / samplerate,
    TAU * 920.0   / samplerate,
    TAU * 1080.0  / samplerate,
    TAU * 1270.0  / samplerate,
    TAU * 1480.0  / samplerate,
    TAU * 1720.0  / samplerate,
    TAU * 2000.0  / samplerate,
    TAU * 2320.0  / samplerate,
    TAU * 2700.0  / samplerate,
    TAU * 3150.0  / samplerate,
    TAU * 3700.0  / samplerate,
    TAU * 4400.0  / samplerate,
    TAU * 5300.0  / samplerate,
    TAU * 6400.0  / samplerate,
    TAU * 7700.0  / samplerate,
    TAU * 9500.0  / samplerate,
    TAU * 12000.0 / samplerate,
    TAU * 15500.0 / samplerate,
  ]
}

pub fn bark_bw() -> [f32; 24] {
  [
    80.0,      //   0.800
    100.0,     //   2.000
    100.0,     //   3.000
    100.0,     //   4.000
    110.0,     //   4.636
    120.0,     //   5.250
    140.0,     //   5.500
    150.0,     //   6.133
    160.0,     //   6.750
    190.0,     //   6.683
    210.0,   //   7.047
    240.0,   //   7.167
    280.0,   //   7.143
    320.0,   //   7.250
    380.0,   //   7.105
    450.0,   //   7.000
    550.0,   //   6.726
    700.0,   //   6.285
    900.0,   //   5.888
    1100.0,  //   5.818
    1300.0,  //   5.923
    1800.0,  //   5.277
    2500.0,  //   4.800
    3500.0,   //   4.428
  ]
}

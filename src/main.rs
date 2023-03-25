use clap::{Parser, Subcommand};
use std::process::Command;

union DrmCtm {
    linux: [u64; 9],
    xrandr: [u32; 18],
}

impl DrmCtm {
    fn to_xrandr_cli(&self) -> String {
        let xrandr = unsafe { self.xrandr };
        xrandr.map(|n| n.to_string()).join(",")
    }
}

struct FloatCtm([f64; 9]);

impl Default for FloatCtm {
    fn default() -> Self {
        Self([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }
}

impl FloatCtm {
    fn to_linux_ctm(&self) -> DrmCtm {
        DrmCtm {
            linux: self.0.map(|x| {
                if x < 0.0 {
                    (-x * (1u64 << 32) as f64) as u64 | (1u64 << 63)
                } else {
                    (x * (1u64 << 32) as f64) as u64
                }
            }),
        }
    }

    fn from_saturation(sat: f64) -> Self {
        let coeff = (1.0 - sat) / 3.0;
        FloatCtm([
            coeff + sat,
            coeff,
            coeff,
            coeff,
            coeff + sat,
            coeff,
            coeff,
            coeff,
            coeff + sat,
        ])
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// small tool for setting Xorg color settings
struct Args {
    #[arg(short, long)]
    /// the name of the display to apply the settings to, you can
    /// run 'xrandr' to discover what it is. Defaults to 'eDP'
    display: Option<String>,

    #[arg(short, long, value_name = "0.2..4.0")]
    /// brightness multiplier, this will clip, so prefer increasing
    /// your display's backlight instead
    brightness: Option<f64>,

    #[arg(short, long, value_name = "0.2..10.0")]
    /// adjust overall gamma
    gamma: Option<f64>,

    #[arg(short = 'R', long)]
    red_gamma: Option<f64>,

    #[arg(short = 'G', long)]
    green_gamma: Option<f64>,

    #[arg(short = 'B', long)]
    blue_gamma: Option<f64>,

    // TODO:
    // #[arg(short, long)]
    // /// adjust color temperature in Kelvin
    // temperature: Option<u64>,

    // #[arg(long)]
    // /// adjust green-magenta tint
    // tint: Option<f64>,

    //
    #[arg(short, long, value_name = "0.0..4.0")]
    saturation: Option<f64>,

    #[arg(long, value_delimiter = ',', value_name = "r,g,b")]
    /// set the individual brightness of each color, as a comma
    /// separated list.
    rgb_gain: Option<Vec<f64>>,

    #[arg(long, value_delimiter = ',', value_name = "rr,rg,rb,gr,gg,gb,br,bg,bb")]
    /// set the values of the CTM explicitly. The CTM is a 3x3 matrix
    /// that maps input RGB to output RGB. Values are given in row-major
    /// order.
    ctm: Option<Vec<f64>>,

    #[arg(long)]
    /// reset to default values
    reset: bool,
}

fn main() {
    let args = Args::parse();

    let mut cmd = Command::new("xrandr");
    cmd.arg("--output").arg("eDP");

    // ==============================================================

    if let Some(brightness) = args.brightness {
        if brightness < 0.2 || brightness > 4.0 {
            panic!("brightness must be between 0.2 and 4.0");
        }
        cmd.arg("--brightness").arg(brightness.to_string());
    }

    if let Some(gamma) = args.gamma {
        if gamma < 0.2 || gamma > 10.0 {
            panic!("gamma must be between 0.2 and 10.0");
        }
        cmd.arg("--gamma").arg(gamma.to_string());
    }

    if let Some(red_gamma) = args.red_gamma {
        cmd.arg("--gamma").arg(red_gamma.to_string() + ":1:1");
    }

    if let Some(green_gamma) = args.green_gamma {
        cmd.arg("--gamma")
            .arg("1:".to_owned() + &green_gamma.to_string() + ":1");
    }

    if let Some(blue_gamma) = args.blue_gamma {
        cmd.arg("--gamma")
            .arg("1:1:".to_owned() + &blue_gamma.to_string());
    }

    // ==============================================================

    if let Some(saturation) = args.saturation {
        cmd.arg("--set").arg("CTM").arg(
            FloatCtm::from_saturation(saturation)
                .to_linux_ctm()
                .to_xrandr_cli(),
        );
    }

    if let Some(rgb) = args.rgb_gain {
        cmd.arg("--set").arg("CTM").arg(
            FloatCtm([rgb[0], 0.0, 0.0, 0.0, rgb[1], 0.0, 0.0, 0.0, rgb[2]])
                .to_linux_ctm()
                .to_xrandr_cli(),
        );
    }

    if let Some(ctm) = args.ctm {
        cmd.arg("--set").arg("CTM").arg(
            FloatCtm(ctm.try_into().unwrap())
                .to_linux_ctm()
                .to_xrandr_cli(),
        );
    }

    // ==============================================================

    if args.reset {
        cmd.arg("--set")
            .arg("CTM")
            .arg(FloatCtm::default().to_linux_ctm().to_xrandr_cli())
            .arg("--brightness")
            .arg("1.0")
            .arg("--gamma")
            .arg("1.0");
    }

    let status = cmd.status().expect("failed to execute xrandr");
    dbg!(status);
}

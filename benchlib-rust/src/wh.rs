const S1_MOD: f32 = 30269f32;
const S2_MOD: f32 = 30307f32;
const S3_MOD: f32 = 30323f32;

pub struct WichmannHillRng {
    s1: f32,
    s2: f32,
    s3: f32
}

impl WichmannHillRng {
    fn new(s1: f32, s2: f32, s3: f32) -> WichmannHillRng {
        // TODO: check if 1< s1,s2,s3 > 30_000
        WichmannHillRng {
            s1: s1,
            s2: s2,
            s3: s3
        }
    }

    pub fn seeded(seed: u32) -> WichmannHillRng {
        let t = seed;
        let s1 = (t % 29999) as f32;
        let s2 = (t % 29347) as f32;
        let s3 = (t % 29097) as f32;
        WichmannHillRng::new(s1, s2, s3)
    }

    pub fn next_f32(&mut self) -> f32 {
        self.s1 = (171f32 * self.s1) % S1_MOD;
        self.s2 = (172f32 * self.s2) % S2_MOD;
        self.s3 = (170f32 * self.s3) % S3_MOD;
        (self.s1 / S1_MOD + self.s2 / S2_MOD + self.s3 / S3_MOD) % 1f32 
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;

    fn mean(seq: &Vec<f32>) -> f32 {
        seq.iter().fold(0f32, |p,c| p+c) / seq.len() as f32
    }

    fn stddev(seq: &Vec<f32>) -> f32 {
        let N = seq.len() as f32;
        let mu = mean(seq);
        (seq.iter().fold(0f32, |p, c| p + (c - mu).powi(2)) / N).sqrt()
    }

    #[test]
    fn test_seeds() {
        let mut wh = WichmannHillRng {
            s1: 1f32,
            s2: 1f32,
            s3: 1f32
        };
        assert_eq!(wh.next_f32(), 0.016930906);
        assert_eq!(wh.next_f32(), 0.89525414);
        assert_eq!(wh.next_f32(), 0.111491084);
    }
   

    #[bench]
    fn bench_next_f32(b: &mut Bencher) {
        let mut wh = WichmannHillRng {
            s1: 1f32,
            s2: 1f32,
            s3: 1f32
        };

        b.iter(|| wh.next_f32());
    }

}
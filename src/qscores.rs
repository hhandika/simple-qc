//! Heru Handika
//! Only support Illumina 1.8 Quality Scores

pub struct QScore {
    pub q_len: u32,
    pub mean_q: f64,
    pub low_bases: u32,
    pub sum: u32,
}

impl QScore {
    pub fn analyze_qscores(q_line: &[u8]) -> Self {
        let q_scores = q_line.iter()
            .map(|scr| 
                    { if *scr < 75 {
                        *scr as u32 - 33
                    } else {
                        panic!("UNSUPPORTED Q-SCORE ENCODING!");
                    }
                })
            .collect::<Vec<u32>>(); 

        let mut q = Self {
                q_len: q_scores.iter().count() as u32,
                low_bases: q_scores.iter()
                    .filter(|&x| *x < 20)
                    .count() as u32,
                sum: q_scores.iter().sum(),
                mean_q: 0.0
            };

        q.mean();

        q
    }

    fn mean(&mut self) {
        self.mean_q = self.sum as f64 / self.q_len as f64
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qscore_test () {
        let p = String::from("II!)");
        let q = String::from("II");

        let q_score = QScore::analyze_qscores(q.as_bytes());
        let p_score = QScore::analyze_qscores(p.as_bytes());

        assert_eq!(2, q_score.q_len);
        assert_eq!(40.0, q_score.mean_q);
        assert_eq!(0, q_score.low_bases);
        assert_eq!(2, p_score.low_bases);
    }

    #[test]
    fn decode_qscores_test() {
        let q = String::from("II");
        let qs = QScore::analyze_qscores(q.as_bytes());

        // let res = vec![40, 40];

        assert_eq!(80, qs.sum);
    }

    #[test]
    #[should_panic(expected = "UNSUPPORTED Q-SCORE ENCODING!")]
    fn decode_panic_qscore() {
        let p = String::from("II!)K");

        QScore::analyze_qscores(&p.as_bytes());
    }
}

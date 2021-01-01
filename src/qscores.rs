//! Heru Handika
//! 31 December 2020
//! Decode Illumina 1.8 Quality Scores

pub struct QScore {
    pub q_len: u32,
    pub mean_q: f64,
}

fn decode_qscores(q_line: &[u8]) -> Vec<u32> {
    q_line.iter()
        .map(|scr| *scr as u32 - 33)
        .collect()
}

impl QScore {
    pub fn analyze_qscores(q_line: &[u8]) -> Self {
        let q_scores = decode_qscores(&q_line); 

        let mut q = Self {
                q_len: q_scores.iter().count() as u32,
                mean_q: 0.0,
            };
        
        let sum_q: u32 = q_scores.iter().sum();
        q.mean_q = sum_q as f64 / q.q_len as f64;

        q
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qscore_test () {
        let q = String::from("II");

        let q_score = QScore::analyze_qscores(q.as_bytes());

        assert_eq!(2, q_score.q_len);
        assert_eq!(40.0, q_score.mean_q);
    }

    #[test]
    fn decode_qscores_test() {
        let q = String::from("II");
        let qs = decode_qscores(q.as_bytes());

        let res = vec![40, 40];

        assert_eq!(res, qs);
    }
}

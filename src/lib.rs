use std::ops::{Mul, Sub};
use nalgebra::{Const, OMatrix, OVector, U1};

fn number_to_matrix_rep_mappings() -> Vec<OMatrix<f32, Const<7>, U1>> {
    vec![
        OVector::<f32, Const<7>>::from_vec(vec![1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0]),
        OVector::<f32, Const<7>>::from_vec(vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
        OVector::<f32, Const<7>>::from_vec(vec![1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0]),
        OVector::<f32, Const<7>>::from_vec(vec![1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0]),
        OVector::<f32, Const<7>>::from_vec(vec![0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0]),
        OVector::<f32, Const<7>>::from_vec(vec![1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 1.0]),
        OVector::<f32, Const<7>>::from_vec(vec![1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0]),
        OVector::<f32, Const<7>>::from_vec(vec![1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0]),
        OVector::<f32, Const<7>>::from_vec(vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
        OVector::<f32, Const<7>>::from_vec(vec![1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0]),
    ]
}


fn get_trial_inverse_decoder(m: &OMatrix<f32, Const::<7>, Const::<7>>) -> OMatrix<f32, Const::<7>, Const::<7>> {
    let mut d = OMatrix::<f32, Const::<7>, Const::<7>>::zeros();
    let mappings = number_to_matrix_rep_mappings();

    d.set_column(0, &mappings[0]);
    d.set_column(1, &mappings[1]);
    d.set_column(2, &mappings[3]);
    d.set_column(3, &mappings[4]);
    d.set_column(4, &mappings[6]);
    d.set_column(5, &mappings[7]);
    d.set_column(6, &mappings[9]);

    m.mul(d.try_inverse().unwrap())
}

fn to_col(s: &str) -> OVector<f32, Const::<7>> {
    let mut v = OVector::<f32, Const<7>>::from_element(0.0);

    for c in s.as_bytes() {
        v[(c - 'a' as u8) as usize] = 1.0;
    }

    v
}

pub fn decode(s: &str, decoder: &OMatrix<f32, Const::<7>, Const::<7>>) -> Result<i32, &'static str> {
    let c = to_col(s);

    let mappings = number_to_matrix_rep_mappings();

    for i in 0..mappings.len() {
        if decoder.mul(c).sub(&mappings[i]).norm() < 1e-6 {
            return Ok(i as i32);
        }
    }

    Err("No decoded value")
}

pub fn find_decoder(input: &str) -> Result<OMatrix<f32, Const<7>, Const<7>>, &'static str> {
    let tokens = input.split(" ");

    let mut m = OMatrix::<f32, Const<7>, Const<7>>::from_element(0.0);

    let mut five_lens = vec![];
    let mut six_lens = vec![];
    for token in tokens.clone() {
        match token.len() {
            2 => { m.set_column(1, &to_col(token)) }
            3 => { m.set_column(5, &to_col(token)) }
            4 => { m.set_column(3, &to_col(token)) }
            5 => five_lens.push(to_col(token)),
            6 => six_lens.push(to_col(token)),
            _ => {}
        }
    }


    for five_len in five_lens {
        m.set_column(2, &five_len);

        let permutations = vec![
            vec![0, 1, 2],
            vec![2, 0, 1],
            vec![1, 2, 0],
            vec![0, 2, 1],
            vec![1, 0, 2],
            vec![2, 1, 0],
        ];

        for p in permutations {
            m.set_column(0, &six_lens[p[0]]);
            m.set_column(4, &six_lens[p[1]]);
            m.set_column(6, &six_lens[p[2]]);

            let trial_inverse_decoder = get_trial_inverse_decoder(&m);

            let trial_decoder = trial_inverse_decoder.try_inverse().unwrap();


            let mut bad = false;
            for token in tokens.clone() {
                if decode(token, &trial_decoder).is_err() {
                    bad = true;
                    break;
                }
            }

            if !bad {
                return Ok(trial_decoder);
            }
        }
    }

    Err("No decoder found")
}

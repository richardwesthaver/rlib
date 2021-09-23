//! # quick maths
//! TODO: generics and Complex nums - nalgebra + ??

use std::cmp::Ordering;

fn partition(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
  match data.len() {
    0 => None,
    _ => {
      let (pivot_slice, tail) = data.split_at(1);
      let pivot = pivot_slice[0];
      let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
        {
          let (ref mut left, ref mut right) = &mut splits;
          if next < &pivot {
            left.push(*next);
          } else {
            right.push(*next);
          }
        }
        splits
      });

      Some((left, pivot, right))
    }
  }
}

fn select(data: &[i32], k: usize) -> Option<i32> {
  let part = partition(data);

  match part {
    None => None,
    Some((left, pivot, right)) => {
      let pivot_idx = left.len();

      match pivot_idx.cmp(&k) {
        Ordering::Equal => Some(pivot),
        Ordering::Greater => select(&left, k),
        Ordering::Less => select(&right, k - (pivot_idx + 1)),
      }
    }
  }
}

fn median(data: &[i32]) -> Option<f32> {
  let size = data.len();

  match size {
    even if even % 2 == 0 => {
      let fst_med = select(data, (even / 2) - 1);
      let snd_med = select(data, even / 2);

      match (fst_med, snd_med) {
        (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
        _ => None,
      }
    }
    odd => select(data, odd / 2).map(|x| x as f32),
  }
}

fn mean(data: &[i32]) -> Option<f32> {
  let sum = data.iter().sum::<i32>() as f32;
  let count = data.len();

  match count {
    positive if positive > 0 => Some(sum / count as f32),
    _ => None,
  }
}

fn std_deviation(data: &[i32]) -> Option<f32> {
  match (mean(data), data.len()) {
    (Some(data_mean), count) if count > 0 => {
      let variance = data
        .iter()
        .map(|value| {
          let diff = data_mean - (*value as f32);

          diff * diff
        })
        .sum::<f32>()
        / count as f32;

      Some(variance.sqrt())
    }
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}

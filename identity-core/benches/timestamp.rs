// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use identity_core::common::Timestamp;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;

fn deserialize_timestamp(input: &[u8]) -> Timestamp {
  Timestamp::from_json_slice(&input).unwrap()
}

pub fn deserialize_timestamp_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("Deserialize timestamp");

  for timestamp_string in std::iter::once("1937-01-01T12:00:27.87+00:20".to_owned()).chain(
    [-62167219200, 0, -9999, 9999, 253402300799]
      .into_iter()
      .map(|seconds| Timestamp::from_unix(seconds).unwrap().to_string()),
  ) {
    group.bench_with_input(timestamp_string.clone(), &timestamp_string, |b, input| {
      // Use iter_batched to avoid timing the `to_json_vec` call per iteration.
      b.iter_batched(
        || input.to_json_vec().unwrap(),
        |byte_vec| deserialize_timestamp(byte_vec.as_slice()),
        criterion::BatchSize::SmallInput,
      )
    });
  }
  group.finish();
}

criterion_group!(benches, deserialize_timestamp_benchmark);
criterion_main!(benches);

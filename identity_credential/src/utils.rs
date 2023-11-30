// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::common::Url;

pub(crate) fn url_only_includes_origin(url: &Url) -> bool {
  url.path() == "/" && url.query().is_none() && url.fragment().is_none()
}

#[cfg(test)]
mod tests {
  use super::url_only_includes_origin;
  use identity_core::common::Url;

  #[test]
  fn empty_path_and_root_are_valid_origins() {
    let urls = ["https://example.com", "https://example.com/"];
    for url in urls {
      assert!(url_only_includes_origin(&Url::parse(url).unwrap()));
    }
  }
}

use openfix_spec_generator::Builder;
use std::{env, fs};

fn main() {
    let builder = Builder::new();

    #[cfg(feature = "fix40")]
    let builder = builder.add_path("../protocol-spec/FIX40.xml");

    #[cfg(feature = "fix41")]
    let builder = builder.add_path("../protocol-spec/FIX41.xml");

    #[cfg(feature = "fix42")]
    let builder = builder.add_path("../protocol-spec/FIX42.xml");

    #[cfg(feature = "fix43")]
    let builder = builder.add_path("../protocol-spec/FIX43.xml");

    #[cfg(feature = "fix44")]
    let builder = builder.add_path("../protocol-spec/FIX44.xml");

    #[cfg(feature = "fixt11")]
    let builder = builder.add_path("../protocol-spec/FIXT11.xml");

    #[cfg(feature = "test_spec")]
    let builder = builder
        .add_path("../protocol-spec/TEST_SPEC.xml")
        .add_path("../protocol-spec/TEST_SPEC_SIG.xml");

    let builder = builder.enable_rustfmt(true);

    fs::create_dir_all("./out-preview").unwrap();
    builder.build("./out-preview").unwrap();
    builder.build(&env::var("OUT_DIR").unwrap()).unwrap();
}

use bindgen::CargoCallbacks;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let c_src_path = Path::new("speex-1.2.1/libspeex")
        .canonicalize()
        .expect("Failed to canonicalize");
    let paths = [
        "bits.c",
        "cb_search.c",
        "exc_5_64_table.c",
        "exc_5_256_table.c",
        "exc_8_128_table.c",
        "exc_10_16_table.c",
        "exc_10_32_table.c",
        "exc_20_32_table.c",
        "filters.c",
        "gain_table.c",
        "gain_table_lbr.c",
        "hexc_10_32_table.c",
        "hexc_table.c",
        "high_lsp_tables.c",
        "kiss_fftr.c",
        "lpc.c",
        "lsp.c",
        "lsp_tables_nb.c",
        "ltp.c",
        "modes.c",
        "modes_wb.c",
        "nb_celp.c",
        "quant_lsp.c",
        "sb_celp.c",
        "smallft.c",
        "speex.c",
        "speex_callbacks.c",
        "speex_header.c",
        "stereo.c",
        "vbr.c",
        "vorbis_psy.c",
        "vq.c",
        "window.c",
    ];
    let mut ccomp = cc::Build::new();

    for path in paths {
        ccomp.file(c_src_path.join(path));
    }

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:warning={:?}", dst.canonicalize().unwrap());

    ccomp.out_dir(dst.join("lib"));
    ccomp.compile("speex");

    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Failed to write bindings");
}

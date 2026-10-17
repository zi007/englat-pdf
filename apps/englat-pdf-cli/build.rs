//! Windows only: embed version info (VERSIONINFO) into `englat-pdf-cli.exe`, so Explorer's
//! Details tab, `(Get-Item englat-pdf-cli.exe).VersionInfo` and inventory tools can read the version
//! and publisher, as they can for `englat-pdf.exe` (`apps/englat-pdf/build.rs`).
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `ENGLAT_PDF_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=ENGLAT_PDF_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "ENGLAT PDF")
        .set("FileDescription", "ENGLAT PDF command-line tool")
        .set("LegalCopyright", "Copyright (c) the PdfCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "englat-pdf-cli.exe")
        .set("InternalName", "englat-pdf-cli");
    if let Err(e) = res.compile() {
        if std::env::var_os("ENGLAT_PDF_REQUIRE_WINRES").is_some() {
            println!("cargo::error=embedding Windows resources failed: {e}");
            return;
        }
        println!("cargo:warning=englat-pdf-cli.exe built without version resources: {e}");
    }
}

extern crate bindgen;

use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let zrtp_includes = Path::new("vendor/zrtp/");
    let common_includes = Path::new("vendor/");
    let crypto_includes = Path::new("vendor/zrtp/crypto");

    cc::Build::new()
    	.cpp(true)
        .flag("-std=c++11")
        .flag("-Wno-unknown-pragmas")
        .include(zrtp_includes)
        .include(common_includes)
        .include(crypto_includes)
        .file("vendor/zrtp/ZrtpCallbackWrapper.cpp")
        .file("vendor/zrtp/ZRtp.cpp")
        .file("vendor/zrtp/ZrtpCrc32.cpp")
        .file("vendor/zrtp/ZrtpPacketCommit.cpp")
        .file("vendor/zrtp/ZrtpPacketConf2Ack.cpp")
        .file("vendor/zrtp/ZrtpPacketConfirm.cpp")
        .file("vendor/zrtp/ZrtpPacketDHPart.cpp")
        .file("vendor/zrtp/ZrtpPacketGoClear.cpp")
        .file("vendor/zrtp/ZrtpPacketClearAck.cpp")
        .file("vendor/zrtp/ZrtpPacketHelloAck.cpp")
        .file("vendor/zrtp/ZrtpPacketHello.cpp")
        .file("vendor/zrtp/ZrtpPacketError.cpp")
        .file("vendor/zrtp/ZrtpPacketErrorAck.cpp")
        .file("vendor/zrtp/ZrtpPacketPingAck.cpp")
        .file("vendor/zrtp/ZrtpPacketPing.cpp")
        .file("vendor/zrtp/ZrtpPacketSASrelay.cpp")
        .file("vendor/zrtp/ZrtpPacketRelayAck.cpp")
        .file("vendor/zrtp/ZrtpStateClass.cpp")
        .file("vendor/zrtp/ZrtpTextData.cpp")
        .file("vendor/zrtp/ZrtpConfigure.cpp")
        .file("vendor/zrtp/ZrtpCWrapper.cpp")
        .file("vendor/zrtp/Base32.cpp")
        .file("vendor/zrtp/EmojiBase32.cpp")
        .file("vendor/zrtp/crypto/skeinMac256.cpp")
        .file("vendor/zrtp/crypto/skein256.cpp")
        .file("vendor/zrtp/crypto/skeinMac384.cpp")
        .file("vendor/zrtp/crypto/skein384.cpp")
        .file("vendor/zrtp/ZIDCacheFile.cpp")
        .file("vendor/zrtp/ZIDRecordFile.cpp")
        .compile("zrtp");

    println!("cargo:rerun-if-changed=zrtp.hpp");

    // generate the bindings for zrtp headers
    let bindings = bindgen::Builder::default()
        .clang_arg("-Ivendor/")
        .clang_arg("-Ivendor/zrtp/")
        .clang_arg("-Ivendor/zrtp/crypto")
        .header("zrtp.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate zrtp bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("zrtp.rs"))
        .expect("Couldn't write zrtp bindings!");
}
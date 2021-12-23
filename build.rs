extern crate bindgen;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut clang_flags = Vec::<String>::new();

    let zrtp_includes = Path::new("vendor/zrtp/");
    let common_includes = Path::new("vendor/");
    let crypto_includes = Path::new("vendor/zrtp/crypto/");
    let bn_crypto_includes = Path::new("vendor/bnlib/");
    let common_crypto_includes = Path::new("vendor/cryptcommon/");

    if target == "wasm32-unknown-emscripten" {
        // add a typedef for uint
        let zrtp_cpp = std::fs::read_to_string("vendor/zrtp/ZRtp.cpp").unwrap();
        let updated_zrtp_cpp = zrtp_cpp.replace(
            "using namespace GnuZrtpCodes;",
            "using namespace GnuZrtpCodes;\n\n#ifndef uint\n#define uint unsigned int\n#endif",
        );

        let mut zrtp_cpp_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("vendor/zrtp/ZRtp.cpp")
            .unwrap();

        zrtp_cpp_file.write(updated_zrtp_cpp.as_bytes()).unwrap();

        clang_flags.push(String::from("-fvisibility=default"));
    }

    cc::Build::new()
        .warnings(false)
        .include(zrtp_includes)
        .include(common_includes)
        .include(crypto_includes)
        .include(bn_crypto_includes)
        .include(common_crypto_includes)
        .file("vendor/common/osSpecifics.c")
        .file("vendor/common/icuUtf8.c")
        .file("vendor/zrtp/zrtpB64Decode.c")
        .file("vendor/zrtp/zrtpB64Encode.c")
        .file("vendor/cryptcommon/aeskey.c")
        .file("vendor/cryptcommon/twofish.c")
        .file("vendor/cryptcommon/aes_modes.c")
        .file("vendor/cryptcommon/skeinApi.c")
        .file("vendor/cryptcommon/twofish_cfb.c")
        .file("vendor/cryptcommon/skein_block.c")
        .file("vendor/cryptcommon/aescrypt.c")
        .file("vendor/cryptcommon/aestab.c")
        .file("vendor/cryptcommon/skein.c")
        .file("vendor/srtp/crypto/sha1.c")
        .file("vendor/zrtp/crypto/sha2.c")
        .file("vendor/bnlib/bn00.c")
        .file("vendor/bnlib/lbn00.c")
        .file("vendor/bnlib/bn.c")
        .file("vendor/bnlib/lbnmem.c")
        .file("vendor/bnlib/sieve.c")
        .file("vendor/bnlib/prime.c")
        .file("vendor/bnlib/bnprint.c")
        .file("vendor/bnlib/jacobi.c")
        .file("vendor/bnlib/germain.c")
        .file("vendor/bnlib/ec/ec.c")
        .file("vendor/bnlib/ec/ecdh.c")
        .file("vendor/bnlib/ec/curve25519-donna.c")
        .compile("zrtpcrypto");

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .warnings(false)
        .include(zrtp_includes)
        .include(common_includes)
        .include(crypto_includes)
        .include(bn_crypto_includes)
        .include(common_crypto_includes)
        .define("CORE_LIB", Some("true"))
        .file("vendor/cryptcommon/macSkein.cpp")
        .file("vendor/cryptcommon/ZrtpRandom.cpp")
        .file("vendor/zrtp/Base32.cpp")
        .file("vendor/zrtp/crypto/aesCFB.cpp")
        .file("vendor/zrtp/crypto/hmac256.cpp")
        .file("vendor/zrtp/crypto/hmac384.cpp")
        .file("vendor/zrtp/crypto/sha256.cpp")
        .file("vendor/zrtp/crypto/sha384.cpp")
        .file("vendor/zrtp/crypto/skein256.cpp")
        .file("vendor/zrtp/crypto/skein384.cpp")
        .file("vendor/zrtp/crypto/skeinMac256.cpp")
        .file("vendor/zrtp/crypto/skeinMac384.cpp")
        .file("vendor/zrtp/crypto/twoCFB.cpp")
        .file("vendor/zrtp/crypto/zrtpDH.cpp")
        .file("vendor/zrtp/EmojiBase32.cpp")
        .file("vendor/zrtp/ZIDCacheFile.cpp")
        .file("vendor/zrtp/ZIDRecordFile.cpp")
        .file("vendor/zrtp/ZRtp.cpp")
        .file("vendor/zrtp/ZrtpCallbackWrapper.cpp")
        .file("vendor/zrtp/ZrtpConfigure.cpp")
        .file("vendor/zrtp/ZrtpCrc32.cpp")
        .file("vendor/zrtp/ZrtpCWrapper.cpp")
        .file("vendor/zrtp/ZrtpPacketClearAck.cpp")
        .file("vendor/zrtp/ZrtpPacketCommit.cpp")
        .file("vendor/zrtp/ZrtpPacketConf2Ack.cpp")
        .file("vendor/zrtp/ZrtpPacketConfirm.cpp")
        .file("vendor/zrtp/ZrtpPacketDHPart.cpp")
        .file("vendor/zrtp/ZrtpPacketError.cpp")
        .file("vendor/zrtp/ZrtpPacketErrorAck.cpp")
        .file("vendor/zrtp/ZrtpPacketGoClear.cpp")
        .file("vendor/zrtp/ZrtpPacketHello.cpp")
        .file("vendor/zrtp/ZrtpPacketHelloAck.cpp")
        .file("vendor/zrtp/ZrtpPacketPing.cpp")
        .file("vendor/zrtp/ZrtpPacketPingAck.cpp")
        .file("vendor/zrtp/ZrtpPacketRelayAck.cpp")
        .file("vendor/zrtp/ZrtpPacketSASrelay.cpp")
        .file("vendor/zrtp/ZrtpStateClass.cpp")
        .file("vendor/zrtp/ZrtpTextData.cpp")
        .compile("zrtp");

    println!("cargo:rerun-if-changed=zrtp.h");

    // generate the bindings for zrtp headers
    let mut builder = bindgen::Builder::default();

    for value in &clang_flags {
        builder = builder.clang_arg(value);
    }

    let bindings = builder
        .clang_arg("-Ivendor/")
        .clang_arg("-Ivendor/zrtp/")
        .clang_arg("-Ivendor/zrtp/crypto/")
        .clang_arg("-Ivendor/cryptcommon/")
        .clang_arg("-Ivendor/bnlib/")
        .allowlist_type(r"zrtp.*")
        .allowlist_type(r"Zrtp.*")
        .allowlist_type(r"ZRTP.*")
        .allowlist_type(r"C_Srtp.*")
        .allowlist_function(r"zrtp.*")
        .allowlist_function(r"Zrtp.*")
        .allowlist_function(r"ZRTP.*")
        .allowlist_var(r"zrtp.*")
        .allowlist_var(r"Zrtp.*")
        .allowlist_var(r"ZRTP.*")
        .header("zrtp.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate zrtp bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // output the bindings
    bindings
        .write_to_file(out_path.join("zrtp.rs"))
        .expect("Couldn't write zrtp bindings!");
}

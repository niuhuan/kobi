pub const UA: &str = "github actions";

pub fn asset_name(app_name: &str, code: &str, target: &str) -> String {
    match target {
        "macos" => format!("{app_name}-{code}.dmg"),
        "ios" => format!("{app_name}-{code}-nosign.ipa"),
        "windows" => format!("{app_name}-{code}-windows-x86_64.zip"),
        "linux" => format!("{app_name}-{code}-linux-x86_64.AppImage"),
        "android-arm32" => format!("{app_name}-{code}-arm32.apk"),
        "android-arm64" => format!("{app_name}-{code}-arm64.apk"),
        "android-x86_64" => format!("{app_name}-{code}-x86_64.apk"),
        un => panic!("unknown target : {un}"),
    }
}

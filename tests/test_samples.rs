use std::fs;
use test_case::test_case;

use sdplugin_manifest;

macro_rules! test_case_path {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/", $fname) // assumes Linux ('/')!
    };
}

#[test_case(test_case_path!("streamdeck-counter.json"))]
#[test_case(test_case_path!("streamdeck-cpu.json"))]
#[test_case(test_case_path!("streamdeck-memorygame.json"))]
#[test_case(test_case_path!("streamdeck-numberdisplay.json"))]
#[test_case(test_case_path!("streamdeck-philipshue.json"))]
#[test_case(test_case_path!("streamdeck-pisamples.json"))]
fn test_read_manifest(manifest_path: &str) {
    read_manifest(manifest_path);
}

#[test_case(test_case_path!("streamdeck-counter.json"), "com.elgato.counter")]
#[test_case(test_case_path!("streamdeck-cpu.json"), "com.elgato.cpu")]
#[test_case(test_case_path!("streamdeck-memorygame.json"), "com.elgato.memorygame")]
#[test_case(test_case_path!("streamdeck-numberdisplay.json"), "com.elgato.numberdisplay")]
#[test_case(test_case_path!("streamdeck-philipshue.json"), "com.elgato.philips-hue")]
#[test_case(test_case_path!("streamdeck-pisamples.json"), "com.elgato.pisamples")]
fn test_uuid(manifest_path: &str, expected_uuid: &str) {
    let manifest = read_manifest(manifest_path);
    assert_eq!(manifest.uuid().unwrap(), expected_uuid);
}

fn read_manifest(manifest_path: &str) -> sdplugin_manifest::Manifest {
    let manifest_data = fs::read_to_string(manifest_path).unwrap();
    serde_json::from_str::<sdplugin_manifest::Manifest>(&manifest_data).unwrap()
}

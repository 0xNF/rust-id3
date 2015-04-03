extern crate id3;

use id3::{Tag, Frame};
use id3::frame::Encoding;

static ID: &'static str = "TRCK";
static TRACK: u32 = 5;
static TOTAL: u32 = 10;
static INVALID: &'static str = "invalid";

// UTF8 {{{
#[test]
fn utf8() {
    let mut tag = Tag::with_version(4);

    tag.set_track_enc(TRACK, Encoding::UTF8);
    tag.set_total_tracks_enc(TOTAL, Encoding::UTF8);
    let frame = tag.get(ID).unwrap();

    assert_eq!(tag.track().unwrap(), TRACK);
    assert_eq!(tag.total_tracks().unwrap(), TOTAL);
    assert_eq!(&frame.content.text()[..], &format!("{}/{}", TRACK, TOTAL)[..]);

    let mut data: Vec<u8> = Vec::new();
    data.push(Encoding::UTF8 as u8);
    data.extend(format!("{}/{}", TRACK, TOTAL).into_bytes().into_iter());
    assert_eq!(frame.content_to_bytes(4), data);
}

#[test]
fn utf8_only_track() {
    let mut tag = Tag::with_version(4);

    tag.set_track_enc(TRACK, Encoding::UTF8);
    let frame = tag.get(ID).unwrap();

    assert_eq!(tag.track().unwrap(), TRACK);
    assert!(tag.total_tracks().is_none());
    assert_eq!(&frame.text().unwrap()[..], &format!("{}", TRACK)[..]);
    assert_eq!(&frame.content.text()[..], &format!("{}", TRACK)[..]);

    let mut data: Vec<u8> = Vec::new();
    data.push(Encoding::UTF8 as u8);
    data.extend(format!("{}", TRACK).into_bytes().into_iter());
    assert_eq!(frame.content_to_bytes(4), data);
}

#[test]
fn utf8_invalid() {
    let mut tag = Tag::with_version(4);
    
    let mut frame = Frame::new(ID);
    let mut data = Vec::new();
    data.push(Encoding::UTF8 as u8);
    data.extend(format!("{}/{}", INVALID, TOTAL).into_bytes().into_iter());
    frame.parse_data(&data[..]).unwrap();
    tag.push(frame);
    assert!(tag.track().is_none());
    assert!(tag.total_tracks().is_none());

    tag.remove(ID);

    let mut frame = Frame::new(ID);
    let mut data = Vec::new();
    data.push(Encoding::UTF8 as u8);
    data.extend(format!("{}/{}", TRACK, INVALID).into_bytes().into_iter());
    frame.parse_data(&data[..]).unwrap();
    tag.push(frame);
    assert!(tag.track().is_none());
    assert!(tag.total_tracks().is_none());
}
//}}}

// UTF16 {{{
#[test]
fn utf16() {
    let mut tag = Tag::with_version(4);

    tag.set_track_enc(TRACK, Encoding::UTF16);
    tag.set_total_tracks_enc(TOTAL, Encoding::UTF16);
    let frame = tag.get(ID).unwrap();

    assert_eq!(tag.track().unwrap(), TRACK);
    assert_eq!(tag.total_tracks().unwrap(), TOTAL);
    assert_eq!(&frame.content.text()[..], &format!("{}/{}", TRACK, TOTAL)[..]);

    let mut data = Vec::new();
    data.push(Encoding::UTF16 as u8);
    data.extend(id3::util::string_to_utf16(&format!("{}/{}", TRACK, TOTAL)[..]).into_iter());
    assert_eq!(frame.content_to_bytes(4), data);
}

#[test]
fn utf16_only_track() {
    let mut tag = Tag::with_version(4);

    tag.set_track_enc(TRACK, Encoding::UTF16);
    let frame = tag.get(ID).unwrap();

    assert_eq!(tag.track().unwrap(), TRACK);
    assert!(tag.total_tracks().is_none());
    assert_eq!(&frame.content.text()[..], &format!("{}", TRACK)[..]);

    let mut data: Vec<u8> = Vec::new();
    data.push(Encoding::UTF16 as u8);
    data.extend(id3::util::string_to_utf16(&format!("{}", TRACK)[..]).into_iter());
    assert_eq!(frame.content_to_bytes(4), data);
}

#[test]
fn utf16_invalid() {
    let mut tag = Tag::with_version(4);
    
    let mut frame = Frame::new(ID);
    let mut data = Vec::new();
    data.push(Encoding::UTF16 as u8);
    data.extend(id3::util::string_to_utf16(&format!("{}/{}", INVALID, TOTAL)[..]).into_iter());
    frame.parse_data(&data[..]).unwrap();
    tag.push(frame);
    assert!(tag.track().is_none());
    assert!(tag.total_tracks().is_none());

    tag.remove(ID);

    let mut frame = Frame::new(ID);
    let mut data = Vec::new();
    data.push(Encoding::UTF16 as u8);
    data.extend(id3::util::string_to_utf16(&format!("{}/{}", TRACK, INVALID)[..]).into_iter());
    frame.parse_data(&data[..]).unwrap();
    tag.push(frame);
    assert!(tag.track().is_none());
    assert!(tag.total_tracks().is_none());
}
//}}}

// UTF16BE {{{
#[test]
fn utf16be() {
    let mut tag = Tag::with_version(4);

    tag.set_track_enc(TRACK, Encoding::UTF16BE);
    tag.set_total_tracks_enc(TOTAL, Encoding::UTF16BE);
    let frame = tag.get(ID).unwrap();

    assert_eq!(tag.track().unwrap(), TRACK);
    assert_eq!(tag.total_tracks().unwrap(), TOTAL);
    assert_eq!(&frame.content.text()[..], &format!("{}/{}", TRACK, TOTAL)[..]);

    let mut data: Vec<u8> = Vec::new();
    data.push(Encoding::UTF16BE as u8);
    data.extend(id3::util::string_to_utf16be(&format!("{}/{}", TRACK, TOTAL)[..]).into_iter());
    assert_eq!(frame.content_to_bytes(4), data);
}

#[test]
fn utf16be_only_track() {
    let mut tag = Tag::with_version(4);

    tag.set_track_enc(TRACK, Encoding::UTF16BE);
    let frame = tag.get(ID).unwrap();

    assert_eq!(tag.track().unwrap(), TRACK);
    assert!(tag.total_tracks().is_none());
    assert_eq!(&frame.content.text()[..], &format!("{}", TRACK)[..]);

    let mut data: Vec<u8> = Vec::new();
    data.push(Encoding::UTF16BE as u8);
    data.extend(id3::util::string_to_utf16be(&format!("{}", TRACK)[..]).into_iter());
    assert_eq!(frame.content_to_bytes(4), data);
}

#[test]
fn utf16be_invalid() {
    let mut tag = Tag::with_version(4);
    
    let mut frame = Frame::new(ID);
    let mut data = Vec::new();
    data.push(Encoding::UTF16BE as u8);
    data.extend(id3::util::string_to_utf16be(&format!("{}/{}", INVALID, TOTAL)[..]).into_iter());
    frame.parse_data(&data[..]).unwrap();
    tag.push(frame);
    assert!(tag.track().is_none());
    assert!(tag.total_tracks().is_none());

    tag.remove(ID);

    let mut frame = Frame::new(ID);
    let mut data = Vec::new();
    data.push(Encoding::UTF16BE as u8);
    data.extend(id3::util::string_to_utf16be(&format!("{}/{}", TRACK, INVALID)[..]).into_iter());
    frame.parse_data(&data[..]).unwrap();
    tag.push(frame);
    assert!(tag.track().is_none());
    assert!(tag.total_tracks().is_none());
}
//}}}

use std::sync::Arc;

use id3::Timestamp;

use crate::*;

#[derive(Default)]
pub struct AnyTag<'a> {
    pub config: Config,
    pub musicbrainz: MusicBrainz,
    pub title: Option<&'a str>,
    pub artists: Option<Vec<&'a str>>,
    pub date: Option<Timestamp>,
    pub year: Option<i32>,
    pub duration: Option<f64>,
    pub album_title: Option<&'a str>,
    pub album_artists: Option<Vec<&'a str>>,
    pub album_cover: Option<Picture<'a>>,
    pub track_number: Option<u16>,
    pub total_tracks: Option<u16>,
    pub disc_number: Option<u16>,
    pub total_discs: Option<u16>,
    pub genre: Option<&'a str>,
    pub composer: Option<&'a str>,
    pub comment: Option<&'a str>,

    // MusicBrainz
    pub acoust_id: Option<Arc<str>>,
    /*
    pub musicbrainz_artist_id: Option<&'a str>,
    pub musicbrainz_recording_id: Option<&'a str>,
    pub musicbrainz_release_artist_id: Option<&'a str>,
    pub musicbrainz_release_group_id: Option<&'a str>,
    pub musicbrainz_release_id: Option<&'a str>,
    pub musicbrainz_track_id: Option<&'a str>,
     */
}

impl AudioTagConfig for AnyTag<'_> {
    fn config(&self) -> &Config {
        &self.config
    }
    fn set_config(&mut self, config: Config) {
        self.config = config;
    }
}

/*
impl MusicBrainzTagConfig for AnyTag<'_> {
    fn musicbrainz(&self) -> &MusicBrainz {
        &self.musicbrainz
    }
    fn set_musicbrainz(&mut self, musicbrainz: MusicBrainz) {
        self.musicbrainz = musicbrainz.clone();
    }
}
 */

impl<'a> AnyTag<'a> {
    pub fn title(&self) -> Option<&str> {
        self.title
    }
    pub fn set_title(&mut self, title: &'a str) {
        self.title = Some(title);
    }
    pub fn artists(&self) -> Option<&[&str]> {
        self.artists.as_deref()
    }
    // set_artists; add_artist
    pub fn date(&self) -> Option<Timestamp> {
        self.date
    }
    pub fn set_date(&mut self, date: Timestamp) {
        self.date = Some(date);
    }
    pub fn year(&self) -> Option<i32> {
        self.year
    }
    pub fn set_year(&mut self, year: i32) {
        self.year = Some(year);
    }
    pub fn duration(&self) -> Option<f64> {
        self.duration
    }
    pub fn album_title(&self) -> Option<&str> {
        self.album_title
    }
    pub fn album_artists(&self) -> Option<&[&str]> {
        self.album_artists.as_deref()
    }
    pub fn track_number(&self) -> Option<u16> {
        self.track_number
    }
    pub fn total_tracks(&self) -> Option<u16> {
        self.total_tracks
    }
    pub fn disc_number(&self) -> Option<u16> {
        self.disc_number
    }
    pub fn total_discs(&self) -> Option<u16> {
        self.total_discs
    }
    pub fn genre(&self) -> Option<&str> {
        self.genre
    }
    pub fn composer(&self) -> Option<&str> {
        self.composer
    }
    pub fn comment(&self) -> Option<&str> {
        self.comment
    }
}

impl AnyTag<'_> {
    pub fn artists_as_string(&self) -> Option<String> {
        self.artists()
            .map(|artists| artists.join(self.config.sep_artist))
    }
    pub fn album_artists_as_string(&self) -> Option<String> {
        self.album_artists()
            .map(|artists| artists.join(self.config.sep_artist))
    }
}

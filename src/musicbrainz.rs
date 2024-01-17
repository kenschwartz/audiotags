#[derive(Debug, Clone, Copy)]
pub struct MusicBrainz<'a> {
    pub acoust_id: String,
    pub musicbrainz_artist_id: String,
    pub musicbrainz_album_id: String,
    pub musicbrainz_track_id: String,
}

impl Default for MusicBrainz {
    fn default() -> Self {
        Self {
            acoust_id: String::default(),
            musicbrainz_artist_id: String::default(),
            musicbrainz_album_id: String::default(),
            musicbrainz_track_id: String::default(),
        }
    }

    fn new() {
        todo!()
    }
}

impl MusicBrainz {
    /*
    fn new(acoust_id: &str, artist: &str, album: &str, track: &str) -> Self {
        Self {
            acoust_id: acoust_id.to_string(),
            musicbrainz_artist_id: artist.to_string(),
            musicbrainz_album_id: album.to_string(),
            musicbrainz_track_id: track.to_string(),
        }
    }
     */

    pub fn acoust_id(&self) -> &str {
        self.acoust_id.as_str()
    }
    pub fn set_acoust_id(&mut self, id: &str) -> Self {
        self.acoust_id = id.to_owned();
        self.clone()
    }

    pub fn musicbrainz_artist_id(&self) -> &str {
        self.musicbrainz_artist_id.as_str()
    }
    pub fn set_musicbrainz_artist_id(&mut self, id: &str) -> Self {
        self.musicbrainz_artist_id = id.to_owned();
        self.clone()
    }
    pub fn musicbrainz_album_id(&self) -> &str {
        self.musicbrainz_album_id.as_str()
    }
    pub fn set_musicbrainz_album_id(&mut self, id: &str) -> Self {
        self.musicbrainz_album_id = id.to_owned();
        self.clone()
    }
    pub fn musicbrainz_track_id(&self) -> &str {
        self.musicbrainz_track_id.as_str()
    }
    pub fn set_musicbrainz_track_id(&mut self, id: &str) -> Self {
        self.musicbrainz_track_id = id.to_owned();
        self.clone()
    }
}
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct MusicBrainz {
    pub acoust_id: RwLock<Arc<str>>,
    pub musicbrainz_artist_id: RwLock<Arc<str>>,
    pub musicbrainz_album_id: RwLock<Arc<str>>,
    pub musicbrainz_track_id: RwLock<Arc<str>>,
}

impl Default for MusicBrainz {
    fn default() -> Self {
        Self {
            acoust_id: RwLock::new(Arc::from(String::new())),
            musicbrainz_artist_id: RwLock::new(Arc::from(String::new())),
            musicbrainz_album_id: RwLock::new(Arc::from(String::new())),
            musicbrainz_track_id: RwLock::new(Arc::from(String::new())),
        }
    }
}

impl MusicBrainz {
    pub fn acoust_id(&self) -> Arc<str> {
        let read_lock = self
            .acoust_id
            .read()
            .expect("acoust_id: error getting read");
        read_lock.clone()
    }
    pub fn set_acoust_id(&self, id: &String) {
        let mut write_lock = self
            .acoust_id
            .write()
            .expect("acoust_id: error getting write");
        *write_lock = Arc::from(id.as_str());
    }

    pub fn musicbrainz_artist_id(&self) -> Arc<str> {
        let read_lock = self
            .musicbrainz_artist_id
            .read()
            .expect("musicbrainz_artist_id: error getting read");
        read_lock.clone()
    }
    pub fn set_musicbrainz_artist_id(&self, id: &String) {
        let mut write_lock = self
            .musicbrainz_artist_id
            .write()
            .expect("musicbrainz_artist_id: error getting write");
        *write_lock = Arc::from(id.as_str());
    }
    pub fn musicbrainz_album_id(&self) -> Arc<str> {
        let read_lock = self
            .musicbrainz_album_id
            .read()
            .expect("musicbrainz_album_id: error getting read");
        read_lock.clone()
    }
    pub fn set_musicbrainz_album_id(&self, id: &String) {
        let mut write_lock = self
            .musicbrainz_album_id
            .write()
            .expect("musicbrainz_album_id: error getting write");
        *write_lock = Arc::from(id.as_str());
    }
    pub fn musicbrainz_track_id(&self) -> Arc<str> {
        let read_lock = self
            .musicbrainz_track_id
            .read()
            .expect("musicbrainz_track_id: error getting read");
        read_lock.clone()
    }
    pub fn set_musicbrainz_track_id(&self, id: &String) {
        let mut write_lock = self
            .musicbrainz_track_id
            .write()
            .expect("musicbrainz_track_id: error getting write");
        *write_lock = Arc::from(id.as_str());
    }
}

impl Clone for MusicBrainz {
    fn clone(&self) -> Self {
        MusicBrainz {
            acoust_id: RwLock::new(Arc::from(self.acoust_id().to_string().as_str())),
            musicbrainz_artist_id: RwLock::new(Arc::from(self.musicbrainz_artist_id().to_string().as_str())),
            musicbrainz_album_id: RwLock::new(Arc::from(self.musicbrainz_album_id().to_string().as_str())),
            musicbrainz_track_id: RwLock::new(Arc::from(self.musicbrainz_track_id().to_string().as_str())),
        }
    }
}
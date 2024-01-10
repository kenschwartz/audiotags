use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct MusicBrainz {
    pub acoust_id: RwLock<Arc<str>>,
    pub musicbrainz_id: RwLock<Arc<str>>,
}

impl Default for MusicBrainz {
    fn default() -> Self {
        Self {
            acoust_id: RwLock::new(Arc::from(String::new())),
            musicbrainz_id: RwLock::new(Arc::from(String::new())),
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
    pub fn set_acoust_id(&self, id: String) {
        let mut write_lock = self
            .acoust_id
            .write()
            .expect("acoust_id: error getting write");
        *write_lock = Arc::from(id.as_str());
    }

    pub fn musicbrainz_id(&self) -> Arc<str> {
        let read_lock = self
            .musicbrainz_id
            .read()
            .expect("musicbrainz_id: error getting read");
        read_lock.clone()
    }
    pub fn set_musicbrainz_id(&self, id: String) -> Arc<str> {
        let mut write_lock = self
            .musicbrainz_id
            .write()
            .expect("musicbrainz_id: error getting write");
        *write_lock = Arc::from(id.as_str());
        self.musicbrainz_id()
    }
}

impl Clone for MusicBrainz {
    fn clone(&self) -> Self {
        MusicBrainz {
            acoust_id: RwLock::new(Arc::from(self.acoust_id().to_string().as_str())),
            musicbrainz_id: RwLock::new(Arc::from(self.musicbrainz_id().to_string().as_str())),
        }
    }
}
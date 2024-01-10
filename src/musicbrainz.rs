use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct MusicBrainz {
    pub acoust_id: RwLock<Option<Arc<str>>>,
    pub musicbrainz_id: RwLock<Option<Arc<str>>>,
}

impl Default for MusicBrainz {
    fn default() -> Self {
        Self {
            acoust_id: RwLock::new(None),
            musicbrainz_id: RwLock::new(None),
        }
    }
}

impl MusicBrainz {
    pub fn acoust_id(&self) -> Option<Arc<str>> {
        let read_lock = self
            .acoust_id
            .read()
            .expect("acoust_id: error getting read");
        if read_lock.is_none() {
            return None;
        }
        let x = match Option::as_ref(&read_lock) {
            Some(x) => Some(Arc::clone(x)),
            None => None,
        };
        return x;
        //Some(Arc::clone(x))
    }
    pub fn set_acoust_id(&self, aid: String)  {
        let mut write_lock = self
            .acoust_id
            .write()
            .expect("acoust_id: error getting write");
        if write_lock.is_none() {
            *write_lock = Some(Arc::from(aid.as_str()))
        } else {
            println!("acoust_id is already set!");
        }
    }

    pub fn musicbrainz_id(&self) -> Option<Arc<str>> {
        let read_lock = self
            .musicbrainz_id
            .read()
            .expect("musicbrainz_id: error getting read");
        if read_lock.is_none() {
            return None;
        }
        let x = match Option::as_ref(&read_lock) {
            Some(x) => Some(Arc::clone(x)),
            None => None,
        };
        return x;
    }
    pub fn set_musicbrainz_id(&self, aid: String) {
        let mut write_lock = self
            .musicbrainz_id
            .write()
            .expect("musicbrainz_id: error getting write");
        if write_lock.is_none() {
            *write_lock = Some(Arc::from(aid.as_str()))
        } else {
            println!("musicbrainz_id is already set!");
        }
    }
}

impl Clone for MusicBrainz {
    fn clone(&self) -> Self {
        MusicBrainz {
            acoust_id: match self.acoust_id() {
                None => RwLock::new(None),
                Some(x) => RwLock::new(Some(x)),
            },
            musicbrainz_id: match self.musicbrainz_id() {
                None => RwLock::new(None),
                Some(x) => RwLock::new(Some(x)),
            },
        }
    }
}
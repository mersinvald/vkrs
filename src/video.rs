use std::borrow::Borrow;
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use api::{Bool, Collection, Duration, FullId, Id, LikesCount, OwnerId, Sort, Timestamp};

#[cfg(feature = "unstable")]
include!("video.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/video.rs"));

request_ref! {
    #[derive(Eq, Copy)]
    struct Get for ["video.get"](v => 5.44) -> Collection<Video> [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            album_id: Option<Id> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            videos: [FullId] = (&[][..]) => { |value|
                &*value.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(",")
            }
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Search for ["video.search"](v => 5.44) -> Collection<Video> [Video] {
        sized {
            sort: Sort = (Sort::Popularity) => {AsRef},
            hd: bool = () => {bool},
            adult: bool = () => {bool},
            search_own: bool = () => {bool},
            longer: Option<usize> = () => {Option},
            shorter: Option<usize> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            filters: [Filter] = (&[][..]) => {AsRef<Vec>},
            q: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Edit for ["video.edit"](v => 5.44) -> Bool [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            video_id: Id = () => {},
            no_comments: bool = () => {bool},
            repeat: bool = () => {bool},
        }
        unsized {
            name: str = ("") => {=},
            desc: str = ("") => {=},
            privacy_view: str = ("") => {=},
            privacy_comment: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Add for ["video.add"](v => 5.44) -> Bool [Video] {
        target_id: OwnerId = () => {},
        video_id: Id = () => {},
        owner_id: OwnerId = () => {},
    }
}

enum_str! { Filter {
    YouTube = "youtube",
    Vimeo = "vimeo",
    Short = "short",
    Long = "long",
}}

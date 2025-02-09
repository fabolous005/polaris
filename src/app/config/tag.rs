use crate::app::Error;
use super::storage;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tag {
    pub show: bool,
    pub weight: bool,
    pub collection: bool,
}

impl TryFrom<storage::Tag> for Tag {
    type Error = Error;

    fn try_from(tag: storage::Tag) -> Result<Self, Self::Error> {
        Ok(Self {
            show: tag.show.unwrap_or(false),
            weight: tag.weight.unwrap_or(false),
            collection: tag.collection.unwrap_or(false),
        })
    }
}

// impl From<storage::Tag> for Tag {
//     fn from(tag: storage::Tag) -> Self {
//         Self {
//             show: tag.show.unwrap_or(false),
//             weight: tag.weight.unwrap_or(false),
//             collection: tag.collection.unwrap_or(false),
//         }
//     }
// }

impl From<Tag> for storage::Tag {
    fn from(tag: Tag) -> Self {
        Self {
            show: Some(tag.show),
            weight: Some(tag.weight),
            collection: Some(tag.collection)
        }
    }
}

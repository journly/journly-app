use crate::{
    models::user::{Collaborator, User},
    views::{EncodableCollaborator, EncodableUser, EncodableUserPreview},
};

impl From<&Collaborator> for EncodableCollaborator {
    fn from(value: &Collaborator) -> Self {
        Self {
            id: value.id,
            username: value.username.clone(),
            avatar: value.avatar.clone(),
            permission: value.permission.clone(),
        }
    }
}

impl From<User> for EncodableUser {
    fn from(value: User) -> Self {
        EncodableUser {
            id: value.id,
            username: value.username,
            email: value.email,
            avatar: value.avatar,
        }
    }
}

impl From<User> for EncodableUserPreview {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            username: value.username,
            avatar: value.avatar,
        }
    }
}

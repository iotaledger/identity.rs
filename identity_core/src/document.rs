use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

use crate::utils::{Context, PublicKey, Service, Subject, VerificationMethod};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DIDDocument {
    context: Context,
    id: Subject,
    created: String,
    updated: String,
    public_keys: Vec<PublicKey>,
    authentications: Vec<VerificationMethod>,
    services: Vec<Service>,
    metadata: HashMap<String, String>,
}

impl DIDDocument {
    pub fn new(context: String, id: String) -> crate::Result<Self> {
        Ok(DIDDocument {
            context: Context::from_str(&context)?,
            id: Subject::from_str(&id)?,
            created: Utc::now().to_string(),
            updated: Utc::now().to_string(),
            ..Default::default()
        })
    }

    pub fn context(&self) -> &Vec<String> {
        &self.context.as_inner()
    }

    pub fn add_service(&mut self, service: Service) {
        self.services.push(service);

        self.update_time();
    }

    pub fn add_key_pair(&mut self, key_pair: PublicKey) {
        self.public_keys.push(key_pair);

        self.update_time();
    }

    pub fn update_time(&mut self) {
        self.updated = Utc::now().to_string();
    }
}

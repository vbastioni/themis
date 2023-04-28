use serde::{Deserialize, Serialize};

use super::text_acco::{TextAcco, self};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Acco {
    // Meta Commun
    pub id: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub ancien_id: Option<String>,
    #[serde(skip_deserializing, skip_serializing)]
    pub origine: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub url: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub nature: String,

    // Meta Acco
    pub titre_txt: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub document_bureautique: String,
    pub numero: String,
    pub siret: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub code_unite_signataire: String,
    pub date_maj: Datetime,
    pub date_depot: Datetime,
    pub date_texte: Datetime,
    pub date_effet: Datetime,
    pub date_fin: Datetime,
    pub date_diffusion: Datetime,
    pub code_ape: String,
    pub code_idcc: String,
    pub raison_sociale: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub conforme_version_integrale: String,
    pub secteur: String,
    pub themes: Option<Vec<Theme>>,
}

impl From<&TextAcco> for Acco {
    fn from(value: &TextAcco) -> Self {
        Acco {
            id: value.meta.meta_commun.id.to_owned(),
            ancien_id: value
                .meta
                .meta_commun
                .ancien_id
                .to_owned()
                .filter(|s| s != ""),
            origine: value.meta.meta_commun.origine.to_owned(),
            url: value.meta.meta_commun.url.to_owned(),
            nature: value.meta.meta_commun.nature.to_owned(),
            titre_txt: value.meta.meta_spec.meta_acco.titre_txt.to_owned(),
            document_bureautique: value
                .meta
                .meta_spec
                .meta_acco
                .document_bureautique
                .to_owned(),
            numero: value.meta.meta_spec.meta_acco.numero.to_owned(),
            siret: value.meta.meta_spec.meta_acco.siret.to_owned(),
            code_unite_signataire: value
                .meta
                .meta_spec
                .meta_acco
                .code_unite_signataire
                .to_owned(),
            date_maj: (&value.meta.meta_spec.meta_acco.date_maj).into(),
            date_depot: (&value.meta.meta_spec.meta_acco.date_depot).into(),
            date_texte: (&value.meta.meta_spec.meta_acco.date_texte).into(),
            date_effet: (&value.meta.meta_spec.meta_acco.date_effet).into(),
            date_fin: (&value.meta.meta_spec.meta_acco.date_fin).into(),
            date_diffusion: (&value.meta.meta_spec.meta_acco.date_diffusion).into(),
            code_ape: value.meta.meta_spec.meta_acco.code_ape.to_owned(),
            code_idcc: value.meta.meta_spec.meta_acco.code_idcc.to_owned(),
            raison_sociale: value.meta.meta_spec.meta_acco.raison_sociale.to_owned(),
            conforme_version_integrale: value
                .meta
                .meta_spec
                .meta_acco
                .conforme_version_integrale
                .to_owned(),
            secteur: value.meta.meta_spec.meta_acco.secteur.to_owned(),
            themes: value
                .meta
                .meta_spec
                .meta_acco
                .themes
                .theme
                .as_ref()
                .map(|v| v.iter().map(|t| t.into()).collect::<Vec<_>>()),
        }
    }
}

const LONG_FMT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Datetime(i64);

impl From<&String> for Datetime {
    fn from(value: &String) -> Self {
        use chrono::prelude::*;
        let ns = format!("{value} 00:00:00");
        Datetime(Utc.datetime_from_str(&ns, LONG_FMT).unwrap().timestamp())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Theme {
    pub code: String,
    pub libelle: String,
    pub groupe: String,
}

impl From<&text_acco::Theme> for Theme {
    fn from(value: &text_acco::Theme) -> Self {
        Theme {
            code: value.code.to_owned(),
            libelle: value.libelle.to_owned(),
            groupe: value.groupe.to_owned(),
        }
    }
}
